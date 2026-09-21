# Durable save and recovery spike

## Question

Can MiniMark save a Markdown file without silently overwriting an external edit, and can it offer an unfinished draft after restart while preserving the disk version?

## Evidence

Run:

```sh
cargo test
```

Observed on 2026-09-21: four integration tests passed.

- A matching disk version is replaced through a same-directory temporary file.
- A changed disk version is refused and preserved.
- An unfinished draft is offered after restart.
- When disk and draft both changed, both versions are preserved and the conflict is reported.

## Decision

Keep the fingerprint check plus write–flush–rename approach for the first vertical slice. Do not use direct truncating writes.

## Known limit

There is still a small time-of-check/time-of-use race between the fingerprint check and rename. This spike reduces the main data-loss risks; it does not prove absolute safety under simultaneous writers.
