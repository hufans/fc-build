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
| 配置目录 | **`~/.kiro`** / `$KIRO_HOME`（首次可从 `~/.grok` 迁移登录态） |

**一句话：** 官方能力 + 本地命令叫 `kiro`；配置目录与进程环境去 `grok` 指纹；API/登录协议不变；用 CI + `continuous` Release 做简易分发。

---

## 2. 改动履历（Changelog）

按时间从新到旧，覆盖本 fork 上的维护提交（不含上游 monorepo sync）。

| 提交 | 说明 |
|------|------|
| *(next)* | Actions **Sync upstream**：定时 merge 官方并开 PR / 自动合入 |
| *(next)* | `kiro update` 自更新：`kiro_installer` + `installer = "kiro"` |
| *(ops)* | 手动用已编好的 arm64/linux 创建 **`continuous`** Release，解除 install 404 |
| `2176d66` | 新增 `scripts/install.sh`；CI 在 main 构建成功后发布 **`continuous`** Release |
| `512f216` | README 改为 **Kiro** 标题与本 fork 安装/CI 说明 |
| `e4cbe04` | 文档中的仓库链接改为 `hufans/kiro-build`（GitHub 仓库已改名） |
| `63ff69d` | 首次加入 GitHub Actions `Build kiro`（三平台 release 编译 + Artifacts） |
| *(next)* | Sync upstream：无冲突直推 main + dispatch Build kiro（不再依赖开 PR） |
| `7f33568` | 去指纹：`~/.kiro` + 从 `~/.grok` 迁移登录；`KIRO_AGENT`；安装避开 `~/.grok/bin` |
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
| `crates/codegen/xai-grok-update/src/kiro_installer.rs` | **新增**：`kiro update` 从 continuous Release 自更新 |
| `crates/codegen/xai-grok-update/src/auto_update.rs` 等 | 注册 `installer = "kiro"`，避免走 x.ai CDN |

### 4.2 本 fork 独有（upstream 无对应，merge 时整文件保留）

| 路径 | 作用 |
|------|------|
| `KIRO.md` | **本维护手册** |
| `scripts/install.sh` | 一键安装 |
| `.github/workflows/build-kiro.yml` | 编译 + 发布 continuous |
| `README.md` | 已按 Kiro 重写（与 upstream README 会冲突，merge 后以本版为准或手工合并） |

### 4.3 进程/路径去指纹（公司扫描友好）

目标：**继续用官方 Grok 登录与 API**，但进程名、安装路径、配置目录、子进程环境里尽量不出现 `grok` 字样（Cursor/OpenCode 里用 Grok 模型通常仍允许；公司若扫的是「grok CLI」路径/进程指纹，本 fork 针对性弱化）。

| 项 | 行为 |
|----|------|
| 默认配置目录 | **`~/.kiro`**（`$KIRO_HOME` 优先，其次兼容 `$GROK_HOME`） |
| 登录迁移 | 首次启动若 `~/.kiro` 缺 `auth.json`/`config.toml` 等，从官方 **`~/.grok` 复制一次**（之后不再依赖 `~/.grok` 路径） |
| 二进制落点 | **`~/.local/bin/kiro`**（不要装到 `~/.grok/bin`） |
| 子进程哨兵 | **`KIRO_AGENT=1`**（不再导出 `GROK_AGENT=1`） |
| 认证 / API | **不变**（仍官方 OAuth / x.ai） |

**你本机必须做的清理（代码改完后）：**

1. 重新编译并安装到 `~/.local/bin/kiro`（见 §5）
2. 从 shell 配置里 **删掉** `PATH` 中的 `~/.grok/bin`
3. 确认 `which kiro` 不是 `~/.grok/bin/kiro`
4. 关掉旧会话后用新二进制启动；确认 `lsof -p $(pgrep -n kiro)` 打开的是 `~/.kiro/...` 而不是 `~/.grok/...`
5. 确认登录正常后，可归档/重命名 `~/.grok`（例如 `mv ~/.grok ~/.grok.bak`），避免家目录扫描命中

**仍无法完全抹掉的（诚实边界）：**

- 二进制内部仍有大量 `grok` 字符串（crate 名、文案、模型 id）；深扫 `strings` 仍可能认出
- 网络仍访问 x.ai / 官方鉴权（与 Cursor 用 Grok 模型同类）
- 仓库/工程里若路径含 `Grok`（如 `~/Code/Grok/...`）会出现在进程 cwd
- 项目级 `.grok/` 目录（skills/hooks）尚未全局改名

### 4.4 明确未改

- Crate 包名仍为 `xai-grok-*`
- 官方 OAuth 与 API 协议
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
# 不要装到 ~/.grok/bin（路径含 grok，易被扫描）
```

### 5.3 首次运行

- 浏览器登录 = 官方 Grok 流程（协议不变）
- 数据在 **`~/.kiro`**；若本机已有官方 `~/.grok` 登录态，会自动复制 `auth.json` / `config.toml` 等，一般无需重登

---

## 6. 一键安装与发布链路

### 6.1 用户侧

```sh
# 首次安装
curl -fsSL https://raw.githubusercontent.com/hufans/kiro-build/main/scripts/install.sh | bash

