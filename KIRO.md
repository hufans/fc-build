# Kiro 维护说明

本文记录本 fork 相对官方 **Grok Build** 的定位、改动范围、构建安装与后续同步方式，方便长期维护。

---

## 1. 项目概览

| 项 | 说明 |
|----|------|
| 上游项目 | [xai-org/grok-build](https://github.com/xai-org/grok-build)（SpaceXAI Grok Build CLI/TUI） |
| 本仓库 | [hufans/kiro-build](https://github.com/hufans/kiro-build) |
| 本地 CLI 名 | **`kiro`**（可执行文件、`--help`、`--version` 展示） |
| 官方 CLI 名 | `grok`（官方安装脚本 / npm 仍使用此名） |
| 认证与后端 | **与官方一致**（浏览器 OAuth、`cli-chat-proxy.grok.com` 等） |
| 配置目录 | **仍为 `~/.grok`**（或 `$GROK_HOME`），登录态与官方 `grok` 可共用 |
| 当前源码版本基线 | 上游 monorepo sync 提交 + 本 fork 的 kiro 品牌提交（见下方 Git） |

**一句话：** 在官方 Grok Build 源码上，把**本地启动命令/二进制/CLI 展示名**改成 `kiro`，功能与认证不变，便于本地使用与维护，并可继续从官方同步更新。

---

## 2. Git 与远程

### 2.1 Remotes

| remote | URL | 用途 |
|--------|-----|------|
| `origin` | `https://github.com/hufans/kiro-build.git` | 个人 fork，日常 push |
| `upstream` | `https://github.com/xai-org/grok-build` | 官方上游，拉取更新 |

### 2.2 关键提交（历史快照）

| 提交 | 说明 |
|------|------|
| `5da6962` | 官方 tree 同步基线（Synced from monorepo） |
| `0eefcc7` | 本 fork：`Rebrand local CLI binary and display name to kiro` |

`SOURCE_REV` 记录上游 monorepo 的完整 SHA（与官方同步树一致，**不**因 kiro 改名而改变）。

### 2.3 日常推送

```sh
git add -A
git commit -m "说明本次改动"
git push origin main
```

---

## 3. 改了什么 / 没改什么

### 3.1 已改动（本 fork 维护面）

| 类别 | 内容 |
|------|------|
| 二进制名 | `xai-grok-pager` → **`kiro`**（`xai-grok-pager-bin` 的 `[[bin]]` / `default-run`） |
| CLI 展示 | clap `name = "kiro"`，`about = "Kiro TUI"` |
| 版本输出 | `kiro 0.x.x (commit)` |
| Help / 示例 | `Usage: kiro ...`，部分示例命令改为 `kiro` |
| Completions | 按 `kiro` 生成 shell 补全 |
| 测试辅助 | 本地构建/查找二进制路径改为 `kiro` / `CARGO_BIN_EXE_kiro` |
| 进程识别 | leader 相关逻辑同时识别 **`kiro` 与 `grok`** 进程名 |

### 3.2 明确未改（与官方兼容）

| 类别 | 说明 |
|------|------|
| Crate / 包名 | 仍为 `xai-grok-pager-bin`、`xai-grok-pager` 等 |
| 配置与数据 | `~/.grok`、`$GROK_HOME`、`GROK_*` 环境变量 |
| 认证 | 官方 OAuth / 凭证路径不变 |
| API 端点 | 生产 `*.grok.com` 等（见 `xai-grok-env`） |
| 业务逻辑 | 工具、agent、TUI 行为与上游一致 |
| 官方 npm / install.sh | 未改；官方产物仍叫 `grok` |

### 3.3 改动文件清单（便于 merge 冲突时定位）

```
README.md
crates/codegen/xai-grok-pager-bin/Cargo.toml
crates/codegen/xai-grok-pager-bin/src/main.rs
crates/codegen/xai-grok-pager/src/app/cli.rs
crates/codegen/xai-grok-pager/src/completions_cmd.rs
crates/codegen/xai-grok-pager/tests/doctor_early_dispatch.rs
crates/codegen/xai-grok-pager-pty-harness/src/env.rs
crates/codegen/xai-grok-pager-pty-harness/src/bin/pty_scenario.rs
crates/codegen/xai-grok-pager-pty-harness/src/bin/scroll_matrix.rs
crates/codegen/xai-grok-test-support/src/env.rs
crates/codegen/xai-grok-shell-base/src/util/mod.rs
```

上游大更新后若冲突，**优先保留本表中的 kiro 命名**，再合并官方逻辑改动。

---

## 4. 仓库结构（与官方一致）

| 路径 | 作用 |
|------|------|
| `crates/codegen/xai-grok-pager-bin` | 组合根；编译出 **`kiro`** 可执行文件 |
| `crates/codegen/xai-grok-pager` | TUI（滚动、输入、模态框、渲染） |
| `crates/codegen/xai-grok-shell` | Agent 运行时、leader / headless / stdio |
| `crates/codegen/xai-grok-tools` | 工具实现（终端、文件、搜索等） |
| `crates/codegen/xai-grok-workspace` | 工作区 / VCS / 执行 / checkpoint |
| `crates/codegen/...` | 配置、MCP、markdown、sandbox 等 |
| `crates/common/`、`crates/build/`、`prod/mc/` | 共享叶子 crate |
| `third_party/` |  vendored 依赖（如 Mermaid 相关） |
| `bin/protoc` | DotSlash 封装的 protoc（**非** CLI 产品） |

根目录 `Cargo.toml` 为工作区生成/聚合文件，尽量只改各 crate 自己的 `Cargo.toml`。

---

## 5. 构建与安装

### 5.1 依赖

- **Rust**：`rust-toolchain.toml` 锁定版本（当前为 1.92.0 量级，以文件为准）
- **DotSlash**：`cargo install dotslash`（用于 `bin/protoc`）
- **protoc**
  - Apple Silicon / Linux：仓库内 `bin/protoc`（DotSlash）通常可用
  - **Intel Mac（x86_64）**：仓库 DotSlash **未提供** `macos-x86_64`，需自备 protoc，例如：
    - 下载 [protoc 29.3 osx-x86_64](https://github.com/protocolbuffers/protobuf/releases/download/v29.3/protoc-29.3-osx-x86_64.zip)
    - 解压到 `~/.local/protoc-29.3`，并设置：
      ```sh
      export PROTOC="$HOME/.local/protoc-29.3/bin/protoc"
      export PATH="$HOME/.local/protoc-29.3/bin:$PATH"
      ```

### 5.2 编译

```sh
source "$HOME/.cargo/env"
export PROTOC="${PROTOC:-$HOME/.local/protoc-29.3/bin/protoc}"   # Intel Mac 示例

cd /path/to/grok-build
cargo build -p xai-grok-pager-bin --release --bin kiro
# 产物：target/release/kiro
```

其它常用命令：

```sh
cargo run -p xai-grok-pager-bin -- --version
cargo check -p xai-grok-pager-bin
```

### 5.3 安装到本机 PATH

```sh
install -m 755 target/release/kiro ~/.local/bin/kiro
# 可选：与官方 grok 同目录
install -m 755 target/release/kiro ~/.grok/bin/kiro

kiro --version   # 期望：kiro x.y.z (commit)
kiro --help      # 期望：Kiro TUI / Usage: kiro ...
```

### 5.4 首次运行

- 与官方相同：可能打开浏览器完成登录
- 凭证与配置写在 **`~/.grok`**
- 若本机已安装并登录过官方 `grok`，一般可直接复用登录态

---

## 6. 从官方同步更新

```sh
cd /path/to/grok-build
git fetch upstream
git merge upstream/main
# 或：git rebase upstream/main

# 解决冲突时：保留 kiro 相关命名（见 §3.3）
git push origin main
```

再按 §5 重新 release 编译并 `install` 到 `~/.local/bin/kiro`。

**建议：** kiro 品牌 diff 尽量保持小而集中，降低与 monorepo sync 的冲突面。

---

## 7. 分发给其它电脑

| 目标环境 | 是否可用当前二进制 |
|----------|--------------------|
| 同架构 macOS（例如本机为 **x86_64 Intel**） | 可：拷贝 `kiro` 即可（仅依赖系统框架） |
| Apple Silicon | 建议在 ARM Mac 上重新编译，或依赖 Rosetta 跑 x86_64 |
| Linux / Windows | 需在对应平台编译 |

打包示例：

```sh
cp target/release/kiro ./kiro
zip kiro-macos-x86_64.zip kiro
```

接收方：

```sh
chmod +x kiro
# macOS 可能需：xattr -dr com.apple.quarantine ./kiro
mv kiro ~/.local/bin/
```

说明：

- 当前自编译产物 **通常未 codesign**，可能触发 Gatekeeper
- **网络仍访问官方 Grok/xAI 端点**（与 opencode 直连 xAI 类似，属服务使用而非“本地伪装”）
- 每人各自登录；不要随意分发含凭证的 `~/.grok`

---

## 8. 与监控 / 关键字扫描相关说明（事实记录）

| 检测方式 | 仅改名为 kiro 时 |
|----------|------------------|
| 进程名 / 路径含 `bin/grok` | 通常扫不到 `kiro` |
| CLI `--version` / `--help` 展示 | 已为 `kiro` / `Kiro TUI` |
| 二进制内部其它字符串、crate 名 | 仍可能含 `grok` |
| `~/.grok` 目录 | **仍存在** |
| 访问 `*.grok.com` 等网络 | **仍会发生**（设计如此） |

本 fork 的目标是 **本地命令与展示名使用 kiro**，不是隐藏官方服务访问。

---

## 9. 验证清单

安装或同步后建议快速检查：

```sh
which kiro
kiro --version          # kiro <version> (<sha>)
kiro --help | head      # Kiro TUI / Usage: kiro
kiro doctor             # 环境检查（可选）
```

---

## 10. 参考链接

- 本仓库：https://github.com/hufans/kiro-build  
- 官方仓库：https://github.com/xai-org/grok-build  
- 官方产品页：https://x.ai/cli  
- 官方文档：https://docs.x.ai/build/overview  
- 本仓库内用户指南：`crates/codegen/xai-grok-pager/docs/user-guide/`  
- 认证说明：`crates/codegen/xai-grok-pager/docs/user-guide/02-authentication.md`  

---

## 11. GitHub Actions 自动编译与一键安装

工作流文件：`.github/workflows/build-kiro.yml`  
安装脚本：`scripts/install.sh`

### 一键安装

```sh
curl -fsSL https://raw.githubusercontent.com/hufans/kiro-build/main/scripts/install.sh | bash
```

脚本从 Release 标签 **`continuous`** 下载对应平台二进制，默认装到 `~/.local/bin/kiro`。

### 触发条件

| 事件 | 说明 |
|------|------|
| **push → `main`** | 编译三端 + 发布/更新 `continuous` Release |
| **pull_request → `main`** | 只编译，不发 Release |
| **workflow_dispatch** | 手动运行 |

### 产物矩阵

| 文件名 | Runner | 适用机器 |
|--------|--------|----------|
| `kiro-linux-x86_64` | `ubuntu-latest` | Linux x86_64 |
| `kiro-darwin-arm64` | `macos-14` | Apple Silicon |
| `kiro-darwin-x86_64` | `macos-13` | Intel Mac |

- Actions Artifacts：保留 30 天  
- Release：https://github.com/hufans/kiro-build/releases/tag/continuous  

### 说明

- 三端都成功才会更新 `continuous`（缺任一平台则 release job 失败）  
- 单平台约 20–60+ 分钟；macOS 更费 Actions 额度  
- 若安装脚本 404：等 main 上最新 Build kiro 跑完并发布 Release  

---

## 12. 维护约定（建议）

1. **不要**大范围把 crate 名 / 全仓库 `grok` 字符串改成 `kiro`（跟 upstream 会极难合并）。  
2. 品牌相关改动尽量落在 §3.3 所列文件。  
3. `origin` 只推个人维护提交；同步时用 `upstream`。  
4. 本文件（`KIRO.md`）与 kiro 行为变更同步更新。  
5. CI 改动放在 `.github/workflows/`，与品牌 patch 分开提交亦可。  

---

*文档随本 fork 维护；若与代码不一致，以代码与最新 git 历史为准。*
