---
name: fc-actions
description: >
  Diagnose and fix GitHub Actions on hufans/fc-build. Default path: Sync
  upstream merge conflicts — take upstream function, keep fc process fingerprints,
  push main so Build fc refreshes continuous. Use when the user says Actions
  failed, CI 红了, Sync upstream 失败, github 有错误, 处理一下, merge conflict,
  or /fc-actions. Branding rules live in fc-branding; this skill is the
  operations loop.
---

# Fix fc-build GitHub Actions

Repo: **hufans/fc-build**. Upstream: **xai-org/grok-build**.

When the user reports Actions errors (including just “处理一下”), **do this loop**. Do not ask for the fc-branding background again.

Load **`fc-branding`** (`.fc/skills/fc-branding/SKILL.md`) for conflict policy and the post-merge checklist. Do not copy those tables here.

## 1. Diagnose

```sh
gh run list --repo hufans/fc-build --limit 15
```

Classify the latest failure:

| Workflow | Typical cause | Action |
|----------|---------------|--------|
| **Sync upstream** + `Fail when merge conflicts` | `upstream/main` vs fc fingerprints | §2 resolve and push |
| **Build fc** compile/smoke | real build break | logs → fix code (still fc-branding) → push |
| Other | report, don't guess a merge |

For Sync conflicts, pull `CONFLICT (content):` lines from the failed job log (`gh api repos/hufans/fc-build/actions/jobs/<id>/logs`).

## 2. Sync upstream conflicts

1. Fetch via HTTPS if SSH to GitHub fails:
   `git fetch https://github.com/hufans/fc-build.git main:refs/remotes/origin/main`
   `git fetch https://github.com/xai-org/grok-build.git main:refs/remotes/upstream/main`
2. Work from **GitHub `main` SHA**, not unpushed local extras (especially `.github/workflows` commits — OAuth often lacks `workflow` scope). Park unrelated local commits on a side branch if needed.
3. `git merge upstream/main` with a `chore: sync from upstream YYYY-MM-DD` message. On conflict: **upstream function + fc names**. Never “take theirs” on binary name, `FC_AGENT` / `__fc_*`, `FC_*` child env, `~/.fc` / `$FC_HOME` first, `hufans/fc-build` updates, `CARGO_BIN_EXE_fc`. Combine (e.g. new API + `.fc` constants).
4. `rustfmt` resolved Rust. `cargo check` the touched crates.
5. Run the **fc-branding** must-pass checklist. Then commit the merge and push. Prefer HTTPS: `git push https://github.com/hufans/fc-build.git HEAD:main`
6. Confirm **Build fc** queued/running. Report SHA, CLI version (`xai-grok-pager-bin` / `xai-grok-version`), Build fc URL, branding result, residuals.

If push is rejected for **workflow scope**, drop or park `.github/workflows` changes; do not block the sync merge.

## 3. Report

Short: what failed, what you merged, version, Build fc link. Process/default-path fingerprints remain **fc**.
