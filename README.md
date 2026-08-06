<div align="center">

# fc

**fc** is a terminal AI coding agent: full-screen TUI, codebase-aware edits,
shell tools, web search, long-running tasks, headless / CI, and Agent Client
Protocol (ACP).

[Install](#install) ·
[Build from source](#build-from-source) ·
[CI artifacts](#ci-artifacts) ·
[Docs](#documentation) ·
[Layout](#repository-layout) ·
[License](#license)

</div>

---

## Quick facts

| | |
|--|--|
| CLI / binary | **`fc`** |
| Config directory | **`~/.fc`** (`$FC_HOME`) |
| Install | `~/.local/bin/fc` (see below) |
| Source | [hufans/fc-build](https://github.com/hufans/fc-build) |

Maintainer notes: **[FC.md](./FC.md)**.

---

## Install

### One-line install (recommended)

After CI has published a **continuous** release (runs on every merge to `main`):

```sh
curl -fsSL https://raw.githubusercontent.com/hufans/fc-build/main/scripts/install.sh | bash
fc --version
```

What it does:

1. Detects OS/arch (Apple Silicon / Intel Mac / Linux x86_64)
2. Downloads the matching binary from  
   `https://github.com/hufans/fc-build/releases/download/continuous/...`
3. Installs to `~/.local/bin/fc` (override with `FC_BIN_DIR`)

Optional:

```sh
# custom install dir
FC_BIN_DIR=$HOME/bin curl -fsSL https://raw.githubusercontent.com/hufans/fc-build/main/scripts/install.sh | bash
```

On zsh/bash, `fc` may collide with the shell builtin / `/usr/bin/fc`. Prefer:

```sh
alias fc='$HOME/.local/bin/fc'   # in ~/.zshrc or ~/.bashrc
```

### Update in place

```sh
fc update          # download latest continuous release + replace binary
fc update --check  # only check
```

Uses GitHub Release tag `continuous`. Restart the terminal after a successful
update. If download fails, wait for the
[Build fc](https://github.com/hufans/fc-build/actions) workflow, or build from
source below.

### Build from source

```sh
install -m 755 target/release/fc ~/.local/bin/fc
fc --version
```

(See [Build from source](#build-from-source) for full requirements.)

### CI artifacts (manual)

→ [Actions · Build fc](https://github.com/hufans/fc-build/actions)  
→ [Releases · continuous](https://github.com/hufans/fc-build/releases/tag/continuous)

| Asset | Platform |
|-------|----------|
| `fc-darwin-arm64` | Apple Silicon |
| `fc-linux-x86_64` | Linux x86_64 |

Intel Mac: build from source (CI does not publish x86_64 macOS assets by default).

---

## Build from source

**Requirements**

- **Rust** — pinned by [`rust-toolchain.toml`](rust-toolchain.toml)
- **[DotSlash](https://dotslash-cli.com)** — for hermetic tools under [`bin/`](bin/):

  ```sh
  cargo install dotslash
  ```

- **protoc** — via `bin/protoc` (DotSlash) or `$PROTOC` on `PATH`  
  - **Intel Mac (x86_64):** set `PROTOC` to a local protoc (e.g. 29.3) if DotSlash has no `macos-x86_64` stub.

```sh
cargo build -p xai-grok-pager-bin --release --bin fc   # → target/release/fc
cargo check -p xai-grok-pager-bin
./target/release/fc --version
```

On first launch, complete the in-app login flow. Config and credentials live under
**`~/.fc`**.

---

## CI & releases

Workflow: [`.github/workflows/build-fc.yml`](.github/workflows/build-fc.yml)

| Trigger | When |
|---------|------|
| `push` to `main` | Build + publish **`continuous`** release |
| `pull_request` to `main` | Build only |
| `workflow_dispatch` | Manual run |

Installer: [`scripts/install.sh`](scripts/install.sh)

---

## Documentation

- Maintainer guide: **[FC.md](./FC.md)**
- In-tree user guide (some pages still use upstream wording):  
  [`crates/codegen/xai-grok-pager/docs/user-guide/`](crates/codegen/xai-grok-pager/docs/user-guide/)

---

## Repository layout

| Path | Contents |
|------|----------|
| `crates/codegen/xai-grok-pager-bin` | Composition root; builds the **`fc`** binary |
| `crates/codegen/xai-grok-pager` | TUI: scrollback, prompt, modals, rendering |
| `crates/codegen/xai-grok-shell` | Agent runtime + leader / stdio / headless |
| `crates/codegen/xai-grok-tools` | Tools (terminal, file edit, search, …) |
| `crates/codegen/xai-grok-workspace` | Host FS, VCS, execution, checkpoints |
| `crates/codegen/...` | Config, MCP, markdown, sandbox, … |
| `crates/common/`, `crates/build/`, `prod/mc/` | Shared leaf crates |
| `third_party/` | Vendored Mermaid-related sources |
| `FC.md` | Fork-specific maintenance guide |

> [!IMPORTANT]
> The root `Cargo.toml` (workspace members, dependency versions, lints,
> profiles) is **generated** — treat it as read-only. Prefer editing per-crate
> `Cargo.toml` files.

---

## Development

```sh
cargo check -p <crate>
cargo test -p xai-grok-config
cargo clippy -p <crate>
cargo fmt --all
```

### Git remotes

| remote | URL |
|--------|-----|
| `origin` | `git@github.com:hufans/fc-build.git` |

```sh
git push origin main
```

Optional daily sync workflow: [Sync upstream](https://github.com/hufans/fc-build/actions/workflows/sync-upstream.yml) — see [FC.md](./FC.md).

---

## Contributing

> [!NOTE]
> This is a personal packaging fork. Prefer issues/PRs here for **fc-specific**
> install, branding, and release packaging.

---

## License

First-party code is under the **Apache License, Version 2.0** — see [`LICENSE`](LICENSE).

Third-party and vendored code remains under its original licenses:

- [`THIRD-PARTY-NOTICES`](THIRD-PARTY-NOTICES)
- [`crates/codegen/xai-grok-tools/THIRD_PARTY_NOTICES.md`](crates/codegen/xai-grok-tools/THIRD_PARTY_NOTICES.md)
- [`third_party/NOTICE`](third_party/NOTICE)
