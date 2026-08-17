# Agent rules — fc-build fork

This repository is **[hufans/fc-build](https://github.com/hufans/fc-build)** (`fc` CLI), a branded fork of `xai-org/grok-build`.

## Always apply (no user reminder needed)

1. Load and follow the project skill **`fc-branding`** (`.fc/skills/fc-branding/SKILL.md`) whenever you:
   - merge/sync **upstream** monorepo changes,
   - resolve merge conflicts,
   - touch home paths, env sentinels, shell wrap, CLI name, install/update, or discovery,
   - push runtime-affecting changes.
2. **Default product fingerprints must stay `fc`**: binary `fc`, home `~/.fc` / `$FC_HOME`, tool-shell `FC_AGENT=1` + `__fc_*` wrappers. Do not reintroduce `GROK_AGENT` exports or default `~/.grok` for new code.
3. Crate directory names may remain `xai-grok-*` (upstream layout). That is not a license to use `grok` for user-facing CLI name, default config dir, or tool-child process markers.
4. Maintainer handbook: [FC.md](./FC.md). User install: [README.md](./README.md).

## Upstream sync

- Prefer functional upstream changes; on conflicts, **keep fc branding** (see `fc-branding` skill).
- After sync: run the skill checklist, then push so **Build fc** refreshes `continuous`.
