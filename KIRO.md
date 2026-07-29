# Kiro 维护手册

本文件记录 **[hufans/kiro-build](https://github.com/hufans/kiro-build)** 相对官方 Grok Build 的全部 fork 改动、发布链路与日常运维，方便后期维护。

面向读者：仓库维护者。  
用户向安装说明见 [README.md](./README.md)。

---

## 目录

1. [项目定位](#1-项目定位)
2. [改动履历（Changelog）](#2-改动履历changelog)
3. [仓库与 Git](#3-仓库与-git)
4. [代码改动面（与 upstream 冲突地图）](#4-代码改动面与-upstream-冲突地图)
5. [构建（本机）](#5-构建本机)
6. [一键安装与发布链路](#6-一键安装与发布链路)
7. [CI 工作流说明](#7-ci-工作流说明)
8. [从官方同步](#8-从官方同步)
9. [分发与平台](#9-分发与平台)
10. [验证清单](#10-验证清单)
11. [故障排查](#11-故障排查)
12. [维护约定](#12-维护约定)
13. [参考链接](#13-参考链接)

---

## 1. 项目定位

| 项 | 说明 |
|----|------|
| 上游 | [xai-org/grok-build](https://github.com/xai-org/grok-build)（SpaceXAI Grok Build） |
| 本仓库 | [hufans/kiro-build](https://github.com/hufans/kiro-build)（原 fork 名 `grok-build`，已改名） |
| 本地 CLI | **`kiro`**（二进制名、`Usage`、`--version` 前缀） |
| 官方 CLI | `grok` |
| 认证 / API | **不变**，仍走官方 Grok / xAI |
| 配置目录 | **`~/.grok`** / `$GROK_HOME`（可与官方 `grok` 共用登录态） |

**一句话：** 官方能力 + 本地命令叫 `kiro`；不藏网络、不改后端；用 CI + `continuous` Release 做简易分发。

---

## 2. 改动履历（Changelog）

按时间从新到旧，覆盖本 fork 上的维护提交（不含上游 monorepo sync）。

| 提交 | 说明 |
|------|------|
| `2176d66` | 新增 `scripts/install.sh`；CI 在 main 构建成功后发布 **`continuous`** Release |
| `512f216` | README 改为 **Kiro** 标题与本 fork 安装/CI 说明 |
| `e4cbe04` | 文档中的仓库链接改为 `hufans/kiro-build`（GitHub 仓库已改名） |
| `63ff69d` | 首次加入 GitHub Actions `Build kiro`（三平台 release 编译 + Artifacts） |
| `0eefcc7` | 核心 rebrand：二进制 / CLI 展示名为 `kiro`；测试路径与进程识别兼容 kiro |

**仓库改名（GitHub 侧，无独立 commit）：**

- `hufans/grok-build` → **`hufans/kiro-build`**
- 本地 `origin` 使用 SSH：`git@github.com:hufans/kiro-build.git`  
  （HTTPS OAuth 若无 `workflow` scope，无法推送 `.github/workflows/*`）

**基线：**

| 提交 | 说明 |
|------|------|
| `5da6962` | 改动前的上游同步点（Synced from monorepo） |
| `SOURCE_REV` | 上游 monorepo 完整 SHA（与 tree 对应，不因 kiro 改名变化） |

更新本表：每次合并有意义的 fork 改动后，在表顶追加一行。

---

## 3. 仓库与 Git

### 3.1 Remotes

| remote | URL | 用途 |
|--------|-----|------|
| `origin` | `git@github.com:hufans/kiro-build.git` | 个人 fork，日常 push |
| `upstream` | `https://github.com/xai-org/grok-build` | 官方上游 |

```sh
git remote -v
# origin    git@github.com:hufans/kiro-build.git
# upstream  https://github.com/xai-org/grok-build
```

### 3.2 日常推送

```sh
git add -A
git commit -m "说明"
git push origin main
# 会触发 Build kiro；成功后刷新 continuous Release
```

### 3.3 推送 Workflow 注意

若 `git push` 报错：

```text
refusing to allow an OAuth App to create or update workflow ... without workflow scope
```

改用 **SSH remote**（当前已配置），或 `gh auth refresh -s workflow`。

---

## 4. 代码改动面（与 upstream 冲突地图）

### 4.1 功能 / 品牌（会与 upstream 冲突）

| 文件 | 改了什么 |
|------|----------|
| `crates/codegen/xai-grok-pager-bin/Cargo.toml` | `[[bin]] name = "kiro"`，`default-run = "kiro"` |
| `crates/codegen/xai-grok-pager-bin/src/main.rs` | `version_text` → `kiro …`；相关测试 |
| `crates/codegen/xai-grok-pager/src/app/cli.rs` | clap `name`/`about`；示例文案；`parse_cli` 认 kiro/grok/agent |
| `crates/codegen/xai-grok-pager/src/completions_cmd.rs` | 补全按 `kiro` 生成 |
| `crates/codegen/xai-grok-pager/tests/doctor_early_dispatch.rs` | `CARGO_BIN_EXE_kiro` |
| `crates/codegen/xai-grok-test-support/src/env.rs` | 本地二进制路径 / `cargo build --bin kiro` |
| `crates/codegen/xai-grok-pager-pty-harness/src/env.rs` | 同上 |
| `crates/codegen/xai-grok-pager-pty-harness/src/bin/*.rs` | 注释中的 bin 名 |
| `crates/codegen/xai-grok-shell-base/src/util/mod.rs` | `is_grok_process*` 同时认 **kiro** 与 **grok** |

### 4.2 本 fork 独有（upstream 无对应，merge 时整文件保留）

| 路径 | 作用 |
|------|------|
| `KIRO.md` | **本维护手册** |
| `scripts/install.sh` | 一键安装 |
| `.github/workflows/build-kiro.yml` | 编译 + 发布 continuous |
| `README.md` | 已按 Kiro 重写（与 upstream README 会冲突，merge 后以本版为准或手工合并） |

### 4.3 明确未改

- Crate 包名仍为 `xai-grok-*`
- `~/.grok`、`GROK_*`、官方 OAuth 与 API
- 业务逻辑、工具实现
- 官方 `https://x.ai/cli/install.sh`（装的是 `grok`，不是 kiro）

**merge 原则：** 品牌相关冲突 → 保留 kiro 命名；逻辑冲突 → 取 upstream 逻辑后再套 kiro 名。

---

## 5. 构建（本机）

### 5.1 依赖

- Rust：`rust-toolchain.toml`（当前 channel **1.92.0**）
- DotSlash：`cargo install dotslash`（`bin/protoc`）
- protoc：
  - Apple Silicon / Linux：一般可用仓库 `bin/protoc`
  - **Intel Mac x86_64**：DotSlash **无** `macos-x86_64`，需自备，例如 protoc **29.3**：
    ```sh
    export PROTOC="$HOME/.local/protoc-29.3/bin/protoc"
    export PATH="$HOME/.local/protoc-29.3/bin:$PATH"
    ```

### 5.2 命令

```sh
source "$HOME/.cargo/env"
export PROTOC="${PROTOC:-$HOME/.local/protoc-29.3/bin/protoc}"

cargo build -p xai-grok-pager-bin --release --bin kiro
# → target/release/kiro

install -m 755 target/release/kiro ~/.local/bin/kiro
# 可选：install -m 755 target/release/kiro ~/.grok/bin/kiro
```

### 5.3 首次运行

- 浏览器登录 = 官方 Grok 流程
- 数据在 `~/.grok`；已登录过官方 `grok` 通常可直接用

---

## 6. 一键安装与发布链路

### 6.1 用户侧

```sh
curl -fsSL https://raw.githubusercontent.com/hufans/kiro-build/main/scripts/install.sh | bash
```

| 环境变量 | 默认 | 含义 |
|----------|------|------|
| `KIRO_REPO` | `hufans/kiro-build` | GitHub 仓库 |
| `KIRO_TAG` / `KIRO_VERSION` | `continuous` | Release 标签 |
| `KIRO_BIN_DIR` | `~/.local/bin` | 安装目录 |

脚本逻辑摘要：

1. `uname` → `darwin-arm64` / `darwin-x86_64` / `linux-x86_64`
2. 下载  
   `https://github.com/<repo>/releases/download/<tag>/kiro-<platform>`
3. `chmod +x`、试跑 `--version`、写入 `$KIRO_BIN_DIR/kiro`
4. PATH 不包含安装目录时打印配置提示

源码：[`scripts/install.sh`](./scripts/install.sh)

### 6.2 发布侧（自动化）

```text
push main
  → matrix 编译三端（Artifacts）
  → 全部成功
  → softprops/action-gh-release
  → tag: continuous（make_latest）
  → assets: kiro-linux-x86_64, kiro-darwin-arm64, kiro-darwin-x86_64
```

Release 页：https://github.com/hufans/kiro-build/releases/tag/continuous  

**注意：** 任一平台构建失败 → **不会**更新 `continuous`；旧 Release 资产可能仍可装，但不是最新 commit。

### 6.3 与官方 `curl | bash` 的对比

| | 官方 Grok | 本 fork Kiro |
|--|-----------|--------------|
| 脚本 | `https://x.ai/cli/install.sh` | `scripts/install.sh`（raw.githubusercontent） |
| 二进制托管 | x.ai CDN + GCS | GitHub Release `continuous` |
| 命名 | `grok-{ver}-{platform}` | `kiro-darwin-arm64` 等 |
| 频道 | stable/alpha 指针文件 | 固定 tag `continuous`（滚动覆盖） |

---

## 7. CI 工作流说明

文件：[`.github/workflows/build-kiro.yml`](./.github/workflows/build-kiro.yml)  
Actions：https://github.com/hufans/kiro-build/actions  

### 7.1 触发

| 事件 | 行为 |
|------|------|
| `push` → `main` | 编译 + **发布 continuous** |
| `pull_request` → `main` | 仅编译 |
| `workflow_dispatch` | 手动（是否发 release 取决于 ref 是否为 main push 条件） |

### 7.2 Matrix

| Artifact / Release 资产 | Runner | 目标用户 |
|-------------------------|--------|----------|
| `kiro-linux-x86_64` | `ubuntu-latest` | Linux x86_64 |
| `kiro-darwin-arm64` | `macos-14` | **Apple Silicon** |
| `kiro-darwin-x86_64` | `macos-13` | Intel Mac |

- 超时：120 分钟/job  
- 缓存：`Swatinem/rust-cache`（`shared-key: kiro-release`）  
- protoc：Linux apt；macOS 下载官方 zip 29.3  
- Artifacts 保留 30 天  

### 7.3 费用与优化

- macOS 分钟数贵；额度紧时可删掉 matrix 中某一行（同时改 `install.sh` 与 release 必选文件列表）
- 文档-only 改动也会触发全量编译（可后续加 `paths-ignore`）

---

## 8. 从官方同步

```sh
cd /path/to/kiro-build   # 本地克隆路径可能仍叫 grok-build，无妨
git fetch upstream
git merge upstream/main
# 或：git rebase upstream/main

# 冲突时：
#   - §4.1 文件 → 保留 kiro 命名
#   - README.md → 保留本 fork 版或手工合并
#   - 独有文件 §4.2 → 勿删

git push origin main
# 等 CI 绿了，continuous 自动更新
```

可选：同步后本地再编一次自测：

```sh
cargo build -p xai-grok-pager-bin --release --bin kiro
./target/release/kiro --version
```

---

## 9. 分发与平台

| 方式 | 适用 |
|------|------|
| `install.sh` | 有 `continuous` Release 后的 macOS / Linux |
| Release 网页下载资产 | 同上，手动选文件 |
| Actions Artifacts | 调试单次 run；需进 Actions 页 |
| 本机 `cargo build` + 拷贝 | 无 Release 时 / 改完立刻自用 |
| 官方 install.sh | 需要官方 `grok` 品牌时 |

| 平台 | 支持情况 |
|------|----------|
| macOS arm64 | CI + install.sh |
| macOS x86_64 | CI + install.sh |
| Linux x86_64 | CI + install.sh |
| Linux arm64 | **未发布**（install 会报错） |
| Windows | **未做** fork CI；可用官方 `install.ps1` 装 grok |

未签名：macOS 可能提示无法验证开发者；`xattr -dr com.apple.quarantine` 或系统设置允许。

---

## 10. 验证清单

```sh
# 一键装
curl -fsSL https://raw.githubusercontent.com/hufans/kiro-build/main/scripts/install.sh | bash
which kiro
kiro --version          # 期望：kiro <ver> (<sha>)
kiro --help | head -5   # 期望：Kiro TUI / Usage: kiro

# 本机构建
cargo build -p xai-grok-pager-bin --release --bin kiro
./target/release/kiro --version

# Release 是否齐全
# 打开 https://github.com/hufans/kiro-build/releases/tag/continuous
# 应有三个 kiro-* 文件
```

---

## 11. 故障排查

| 现象 | 可能原因 | 处理 |
|------|----------|------|
| install 404 / HTML | 尚无 `continuous` 或 CI 失败 | 看 Actions；等三端成功；或源码编译 |
| install 下载后无法执行 | 架构下错 / 坏文件 | 对照 `uname -m`；重装；看 `--version` |
| `kiro: command not found` | PATH 无 `~/.local/bin` | 按脚本提示写 zshrc/bashrc |
| Intel Mac 编不过 protoc | DotSlash 无 x86_64 | 自备 protoc + `PROTOC` |
| push workflow 被拒 | OAuth 无 workflow scope | 用 SSH origin |
| continuous 未更新 | 某一 matrix job 失败 | 修失败 job；`fail-fast: false` 时其它端可能仍绿但 release 不发 |
| merge upstream 大量冲突 | 改动面扩大 | 控制在 §4.1；不要全库 rename |

---

## 12. 维护约定

1. **品牌 diff 保持小**：只动 §4.1 列表；禁止全仓库 `grok`→`kiro`。  
2. **本手册与行为同步**：改 install/CI/命名时更新 §2 履历 + 对应章节。  
3. **origin / upstream 职责清晰**：只向 origin 推 fork 维护；上游只 fetch/merge。  
4. **发版以 CI 为准**：不要手搓 continuous 除非紧急；紧急时可 `gh release upload continuous --clobber`。  
5. **README 面向用户，KIRO.md 面向维护者**。  

---

## 13. 参考链接

| 资源 | URL |
|------|-----|
| 本仓库 | https://github.com/hufans/kiro-build |
| Actions | https://github.com/hufans/kiro-build/actions |
| continuous Release | https://github.com/hufans/kiro-build/releases/tag/continuous |
| install.sh（raw） | https://raw.githubusercontent.com/hufans/kiro-build/main/scripts/install.sh |
| 上游 | https://github.com/xai-org/grok-build |
| 官方安装 | `curl -fsSL https://x.ai/cli/install.sh \| bash` |
| 官方文档 | https://docs.x.ai/build/overview |
| 认证（树内） | `crates/codegen/xai-grok-pager/docs/user-guide/02-authentication.md` |

---

*最后整理：覆盖至提交 `2176d66`（install.sh + continuous release）。*  
*若与代码不一致，以仓库当前文件与 `git log` 为准，并回写本文件。*
