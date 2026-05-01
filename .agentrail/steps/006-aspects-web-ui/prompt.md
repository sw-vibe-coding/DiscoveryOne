# Step: aspects-web-ui

Expose the M11 aspect demo in the Web UI.

Scope:
- Add a compact inspection/control surface for trace/profile/on-error aspect toggles or status.
- Preserve existing M7-M10 demo panels and snapshots.
- Keep the UI browse/inspect focused; do not add a full aspect editor.

Verification:
- Add/update Web UI snapshots for the aspect surface.
- Run M7-M10 regressions, aspect baselines, web crate tests, wasm build, and sw-checklist.

Commit message: web: show aspect toggles