# MiniMark agent instructions

## Mission

Build the smallest dependable local Markdown editor that satisfies `docs/PRODUCT.md`. Product behavior outranks implementation convenience.

## Working loop

1. Read the relevant product requirement and inspect the current repository state.
2. State the user-visible behavior, risks, and smallest affected seam.
3. For behavior changes, obtain agreement on the public test seam, then run one RED → GREEN cycle.
4. Keep the change to one vertical slice; update the plan when evidence changes it.
5. Run the narrow check during development and the full gate before proposing acceptance.
6. Request an independent read-only audit for data loss, correctness, scope drift, and missing edge cases.
7. Stop for human approval before committing, publishing, destructive actions, or expanding product scope.

## Product invariants

- The user's Markdown file is the source of truth.
- A crash, failed save, or interrupted operation must preserve the last recoverable user text.
- Local files remain local unless the user explicitly requests an external action.
- Visible behavior has a deterministic acceptance test or an explicit manual test.
- New complexity needs evidence from a current requirement or reproduced failure.

## Context pointers

- Product scope or feature proposal: read `docs/PRODUCT.md` and `.agents/skills/product-boundary/SKILL.md`.
- Desktop runtime, filesystem, windows, packaging, or crash recovery: read `.agents/skills/desktop-spike/SKILL.md` before choosing or changing technology.
- Markdown parsing, rendering, editing, serialization, or save behavior: read `.agents/skills/markdown-integrity/SKILL.md`.
- Tests: read `docs/TESTING-SEAMS.md`; no behavior test is written against an unapproved public seam.
- Process or phase changes: read `docs/WORKFLOW.md` and record the reason in `docs/DECISIONS.md`.

## Repository discipline

- `AGENTS.md` is the single source of shared agent instructions.
- Keep diffs focused and preserve unrelated user work.
- Do not commit unless the user explicitly accepts the completed slice.
- Record facts, inferences, and unresolved assumptions separately.
- Use synthetic fixtures; never place personal documents, paths, credentials, or machine details in tracked files.

