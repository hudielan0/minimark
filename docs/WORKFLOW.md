# MiniMark development workflow

This workflow follows the sequence evidenced in VMark: capability preparation, runnable vertical slice, rapid small changes, independent audit, then structural gates learned from real failures.

Every phase produces two parallel outputs: product evidence and a method card. The method card records what was learned, why the sequence was chosen, rejected alternatives, reusable steps, unresolved risks, and the decisions reserved for the human owner. See `METHOD.md`.

## Phase 0 — Capability preparation

Deliverables:

- Product hypothesis, first-release promise, non-goals, and trust boundary
- Shared `AGENTS.md`
- Three focused capability skills
- Proposed public test seams
- A Git repository containing no product implementation

Exit criterion: a new agent can state what MiniMark must do, what it must not do, and what remains unverified without guessing.

## Phase 1 — Risk spikes

Build disposable probes for the four unknowns in `PRODUCT.md`. Each spike records the question, smallest experiment, observed result, rejected alternatives, and decision.

Exit criterion: one desktop stack can open, edit, preview, save atomically, and recover synthetic text in a development build.

## Phase 2 — First vertical slice

Implement one user journey: choose a Markdown file → edit → preview → save. Work one behavior test at a time against approved seams.

Exit criterion: the journey works on a synthetic file and passes automated and manual checks.

## Phase 3 — Recovery slice

Add bounded draft recovery and disk-conflict handling. Treat any possible text loss as a release blocker.

Exit criterion: forced interruption and external file modification scenarios preserve both versions or require an explicit user choice.

## Phase 4 — Independent audit

Give a read-only reviewer the product contract, diff, tests, and reproduction steps. The reviewer searches for data loss, untested state transitions, unsafe rendering, and scope drift. The human accepts or rejects each finding.

Exit criterion: every accepted finding is fixed with a reproducing test; rejected findings include evidence.

## Phase 5 — Packaging and first release

Package for macOS, run the complete manual journey, publish checksums and release notes, and retain human approval for release.

Exit criterion: a fresh machine can install and complete the journey without developer tools.

## Phase 6 — Learning loop

For each reproducible failure: reproduce → add a regression test → fix → audit → classify the lesson as test, project rule, reusable skill, safety hook, or release gate.

Exit criterion: the repository explains why every added control exists and the failure that justifies its continued cost.