# 之后更新（推荐）
kiro update
kiro update --check
```

`kiro update` 由本 fork 的 **`kiro` installer** 实现（见 `crates/codegen/xai-grok-update/src/kiro_installer.rs`）：

- 进程名为 `kiro`，或 `~/.grok/config.toml` 中 `installer = "kiro"`
- 从 `continuous` Release 下载对应平台资产并替换 `current_exe` / `~/.local/bin/kiro` 等
- **不会**走官方 `https://x.ai/cli`（避免被覆盖成 `grok`）

| 环境变量 | 默认 | 含义 |
|----------|------|------|
| `KIRO_REPO` | `hufans/kiro-build` | install.sh 用的仓库 |
| `KIRO_TAG` / `KIRO_VERSION` | `continuous` | install.sh Release 标签 |
| `KIRO_BIN_DIR` | `~/.local/bin` | install.sh 安装目录 |
| `KIRO_RELEASE_REPO` | `hufans/kiro-build` | `kiro update` 用的仓库 |
| `KIRO_RELEASE_TAG` | `continuous` | `kiro update` 用的标签 |

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
  → matrix 编译（linux-x86_64 + darwin-arm64 Artifacts）
  → 全部成功
  → softprops/action-gh-release
  → tag: continuous（make_latest）
  → assets: kiro-linux-x86_64, kiro-darwin-arm64
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

- **已移除** `macos-13` / `kiro-darwin-x86_64`：免费 runner 常排队数小时，曾导致 `continuous` 永远不发版、install 404  
- 超时：120 分钟/job  
- 缓存：`Swatinem/rust-cache`（`shared-key: kiro-release`）  
- protoc：Linux apt；macOS 下载官方 zip 29.3  
- Artifacts 保留 30 天  

### 7.3 费用与优化

- macOS 分钟数贵；文档-only 改动也会触发全量编译（可后续加 `paths-ignore`）
- 紧急发版：`gh release upload continuous --clobber <binary>`（2026-07-30 曾用手搓 arm64/linux 解 404）

---

## 8. 从官方同步

### 8.1 自动同步（已配置，24h 一次）

工作流：[`.github/workflows/sync-upstream.yml`](./.github/workflows/sync-upstream.yml)  
Actions：https://github.com/hufans/kiro-build/actions/workflows/sync-upstream.yml  

| 触发 | 行为 |
|------|------|
| **每天 16:00 UTC**（约北京时间 00:00） | 检查官方是否有新提交 |
| **手动 Run workflow** | 立刻跑一遍；可关 auto-merge（不推 main） |

流程（无人值守）：

```text
cron / 手动
  → fetch xai-org/grok-build
  → 无新提交 → 结束
  → merge upstream/main → main
      ├─ 无冲突 → push main → **同一 run 内 workflow_call「Build kiro」** → continuous
      └─ 有冲突 → 开/更新 Issue（需你本地 resolve 后 push）
```

你不需要再开「额外的定时器」：GitHub Actions `schedule` 已经是 **每 24 小时** 一次。  
（GitHub cron 可能延迟数十分钟到数小时，属正常现象。）

#### 仓库设置检查清单

| 项 | 期望 |
|----|------|
| Actions 已启用 | Settings → Actions → Allow all actions |
| Workflow 权限 | 建议 **Read and write**（发 continuous 需要 contents:write） |
| main 分支保护 | 建议 **不要** 挡 bot push |
| Secret `SYNC_PAT` | **可选**（仅改变 push 身份；构建不再依赖它） |

#### 故障史

| 日期 | 现象 | 原因 | 修复 |
|------|------|------|------|
| 2026-07-30 | 开 PR 失败 | GITHUB_TOKEN 不能 createPullRequest | 改为直推 main |
| 2026-07-31 | 已 push main，job 仍红 | `gh workflow run "Build kiro"` → `could not find any workflows named Build kiro`；且 bot push **不会**连锁触发 Build | 改为 `workflow_call` 在同一 pipeline 编包发 continuous |

### 8.2 手动同步

```sh
cd /path/to/kiro-build
git fetch upstream
git checkout main && git pull origin main
git merge upstream/main
# 冲突：§4.1 保留 kiro；§4.2 独有文件勿删
git push origin main
```

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
| macOS arm64 | CI + install.sh ✅ |
| macOS x86_64 | **不发布**；本机 `cargo build` |
| Linux x86_64 | CI + install.sh ✅ |
| Linux arm64 | **未发布** |
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
# 应有 kiro-darwin-arm64 与 kiro-linux-x86_64
```

---

## 11. 故障排查

| 现象 | 可能原因 | 处理 |
|------|----------|------|
| install 404 / HTML | 尚无 `continuous` 或 CI 失败 | 看 [Releases](https://github.com/hufans/kiro-build/releases)；等 CI；或源码编译 |
| install 404（历史） | Intel job 排队卡住，release 永不跑 | 已去掉 macos-13；已手搓 continuous |
| Intel Mac install 拒绝 | 故意不发 x86_64 包 | 本机编译安装 |
| install 下载后无法执行 | 架构下错 / 坏文件 | 对照 `uname -m`；重装；看 `--version` |
| `kiro: command not found` | PATH 无 `~/.local/bin` | 按脚本提示写 zshrc/bashrc |
| Intel Mac 编不过 protoc | DotSlash 无 x86_64 | 自备 protoc + `PROTOC` |
| push workflow 被拒 | OAuth 无 workflow scope | 用 SSH origin |
| continuous 未更新 | matrix 失败 | 修失败 job；可 `gh release upload continuous --clobber` |
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
