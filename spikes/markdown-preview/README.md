# Safe Markdown preview spike

## Question

Can MiniMark render useful Markdown while keeping the source text unchanged and removing executable HTML?

## Evidence

Run:

```sh
npm test
```

Observed on 2026-09-21: two tests passed.

- Chinese headings, lists, and emphasis render without changing the source.
- Scripts, event handlers, and `javascript:` links are removed.

## Decision

Parse with Marked, sanitize every generated preview with DOMPurify, and keep source text separate from preview HTML.

## Known limit

The policy for remote images is not settled. The first product slice should block or explicitly control network-loaded content before release.
