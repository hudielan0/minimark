# Phase 1 risk spikes

These probes answer product risks; they are not production application code.

## Environment

- macOS 26.5.1 on Apple Silicon
- Node.js 22.23.2
- Rust 1.98.1
- Apple Command Line Tools

## Runtime decision question

Can Tauri 2 provide MiniMark's local file access, restricted capabilities, native dialogs, and macOS packaging without shipping a bundled browser runtime?

User-facing failure if wrong: the app is not genuinely lightweight, cannot access a file the user selected, or requires developer tools after installation.

### Current evidence

- Tauri uses the operating system webview and a separate Rust core process.
- Tauri provides official dialog and filesystem plugins with explicit capability permissions.
- Tauri requires Rust and Apple build tools for development; both are now available on this machine.
- Electron provides mature dialogs and packaging but bundles Electron/Chromium, which is a worse starting fit for MiniMark's lightweight promise.

### Provisional decision

Use Tauri 2 for the Phase 1 development build. Reconsider only if a runnable spike cannot complete the required open, preview, durable-save, and recovery journey.

## Observed results

- Durable save and recovery: four integration tests pass.
- Markdown rendering boundary: two tests pass, including malicious HTML removal.
- Thin desktop shell: frontend production build and Rust compile check pass.
- macOS application bundle: generated successfully at `spikes/tauri-shell/src-tauri/target/debug/bundle/macos/MiniMark Phase 1.app`.
- DMG packaging: failed at Tauri's final `bundle_dmg.sh` step. This remains packaging work, not evidence against the runtime choice.

The shell now joins native file selection, source editing, sanitized preview, delayed recovery writes, conflict-aware save, and a recovery choice after restart. It is a risk probe, not the product architecture or a release build.

## Spike order

1. Durable save and external-change conflict detection.
2. Safe Markdown preview without source mutation.
3. Bounded recovery record and restart detection.
4. Thin Tauri shell joining the three probes into one synthetic journey.
