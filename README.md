# MiniMark

MiniMark is a local Markdown editor for people who want a fast, dependable writing tool without configuring a developer environment.

## Product promise

Open a Markdown file, edit it, preview it, save it, and recover unfinished work after an interruption—without an account or cloud service.

## Status

Phase 0: product boundary and AI capabilities. No application code has been written yet.

## Development method

This repository is also a public learning record. It reconstructs the development loop evidenced in VMark without copying VMark's product code:

1. Prepare AI capabilities and a small project constitution.
2. Prove risky assumptions with runnable spikes.
3. Build one end-to-end vertical slice.
4. Add behavior test-first, one slice at a time.
5. Audit with an independent model in a read-only role.
6. Turn real failures into tests, rules, skills, or mechanical gates.
7. Release small versions and learn from reproducible user reports.

See [Product](docs/PRODUCT.md), [Workflow](docs/WORKFLOW.md), and [Testing seams](docs/TESTING-SEAMS.md).

