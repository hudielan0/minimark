# MiniMark product contract

## Hypothesis

For people who frequently use Markdown but do not want to configure developer tools, MiniMark provides a local, fast, dependable editor that protects unfinished writing.

## Primary user

A writer, researcher, operator, or learner who works with Markdown files and wants a normal desktop application rather than a development environment.

## First release promise

A user can:

1. Open one local `.md` file.
2. Edit plain Markdown text.
3. See a readable preview.
4. Save back to the same file deliberately.
5. Recover newer unfinished text after an app interruption.

## Non-goals for the first release

- AI writing features
- Cloud sync, accounts, or collaboration
- Multiple tabs or workspaces
- Plugin system
- Non-Markdown formats
- Mobile or web versions
- Rich-text/WYSIWYG editing

## Trust boundary

- MiniMark reads only a file the user selected and its own recovery state.
- It does not upload document contents.
- It never silently replaces a newer disk version.
- Recovery data is visible, bounded, and removable.

## Definition of done

The first release is complete when the five promised behaviors pass their agreed tests, the packaged application completes a manual open-edit-preview-save-recover journey, and an independent read-only audit reports no known data-loss path.

## Unknowns to resolve in Phase 1

- Desktop runtime and packaging stack
- Preview renderer and sanitization boundary
- Atomic-save mechanism on macOS
- Recovery storage format and conflict policy

