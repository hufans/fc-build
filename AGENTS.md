# Agent rules — fc-build fork

This repository is **[hufans/fc-build](https://github.com/hufans/fc-build)** (`fc` CLI), a branded fork of `xai-org/grok-build`.

**`fc` is the only safe product name.** Runtime process information that still looks like Grok (binary name, argv, exported env, PTY wrap markers, hook child env) is treated as the old product and **will be intercepted**. This is a hard security constraint, not a style preference.

## Always apply (no user reminder needed)

1. Load and follow the project skill **`fc-branding`** (`.fc/skills/fc-branding/SKILL.md`) whenever you:
   - merge/sync **upstream** monorepo changes,
   - resolve merge conflicts,
   - touch home paths, env sentinels, shell wrap, CLI name, install/update, or discovery,
   - push runtime-affecting changes.
2. **Runtime process fingerprints must stay `fc`.** Never ship new (or reintroduce) Grok process markers:
   - binary / clap / version line: `fc`
   - home: `~/.fc` / `$FC_HOME` first
   - tool-shell: export **`FC_AGENT=1` only**; strip `GROK_AGENT` / `KIRO_AGENT` from children
   - wrap argv: `__fc_*` only — never `grok` in `-c` wrappers
   - hook / wrap / auth child env: **`FC_*`**; strip inherited `GROK_*` product keys when injecting FC_*
3. Crate directory names may remain `xai-grok-*` (upstream layout). That is **not** a license to put `grok` in user-facing CLI name, default config dir, or **any child-process argv/env**.
4. Maintainer handbook: [FC.md](./FC.md). User install: [README.md](./README.md). Authoritative runtime checklist: `.fc/skills/fc-branding/SKILL.md`.

## Upstream sync

- Prefer functional upstream changes; on conflicts, **keep fc branding** (see `fc-branding` skill).
- After sync: run the skill checklist, then push so **Build fc** refreshes `continuous`.
