# Step: yew-viewer-panel

Embed the M10 3D viewer in the Yew Web UI.

Scope:
- Add a viewer panel for the Power definition that mounts the three.js scene without disrupting the existing facet, run, library, and pipeline demos.
- Include a controlled no-WebGL/loading fallback that still communicates build and data wiring state.
- Keep the UI inspection-focused; do not add 3D editing controls.

Verification:
- Update or add Web UI snapshots/acceptance checks for the viewer panel.
- Run existing Power, front-edit, library, pipeline, minted, and web crate tests.

Commit message: web: embed 3d viewer panel