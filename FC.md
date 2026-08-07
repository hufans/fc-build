# fc 维护手册

本文件记录 **[hufans/fc-build](https://github.com/hufans/fc-build)** 的 fork 改动、发布链路与日常运维。

面向读者：仓库维护者。  
用户向安装说明见 [README.md](./README.md)。

---

## 1. 项目定位

| 项 | 说明 |
|----|------|
| 本仓库 | [hufans/fc-build](https://github.com/hufans/fc-build) |
| CLI / 二进制 | **`fc`** |
| 配置目录 | **`~/.fc`** / `$FC_HOME` |
| 分发 | GitHub Actions + Release 标签 **`continuous`** |

**一句话：** 本地命令与安装名均为 `fc`；配置与进程环境使用 `FC_*` / `.fc` 命名；用 CI 滚动发版。

---

## 2. 仓库与 Git

| remote | URL | 用途 |
|--------|-----|------|
| `origin` | `git@github.com:hufans/fc-build.git` | 日常 push |

```sh
git remote -v
# origin  git@github.com:hufans/fc-build.git

git push origin main
# 触发 Build fc；成功后刷新 continuous Release
```

推送 workflow 若被 OAuth 拒绝，改用 SSH origin 或 `gh auth refresh -s workflow`。

---

## 3. 代码改动面（品牌 / 指纹）

| 文件 | 改动 |
|------|------|
| `xai-grok-pager-bin` | `[[bin]] name = "fc"`，`version_text` → `fc …` |
| `cli.rs` / `completions_cmd.rs` | clap 展示名、补全按 `fc` |
| `paths.rs` | 默认 home **`~/.fc`** |
| `static_shell.rs` / `shell_state.rs` / `embedded_search_tools.rs` | 子进程 argv：`FC_AGENT` / `__fc_*` |
| hooks / notifications / auth child env | **`FC_*`**（不向子进程 export 产品旧名） |
| 项目发现 | 优先 **`.fc/`**（agents / skills / workflows / sandbox / lsp） |
| `xai-grok-update` | `fc update` 从 `hufans/fc-build` continuous 自更新 |
| `scripts/install.sh` | 一键安装 |
| `.github/workflows/build-fc.yml` | 编译 + continuous |

**运行时去指纹要点：**

| 项 | 行为 |
|----|------|
| 配置目录 | `~/.fc` |
| 安装路径 | `~/.local/bin/fc` |
| 子进程哨兵 | `FC_AGENT=1` |
| Shell 包装 | `__fc_user_cmd` / `__fc_bin` / `__fc_shadow_*` |
| Hooks env | `FC_HOOK_*` / `FC_SESSION_ID` / `FC_WORKSPACE_ROOT` |

自检（子进程包装应为 `FC_AGENT` / `__fc_*`）：

```bash
strings "$(command -v fc 2>/dev/null || echo ~/.local/bin/fc)" \
  | grep -E 'FC_AGENT=1|__fc_user_cmd' | head
```

**说明：** 源码 crate 路径仍沿用 monorepo 目录名；用户可见产品名与仓库名为 **fc** / **fc-build**。

---

## 4. 本机构建

```sh
source "$HOME/.cargo/env"
export PROTOC="${PROTOC:-$HOME/.local/protoc-29.3/bin/protoc}"  # Intel Mac 等

cargo build -p xai-grok-pager-bin --release --bin fc
install -m 755 target/release/fc ~/.local/bin/fc
```

zsh 建议：

```sh
alias fc='$HOME/.local/bin/fc'
```

---

## 5. 安装与发版

### 用户

```sh
curl -fsSL https://raw.githubusercontent.com/hufans/fc-build/main/scripts/install.sh | bash
fc update
```

| 环境变量 | 默认 | 含义 |
|----------|------|------|
| `FC_REPO` | `hufans/fc-build` | install.sh 仓库 |
| `FC_TAG` / `FC_VERSION` | `continuous` | Release 标签 |
| `FC_BIN_DIR` | `~/.local/bin` | 安装目录 |
| `FC_RELEASE_REPO` | `hufans/fc-build` | `fc update` 仓库 |
| `FC_RELEASE_TAG` | `continuous` | `fc update` 标签 |

### CI

文件：[`.github/workflows/build-fc.yml`](./.github/workflows/build-fc.yml)

```text
push main → matrix 编译 → continuous Release
  assets: fc-linux-x86_64, fc-darwin-arm64
```

---

## 6. 上游同步

工作流：[`.github/workflows/sync-upstream.yml`](./.github/workflows/sync-upstream.yml)

- **每 6 小时**（UTC `0 */6 * * *`）及手动 `workflow_dispatch`：检查 `xai-org/grok-build`
- 有更新且无冲突 → merge → push `main` → `workflow_call` **Build fc** → 刷新 `continuous` Release  
- 有冲突 → Issue（或仓库禁用 Issue 时看 Actions 日志），本地 resolve 后 push  

```sh
git fetch upstream   # 若已配置
git merge upstream/main
# 品牌相关冲突保留 fc 命名
git push origin main
```

---

## 7. 验证清单

```sh
curl -fsSL https://raw.githubusercontent.com/hufans/fc-build/main/scripts/install.sh | bash
command -v fc          # 或 ~/.local/bin/fc
fc --version           # 期望：fc <ver> (<sha>)
fc --help | head -5
```

---

## 8. 故障排查

| 现象 | 处理 |
|------|------|
| install 404 | 等 CI 发 continuous，或本机编译 |
| Intel Mac install 拒绝 | 本机 `cargo build --bin fc` |
| `fc` 变成 shell 历史命令 | `alias fc='$HOME/.local/bin/fc'` |
| continuous 未更新 | 看 Actions 失败 job |

---

## 9. 维护约定

1. 品牌 diff 保持小：只动 CLI 名、路径、进程指纹相关文件。  
2. 本手册与行为同步。  
3. 发版以 CI continuous 为准。  
4. **README 面向用户，FC.md 面向维护者。**

---

## 10. 参考链接

| 资源 | URL |
|------|-----|
| 本仓库 | https://github.com/hufans/fc-build |
| Actions | https://github.com/hufans/fc-build/actions |
| continuous | https://github.com/hufans/fc-build/releases/tag/continuous |
| install.sh | https://raw.githubusercontent.com/hufans/fc-build/main/scripts/install.sh |

---

*产品名与仓库名：`fc` / `hufans/fc-build`。*
