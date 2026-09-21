# MiniMark decisions

## D-001 — Begin with capabilities, not application code

- Status: accepted
- Date: 2026-09-21
- Decision: establish the product boundary, shared agent rules, focused skills, and proposed test seams before selecting the desktop stack.
- Evidence: VMark's first public commit prepared Claude Code skills before the application scaffold.
- Revisit when: the capability documents fail to change agent behavior or block the first risk spike without reducing product risk.

## D-002 — Keep the first release deliberately narrow

- Status: accepted
- Date: 2026-09-21
- Decision: ship one-file Markdown editing, preview, durable save, and recovery before AI features, tabs, workspaces, or plugins.
- Reason: the learning objective is to validate the production system through one complete, high-trust journey.
- Revisit when: the first release definition of done is satisfied.

## D-003 — Use Tauri 2 for the first development build

- Status: provisional
- Date: 2026-09-21
- Decision: use Tauri 2 with a vanilla TypeScript interface and a small Rust core.
- Evidence: a macOS application bundle now compiles with a native file dialog, restricted default capability, local recovery storage, and no bundled Chromium runtime.
- Alternative considered: Electron has mature desktop APIs and packaging, but its bundled runtime is a worse first fit for MiniMark's lightweight promise.
- Revisit when: the runnable shell cannot complete the open–edit–preview–recover–save journey, or platform support becomes more costly than the size benefit.

## D-004 — Refuse blind overwrites

- Status: provisional
- Date: 2026-09-21
- Decision: fingerprint the opened file, write to a same-directory temporary file, flush it, rename it, and refuse save when the disk fingerprint changed.
- Evidence: four automated tests cover normal save, external modification, restart recovery, and a conflicting recovery draft.
- Rejected: direct truncating writes, because interruption or an unnoticed external edit could destroy the only good copy.
- Known limit: a small race remains between the final fingerprint check and rename.
- Revisit when: the first vertical slice adds platform-specific file coordination or exposes a simultaneous-writer failure.

## D-005 — Treat preview HTML as untrusted output

- Status: provisional
- Date: 2026-09-21
- Decision: keep Markdown source separate, parse with Marked, and sanitize every preview with DOMPurify before inserting it into the page.
- Evidence: tests preserve the source while removing scripts, event handlers, and executable links.
- Revisit when: the product defines remote image and embedded-content policy.
