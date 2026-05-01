# Step: viewer-red

Create the M10 RED acceptance baseline for a deterministic 3D viewer fixture.

Scope:
- Add a reg-rs baseline that describes the expected 3D viewer demo surface before implementation.
- Target the Power definition and a stable snapshot/output that can run without a browser GPU.
- Include the expected exported symbol count/labels and viewer asset hooks, but keep the baseline narrow.
- Do not implement the viewer yet.

Verification:
- Run the new reg-rs case and confirm it fails for the expected missing viewer/export behavior.
- Run existing web smoke regressions that should remain green.

Commit message: test: add 3d viewer red baseline