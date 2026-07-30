//! Self-update backend for the `kiro` fork binary.
//!
//! Downloads rolling artifacts from GitHub Releases tag `continuous` on
//! `hufans/kiro-build` (override with `KIRO_RELEASE_REPO` / `KIRO_RELEASE_TAG`).

use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};
use tokio::process::Command;

/// Default GitHub repo that hosts `continuous` release assets.
pub const DEFAULT_REPO: &str = "hufans/kiro-build";
/// Default release tag (rolling).
pub const DEFAULT_TAG: &str = "continuous";

fn release_repo() -> String {
    std::env::var("KIRO_RELEASE_REPO").unwrap_or_else(|_| DEFAULT_REPO.to_string())
}

fn release_tag() -> String {
    std::env::var("KIRO_RELEASE_TAG").unwrap_or_else(|_| DEFAULT_TAG.to_string())
}

/// True when this process should use the kiro update channel.
pub fn running_as_kiro() -> bool {
    std::env::current_exe()
        .ok()
        .and_then(|p| {
            p.file_name()
                .map(|n| n.to_string_lossy().eq_ignore_ascii_case("kiro"))
        })
        .unwrap_or(false)
}

/// Map host platform to CI artifact name (`kiro-darwin-arm64`, etc.).
pub fn artifact_name() -> Result<&'static str> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    match (os, arch) {
        ("macos", "aarch64") => Ok("kiro-darwin-arm64"),
        ("macos", "x86_64") => bail!(
            "Intel Mac (x86_64) binaries are not published on the continuous channel.\n\
             Build from source:\n  \
             cargo build -p xai-grok-pager-bin --release --bin kiro"
        ),
        ("linux", "x86_64") => Ok("kiro-linux-x86_64"),
        ("linux", "aarch64") => bail!("linux arm64 binaries are not published yet"),
        _ => bail!("unsupported platform for kiro update: {os}-{arch}"),
    }
}

fn asset_url(artifact: &str) -> String {
    format!(
        "https://github.com/{}/releases/download/{}/{}",
        release_repo(),
        release_tag(),
        artifact
    )
}

/// Parse `kiro 0.2.112 (47c2f2f)` → `0.2.112+47c2f2f` (semver + build meta for equality).
pub fn parse_cli_version_line(line: &str) -> Result<String> {
    let line = line.trim();
    let rest = line
        .strip_prefix("kiro ")
        .or_else(|| line.strip_prefix("grok "))
        .unwrap_or(line)
        .trim();
    // Drop trailing channel labels like " [stable]"
    let rest = rest.split('[').next().unwrap_or(rest).trim();
    let mut parts = rest.split_whitespace();
    let ver = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("empty version line: {line:?}"))?;
    semver::Version::parse(ver)
        .with_context(|| format!("invalid semver in version line: {line:?}"))?;
    if let Some(paren) = parts.next() {
        let sha = paren.trim_matches(|c| c == '(' || c == ')');
        if !sha.is_empty() && sha.chars().all(|c| c.is_ascii_hexdigit()) {
            return Ok(format!("{ver}+{sha}"));
        }
    }
    Ok(ver.to_string())
}

async fn version_from_binary(path: &Path) -> Result<String> {
    let mut cmd = Command::new(path);
    cmd.arg("--version")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null());
    xai_grok_tools::util::detach_command(&mut cmd);
    let out = cmd
        .output()
        .await
        .with_context(|| format!("failed to run {} --version", path.display()))?;
    if !out.status.success() {
        bail!(
            "{} --version failed (exit {:?})",
            path.display(),
            out.status.code()
        );
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let line = stdout
        .lines()
        .next()
        .ok_or_else(|| anyhow::anyhow!("no version output from {}", path.display()))?;
    parse_cli_version_line(line)
}

/// Running binary's version (for compare against continuous).
pub async fn running_version() -> Result<String> {
    let exe = std::env::current_exe().context("current_exe")?;
    version_from_binary(&exe).await
}

