---
name: markdown-integrity
description: Preserve user text when implementing Markdown loading, editing, preview, serialization, saving, recovery, or external-file conflict behavior.
---

# Markdown integrity

Treat the source text as the user's artifact; preview is a derived view.

For each change:

1. Identify whether it reads, transforms, renders, stores, or replaces source text.
2. State the round-trip invariant and the failure behavior.
3. Use synthetic fixtures covering empty text, Unicode/CJK, line endings, malformed Markdown, and large input.
4. Keep rendering sanitization separate from source preservation.
5. Make save atomic or preserve a recoverable prior/new version.
6. Surface external modification as a conflict; preserve both versions until the user chooses.
7. Add a regression fixture for every discovered corruption or loss path.

Completion criterion: the source bytes or explicitly documented normalization survive the tested journey, and every failure preserves recoverable user text.

