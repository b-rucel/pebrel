# Per-tab wallpaper overrides

## Status

Accepted

## Context

The existing wallpaper preference belongs to application runtime settings and is
rendered through one app-wide `VisualEffects` global. Terminal tabs need optional
individual images while retaining the existing image as the inherited default.
Session snapshots are also exported as workspace files.

## Evidence

- `nebula_settings::RuntimeSettings` owns the global image path and presentation
  preferences.
- GPUI terminal tabs already have aligned `TabMeta` state and serialize through
  `session::TabSession` for autosave, restore, and workspace import/export.
- Wallpaper decoding already runs asynchronously with resource bounds.

## Decision

Add an optional image path to each terminal tab's session metadata. Missing values
remain backward-compatible and inherit the global wallpaper. The tab image covers
the terminal card, including all split panes; fit, alignment, opacity, and
whole-window coverage remain shared appearance preferences. With whole-window
coverage enabled, the active tab's image replaces the global image behind both the
terminal and window chrome; without it, the active tab image stays in the terminal
card. Workspace JSON stores the path only and does not copy or bundle the image.
Cache decoded tab images by path with a bounded render-image cache.

## Rejected alternatives

- Per-pane images: they multiply state and UI controls without a requested workflow.
- Bundling image files in workspace exports: portability is not needed for this use.
- A session schema version bump: the optional field is additive and old v4 data
  deserializes to inheritance.

## Consequences

Selecting a tab image overrides the global image only for that tab; clearing the
override restores inheritance. Shared opacity, fit, alignment and whole-window
coverage apply to the selected tab image. Moving, duplicating, restoring, or
exporting a terminal tab carries its path. If the file is missing or not cached,
the global image remains the fallback. The bounded render cache may reload an
image after eviction.

## Validation

Session serialization has a regression test for round-trip and old-v4 defaulting.
Wallpaper settings tests cover shared opacity and whole-window preference state.
The requested implementation increment intentionally skips application builds;
native UI behavior remains to be validated in a later build increment.

## Supersedes

None

## Revisit when

Per-tab-specific fit/opacity controls or portable workspace assets are requested.