async fn download_to(path: &Path, url: &str) -> Result<()> {
    // Prefer the shared downloader (progress + range) from auto_update.
    crate::auto_update::download_with_progress(url, path)
        .await
        .with_context(|| format!("download failed: {url}"))
}

/// Latest continuous release version as `X.Y.Z+sha` (or plain semver).
pub async fn fetch_latest_version() -> Result<String> {
    let artifact = artifact_name()?;
    let url = asset_url(artifact);
    let tmp = std::env::temp_dir().join(format!(
        "kiro-version-check-{}-{}",
        std::process::id(),
        artifact
    ));
    if let Err(e) = download_to(&tmp, &url).await {
        let _ = tokio::fs::remove_file(&tmp).await;
        return Err(e);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        tokio::fs::set_permissions(&tmp, std::fs::Permissions::from_mode(0o755)).await?;
    }
    let ver = version_from_binary(&tmp).await;
    let _ = tokio::fs::remove_file(&tmp).await;
    ver
}

/// Install destinations: current exe first, then common kiro install paths.
fn install_destinations() -> Vec<PathBuf> {
    let mut dests = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        dests.push(exe);
    }
    if let Some(home) = dirs_home() {
        // Prefer paths without a `grok` path segment (scanner fingerprint).
        // Still refresh a legacy `~/.grok/bin/kiro` if that is the running
        // binary, via `current_exe` above — but do not re-create that path.
        for p in [
            home.join(".local/bin/kiro"),
            home.join(".kiro/bin/kiro"),
        ] {
            if p.exists() && !dests.iter().any(|d| d == &p) {
                dests.push(p);
            }
        }
        // Always ensure ~/.local/bin/kiro is a target so users can leave
        // ~/.grok/bin without losing updates.
        let local = home.join(".local/bin/kiro");
        if !dests.iter().any(|d| d == &local) {
            dests.push(local);
        }
    }
    dests
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

/// Download continuous artifact and atomically replace kiro install paths.
pub async fn install(_pinned: Option<&str>) -> Result<()> {
    let artifact = artifact_name()?;
    let url = asset_url(artifact);
    let dests = install_destinations();
    if dests.is_empty() {
        bail!("could not resolve install path for kiro");
    }

    eprintln!(
        "  Downloading kiro ({artifact}) from GitHub Releases ({}/{})...",
        release_repo(),
        release_tag()
    );

    let tmp_dir = std::env::temp_dir().join(format!("kiro-update-{}", std::process::id()));
    tokio::fs::create_dir_all(&tmp_dir).await?;
    let tmp_bin = tmp_dir.join("kiro");
    download_to(&tmp_bin, &url).await?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        tokio::fs::set_permissions(&tmp_bin, std::fs::Permissions::from_mode(0o755)).await?;
    }

    // Smoke-test before replacing anything.
    let new_ver = version_from_binary(&tmp_bin).await?;
    eprintln!("  downloaded: kiro {new_ver}");

    for dest in &dests {
        if let Some(parent) = dest.parent() {
            tokio::fs::create_dir_all(parent).await.ok();
        }
        let staging = dest.with_extension(format!("new.{}", std::process::id()));
        tokio::fs::copy(&tmp_bin, &staging)
            .await
            .with_context(|| format!("copy to staging {}", staging.display()))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            tokio::fs::set_permissions(&staging, std::fs::Permissions::from_mode(0o755)).await?;
        }
        // Replace in place (Unix can rename over a running binary).
        tokio::fs::rename(&staging, dest)
            .await
            .with_context(|| format!("install to {}", dest.display()))?;
        eprintln!("  installed: {}", dest.display());
    }

    let _ = tokio::fs::remove_dir_all(&tmp_dir).await;

    // Remember installer type for future `kiro update` / auto-update.
    let _ = xai_grok_shell::util::config::update_config(|st| {
        st.cli.installer = Some("kiro".to_string());
    })
    .await;

    Ok(())
}
