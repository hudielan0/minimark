# Proposed public testing seams

No behavior tests will be written until these seams are approved.

## Seam A — Document session

Public behavior: open a selected Markdown file into an editing session, track user edits, and expose whether the session differs from disk.

Critical examples: empty file, Unicode/CJK, large text, invalid UTF-8 response, file changed externally.

## Seam B — Preview

Public behavior: convert current Markdown text into a safe preview without changing the source text.

Critical examples: raw HTML, script payloads, broken Markdown, very deep nesting, links and relative images.

## Seam C — Durable save

Public behavior: save the current text to the selected file without producing a partial file or silently overwriting a newer disk version.

Critical examples: permission denial, disk full, interrupted write, renamed file, external modification.

## Seam D — Draft recovery

Public behavior: after interruption, offer the newer recoverable draft while retaining the disk version and explaining the difference.

Critical examples: stale draft, corrupted recovery record, identical draft, recovery write failure.

## Seam E — Packaged user journey

Public behavior: in the installed desktop application, a user can open, edit, preview, save, force-close, relaunch, and recover a synthetic document.

## Approval requested

Approve these five seams before Phase 1. Individual implementation tests may exist below them, but acceptance is judged through these public behaviors.

