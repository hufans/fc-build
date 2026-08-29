---
name: fc-branding
description: >
  Mandatory fc-fork branding and anti-fingerprint rules for hufans/fc-build.
  Use whenever syncing or merging upstream (xai-org/grok-build), resolving merge
  conflicts, reviewing runtime paths/env/process markers, editing install/update/CI,
  shipping continuous, or when the user mentions grok→fc, fingerprints, EDR, scanners,
  rebrand, or FC_HOME. Also use after any monorepo "Synced from monorepo" merge.
  Slash: /fc-branding.
---

# fc branding & fingerprint rules

This repo is the **fc** fork of Grok Build. Crate paths may stay `xai-grok-*` (monorepo layout). **User-visible product, process, and on-disk fingerprints must stay fc.**

**Why this is hard:** `fc` is the only safe runtime name. Endpoint / EDR / process scanners intercept anything that still looks like Grok (process name, argv, exported env, wrap markers). A merge that “takes upstream” on those surfaces will get the binary blocked. Treat Grok process markers as a ship-blocker, not a leftover.

Do **not** wait for the user to remind you. After every upstream merge or any change that touches paths/env/shell/CLI identity, apply this skill and fix regressions before push.

## Runtime process info (must not contain Grok)

These surfaces are visible to scanners on a running process. **Must be `fc` / `FC_*` / `__fc_*`.** Do not export, spawn with, or leave inherited:

| Surface | Required | Forbidden on children |
|---------|----------|------------------------|
| Process / clap name | `fc` | `grok`, `kiro` |
| Version string | `fc <ver>…` | `grok <ver>…` |
| Tool-shell env | `FC_AGENT=1` | `GROK_AGENT=`, `KIRO_AGENT=` |
| Shell wrap argv | `__fc_user_cmd` / `__fc_bin` / `__fc_shadow_*` | `__grok_*`, literal `grok` in `-c` |
| OSC52 wrap env | `FC_OSC52_SINK` / `LC_FC_OSC52_SINK` | `GROK_OSC52_SINK` on the child |
| Hooks child env | `FC_HOOK_*` / `FC_SESSION_ID` / `FC_WORKSPACE_ROOT` | `GROK_HOOK_*` / `GROK_SESSION_ID` / `GROK_WORKSPACE_ROOT` |
| Home default | `~/.fc` via `$FC_HOME` | default `~/.grok` / `$GROK_HOME` as the **first** choice |

Compat **reads** of `$GROK_HOME` / `~/.grok` (seed credentials) are allowed. Compat **writes/exports** onto child processes are not.

## Non-negotiable defaults

| Surface | Required |
|---------|----------|
| Binary / clap | `fc` (not `grok` / `kiro`) |
| Version line | `fc <ver>…` |
| User home | `~/.fc` via `$FC_HOME` first, then `$KIRO_HOME`, then `$GROK_HOME` (compat only) |
| Project dirs | Prefer **`.fc/`** (agents, skills, workflows, hooks, personas, lsp, config); keep `.grok`/`.claude` as **read-compat** only when needed |
| Tool-shell sentinel | Export **`FC_AGENT=1` only**; strip `GROK_AGENT` / `KIRO_AGENT` from tool children |
| Shell wrap argv | `__fc_user_cmd` / `__fc_bin` / `__fc_shadow_*` — never put `grok` in `-c` wrappers |
| OSC52 / wrap env | Prefer `FC_OSC52_SINK` / `LC_FC_OSC52_SINK` (legacy GROK_* may remain as secondary) |
| Hooks child env | `FC_HOOK_*` / `FC_SESSION_ID` / `FC_WORKSPACE_ROOT`; strip inherited `GROK_HOOK_*` when setting FC_* |
| Update / install | `hufans/fc-build` continuous; `scripts/install.sh`; installer id `fc` |
| Docs | User docs = fc; maintainer notes in `FC.md` |

Authoritative short map: [FC.md](../../../FC.md) §3. Always-on agent rule: [AGENTS.md](../../../AGENTS.md).

