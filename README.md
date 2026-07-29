<div align="center">

# Kiro (`kiro`)

**Kiro** is a local rebrand of [SpaceXAI Grok Build](https://x.ai/cli): the same
terminal AI coding agent, compiled so the CLI binary and display name are
`kiro` instead of `grok`.

Full-screen TUI · understands your codebase · edits files · runs shell commands ·
web search · long-running tasks · headless / CI · Agent Client Protocol (ACP)

[Install](#install) ·
[Build from source](#build-from-source) ·
[CI artifacts](#ci-artifacts) ·
[Docs](#documentation) ·
[Layout](#repository-layout) ·
[Upstream](#upstream--maintenance) ·
[License](#license)

![Grok Build TUI](https://media.x.ai/v1/website/universe-tui-screenshot-6f7a0837.png)

</div>

---

## What is this?

| | Official Grok Build | This repo (**Kiro**) |
|--|---------------------|----------------------|
| CLI command | `grok` | **`kiro`** |
| Binary name | `grok` / `xai-grok-pager` | **`kiro`** |
| Auth & API | Grok / xAI | **Same** (official endpoints) |
| Config dir | `~/.grok` | **Same** (`~/.grok`, `$GROK_HOME`) |
| Source | [xai-org/grok-build](https://github.com/xai-org/grok-build) | Fork: [hufans/kiro-build](https://github.com/hufans/kiro-build) |

More detail for maintainers: **[KIRO.md](./KIRO.md)**.

---

## Install

### Option A — Build from source (recommended for now)

See [Build from source](#build-from-source) below, then:

```sh
install -m 755 target/release/kiro ~/.local/bin/kiro
kiro --version   # e.g. kiro 0.2.112 (...)
```

### Option B — Download CI artifacts

On every push/merge to `main`, GitHub Actions builds release binaries:

→ [Actions · Build kiro](https://github.com/hufans/kiro-build/actions)

| Artifact | Platform |
|----------|----------|
| `kiro-darwin-arm64` | Apple Silicon (M1/M2/M3…) |
| `kiro-darwin-x86_64` | Intel Mac |
| `kiro-linux-x86_64` | Linux x86_64 |

```sh
chmod +x kiro-darwin-arm64
xattr -dr com.apple.quarantine kiro-darwin-arm64 2>/dev/null || true
mv kiro-darwin-arm64 ~/.local/bin/kiro
kiro --version
```

### Official `grok` (upstream installer)

If you want the **official** product name and CDN installer instead of this fork:

```sh
curl -fsSL https://x.ai/cli/install.sh | bash   # macOS / Linux
irm https://x.ai/cli/install.ps1 | iex          # Windows PowerShell
grok --version
```

That installs `grok`, not `kiro`. Config/auth still live under `~/.grok` and are
compatible if you switch between the two.

---

## Build from source

**Requirements**

- **Rust** — pinned by [`rust-toolchain.toml`](rust-toolchain.toml) (rustup installs it on first build)
- **[DotSlash](https://dotslash-cli.com)** — for hermetic tools under [`bin/`](bin/) (e.g. [`bin/protoc`](bin/protoc)):

  ```sh
  cargo install dotslash
  /usr/bin/env dotslash --help
  ```

- **protoc** — via `bin/protoc` (DotSlash) or `$PROTOC` / `protoc` on `PATH`  
  - **Intel Mac (x86_64):** repo DotSlash may not ship `macos-x86_64`; install protoc yourself (e.g. protobuf **29.3**) and set `PROTOC`.

```sh
cargo run -p xai-grok-pager-bin -- --version     # build + run
cargo build -p xai-grok-pager-bin --release      # → target/release/kiro
cargo check -p xai-grok-pager-bin
```

On first launch, browser login uses the official Grok auth flow — see the
[authentication guide](crates/codegen/xai-grok-pager/docs/user-guide/02-authentication.md).

---

## CI artifacts

Workflow: [`.github/workflows/build-kiro.yml`](.github/workflows/build-kiro.yml)

| Trigger | When |
|---------|------|
| `push` to `main` | After merge / direct push |
| `pull_request` to `main` | PR validation |
| `workflow_dispatch` | Manual run |

Artifacts are kept for **30 days** on the Actions run page.

---

## Documentation

- Maintainer notes for this fork: **[KIRO.md](./KIRO.md)**
- Official product docs: [docs.x.ai/build/overview](https://docs.x.ai/build/overview)
- In-tree user guide (upstream docs, still say “grok” in places):  
  [`crates/codegen/xai-grok-pager/docs/user-guide/`](crates/codegen/xai-grok-pager/docs/user-guide/)

---

## Repository layout

| Path | Contents |
|------|----------|
| `crates/codegen/xai-grok-pager-bin` | Composition root; builds the **`kiro`** binary |
| `crates/codegen/xai-grok-pager` | TUI: scrollback, prompt, modals, rendering |
| `crates/codegen/xai-grok-shell` | Agent runtime + leader / stdio / headless |
| `crates/codegen/xai-grok-tools` | Tools (terminal, file edit, search, …) |
| `crates/codegen/xai-grok-workspace` | Host FS, VCS, execution, checkpoints |
| `crates/codegen/...` | Config, MCP, markdown, sandbox, … |
| `crates/common/`, `crates/build/`, `prod/mc/` | Shared leaf crates |
| `third_party/` | Vendored Mermaid-related sources |
| `KIRO.md` | Fork-specific maintenance guide |

> [!IMPORTANT]
> The root `Cargo.toml` (workspace members, dependency versions, lints,
> profiles) is **generated** — treat it as read-only. Prefer editing per-crate
> `Cargo.toml` files.

---

## Development

```sh
cargo check -p <crate>        # prefer per-crate; full workspace is slow
cargo test -p xai-grok-config
cargo clippy -p <crate>
cargo fmt --all
```

### Git remotes (this fork)

| remote | URL |
|--------|-----|
| `origin` | `git@github.com:hufans/kiro-build.git` |
| `upstream` | `https://github.com/xai-org/grok-build` |

```sh
git fetch upstream
git merge upstream/main   # or rebase; resolve kiro naming conflicts carefully
git push origin main
```

---

## Upstream & maintenance

- **Upstream:** [xai-org/grok-build](https://github.com/xai-org/grok-build)  
- **This fork:** [hufans/kiro-build](https://github.com/hufans/kiro-build)  
- `SOURCE_REV` records the monorepo commit SHA for the synced tree.

Local changes are intentionally small (binary/CLI name `kiro`) so merges from
upstream stay manageable. See [KIRO.md](./KIRO.md).

---

## Contributing

> [!NOTE]
> This is a personal fork. Prefer issues/PRs on this repo only for **Kiro-specific**
> packaging and branding. Upstream product contributions are not accepted here;
> see upstream [`CONTRIBUTING.md`](CONTRIBUTING.md).

---

## License

First-party code is under the **Apache License, Version 2.0** — see [`LICENSE`](LICENSE).

Third-party and vendored code remains under its original licenses:

- [`THIRD-PARTY-NOTICES`](THIRD-PARTY-NOTICES)
- [`crates/codegen/xai-grok-tools/THIRD_PARTY_NOTICES.md`](crates/codegen/xai-grok-tools/THIRD_PARTY_NOTICES.md)
- [`third_party/NOTICE`](third_party/NOTICE)
