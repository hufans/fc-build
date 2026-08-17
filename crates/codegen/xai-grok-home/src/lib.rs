//! Single source of truth for the CLI home directory.
//!
//! For the **fc** fork: `$FC_HOME` / `$KIRO_HOME` / `$GROK_HOME`, else `<home>/.fc`.
//! Shared by `xai-grok-config` and `xai-fast-worktree`.
//!
//! Which function to call:
//! - [`grok_home`]: the usual choice, a cached, created path to build on.
//! - [`user_grok_home`]: `None` instead of a cwd fallback when no home resolves.
//! - [`default_grok_home`]: the `<home>/.fc` default, ignoring env overrides.
//! - [`resolve_grok_home`]: a fresh, uncached resolve.
//!
//! TODO: collapse these getters by threading the path through config as an
//! explicit value.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// Dir name under `$HOME` for this fork (short, non-product path fingerprint).
pub const DEFAULT_HOME_DIRNAME: &str = ".fc";

/// Previous fork home dir name (credential seed only).
pub const PRIOR_FORK_HOME_DIRNAME: &str = ".kiro";

/// Official CLI home dir name (credential seed only).
pub const LEGACY_HOME_DIRNAME: &str = ".grok";

/// `<home>/.fc`, canonicalized via `dunce` (not `std::fs::canonicalize`,
/// which yields Windows `\\?\` verbatim paths).
fn grok_home_in(home: &Path) -> PathBuf {
    dunce::canonicalize(home)
        .unwrap_or_else(|_| home.to_path_buf())
        .join(DEFAULT_HOME_DIRNAME)
}

/// Prefer `$FC_HOME`, then `$KIRO_HOME`, then `$GROK_HOME` when non-empty;
/// else `<home>/.fc`. Env values are used as-is (not canonicalized).
fn resolve_grok_home_from(
    fc_home: Option<&OsStr>,
    kiro_home: Option<&OsStr>,
    grok_home_env: Option<&OsStr>,
    os_home: Option<&Path>,
) -> Option<PathBuf> {
    for env in [fc_home, kiro_home, grok_home_env] {
        if let Some(v) = env.filter(|e| !e.is_empty()) {
            return Some(PathBuf::from(v));
        }
    }
    os_home.map(grok_home_in)
}

/// Resolve the home from the environment (fresh, no cache); `None` if neither resolves.
pub fn resolve_grok_home() -> Option<PathBuf> {
    resolve_grok_home_from(
        std::env::var_os("FC_HOME").as_deref(),
        std::env::var_os("KIRO_HOME").as_deref(),
        std::env::var_os("GROK_HOME").as_deref(),
        dirs::home_dir().as_deref(),
    )
}

/// The default `<home>/.fc`, used when no home override env is set.
pub fn default_grok_home() -> PathBuf {
    grok_home_in(&dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
}

/// The home, created if missing and cached for the process; falls back to
/// [`default_grok_home`] when no env override or OS home resolves.
pub fn grok_home() -> PathBuf {
    static GROK_HOME: OnceLock<PathBuf> = OnceLock::new();
    GROK_HOME
        .get_or_init(|| {
            let home = resolve_grok_home().unwrap_or_else(default_grok_home);
            if let Err(err) = std::fs::create_dir_all(&home) {
                tracing::warn!(path = %home.display(), %err, "failed to create fc home");
            }
            home
        })
        .clone()
}

/// Like [`grok_home`], but `None` when no home resolves (no cwd fallback).
pub fn user_grok_home() -> Option<PathBuf> {
    resolve_grok_home().is_some().then(grok_home)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::ffi::OsString;

    #[test]
    fn fc_env_wins_over_os_home() {
        let resolved = resolve_grok_home_from(
            Some(OsStr::new("/custom/fc")),
            Some(OsStr::new("/custom/kiro")),
            Some(OsStr::new("/custom/grok")),
            Some(Path::new("/home/u")),
        );
        assert_eq!(resolved, Some(PathBuf::from("/custom/fc")));
    }

    #[test]
    fn kiro_env_wins_when_fc_unset() {
        let resolved = resolve_grok_home_from(
            None,
            Some(OsStr::new("/custom/kiro")),
            Some(OsStr::new("/custom/grok")),
            Some(Path::new("/home/u")),
        );
        assert_eq!(resolved, Some(PathBuf::from("/custom/kiro")));
    }

    #[test]
    fn env_used_verbatim_even_when_it_exists() {
        let tmp = tempfile::tempdir().unwrap();
        let resolved = resolve_grok_home_from(Some(tmp.path().as_os_str()), None, None, None);
        assert_eq!(resolved, Some(tmp.path().to_path_buf()));
    }

    #[test]
    fn empty_env_falls_through_to_os_home() {
        let tmp = tempfile::tempdir().unwrap();
        let resolved = resolve_grok_home_from(
            Some(&OsString::new()),
            Some(&OsString::new()),
            Some(&OsString::new()),
            Some(tmp.path()),
        );
        assert_eq!(
            resolved,
            Some(
                dunce::canonicalize(tmp.path())
                    .unwrap()
                    .join(DEFAULT_HOME_DIRNAME)
            )
        );
    }

    #[test]
    fn neither_env_nor_os_home_returns_none() {
        assert_eq!(resolve_grok_home_from(None, None, None, None), None);
    }

    #[test]
    fn default_dirname_is_fc() {
        assert_eq!(DEFAULT_HOME_DIRNAME, ".fc");
    }
}
