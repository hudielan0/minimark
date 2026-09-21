---
name: desktop-spike
description: Evaluate MiniMark desktop runtimes, filesystem access, packaging, lifecycle, crash recovery, or platform-specific behavior through a disposable evidence-producing spike.
---

# Desktop risk spike

1. State one falsifiable technical question.
2. List the user-facing failure if the assumption is wrong.
3. Build the smallest disposable probe using synthetic files.
4. Record the exact environment, command, result, and failure output.
5. Test the negative path, including denied access or interruption when relevant.
6. Compare alternatives only on requirements in `docs/PRODUCT.md`.
7. Write the decision and rejected options to `docs/DECISIONS.md`; remove product code accidentally created by the spike.

Completion criterion: the question has observed evidence, a reproducible probe, and a bounded decision; uncertainty is named rather than converted into confidence.

