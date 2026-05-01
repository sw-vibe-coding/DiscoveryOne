# Step: threejs-scene

Render the exported Power symbols in a three.js scene bundle.

Scope:
- Add the smallest practical three.js integration for the Web UI build.
- Render positioned glyph blocks or labels from the exported Power symbol data.
- Provide deterministic scene initialization hooks suitable for smoke/snapshot checks.
- Keep authoring, selection editing, and complex camera persistence out of scope.

Verification:
- Add a script or snapshot that confirms the scene bundle is present and wired to the exported data.
- Run the 3D viewer baseline, web crate tests, and sw-checklist.

Commit message: web: add threejs symbol scene