## When you must run this

1. **Any** `git merge upstream/main` / Sync upstream conflict resolution  
2. Editing `xai-dirs`, `paths`, shell wrap, hooks env, pager bin/cli, update, discovery  
3. Before push that ships runtime behavior  
4. User asks about detection / EDR / “还有没有 grok 字样”

## Conflict resolution policy (upstream merge)

1. Take **upstream functional** changes.  
2. Re-apply **fc naming** on the conflict surface — never “resolve by taking theirs” on branding files.  
3. New shared helpers (e.g. `xai_dirs::grok_home`) must default to **`.fc` / `FC_HOME`**, not `.grok`.  
4. Prefer combining: upstream API + fc constants (see recent merges in `paths`, `auto_update`, `static_shell`).

## Mandatory post-merge checklist

Run against **production** code (you may ignore pure tests if time-boxed, but fix production first).

### Must pass (block push if fail)

- [ ] `xai-dirs` (or equivalent): default dirname **`.fc`**, env order **FC → KIRO → GROK**
- [ ] `[[bin]] name = "fc"` and clap `name = "fc"`; version text starts with **`fc `**
- [ ] Tool shells: `FC_AGENT=1`; no export of `GROK_AGENT`/`KIRO_AGENT` on tool children
- [ ] Shell wrappers use `__fc_*` only in argv-facing `-c` strings
- [ ] Project discovery lists **`.fc/...` first** where fork already did (agents/skills/lsp/hooks)
- [ ] `fc update` / continuous still points at `hufans/fc-build`

### Should fix when touched (high detection value)

- [ ] Network client types: prefer `fc-pager` / `fc-shell` over `grok-pager` / `grok-shell` when changing identity code  
- [ ] New project writers: do **not** introduce new hard-coded `{cwd}/.grok` creates; use `.fc` or dual-list with `.fc` first  
- [ ] User-facing strings in **new** UI: `~/.fc`, not `~/.grok`  
- [ ] OSC52 / appearance wrap children: FC_* preferred

### Acceptable residual (do not churn the whole monorepo)

- Crate/path names `xai-grok-*`, Rust module paths in traces  
- Compat reads of `.grok` / `$GROK_HOME` / legacy seed from `~/.grok`  
- Test fixtures that intentionally use `.grok`  
- Upstream protocol/docs strings not on the process-fingerprint path  

## Quick scan commands

```bash
# Defaults and bin
rg -n 'DEFAULT_HOME_DIRNAME|name = "fc"|name = "grok"|"fc \{\}|"grok \{\}' \
  crates/codegen/xai-dirs crates/codegen/xai-grok-pager-bin crates/codegen/xai-grok-pager/src/app/cli.rs

# Tool-shell fingerprints
rg -n 'FC_AGENT|GROK_AGENT|__fc_user_cmd|__grok_' \
  crates/codegen/xai-grok-tools/src/util/env.rs \
  crates/codegen/xai-grok-tools/src/computer/local/static_shell.rs \
  crates/codegen/xai-grok-tools/src/computer/local/shell_state.rs \
  crates/codegen/xai-grok-tools/src/computer/local/terminal.rs

# New hard-coded project .grok writers (review each hit)
rg -n 'join\("\.grok"\)|/\.grok' crates/codegen --glob '!**/*test*' --glob '!**/tests/**'
```

After a release build:

```bash
strings "$(command -v fc 2>/dev/null || echo target/release/fc)" \
  | grep -E 'FC_AGENT=1|__fc_user_cmd|^\.fc$|name = "fc"|grok-shell|GROK_AGENT' | head
```

Expect: `FC_AGENT=1`, `__fc_user_cmd`, `.fc`. Investigate new `GROK_AGENT=` **exports** or default home `.grok`.

## Output when reporting

When you ran this skill (e.g. after sync), briefly report:

1. What was checked  
2. Regressions fixed (files)  
3. Known residual risks left intentionally  

Do not claim “fully grok-free binary”; claim **process/default-path fingerprints remain fc**.
