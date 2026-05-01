# Step: viewer-verification

Make the M10 3D viewer demo verifiable and demoable.

Scope:
- Add focused verification for exported scene data, viewer asset wiring, and the manual localhost demo path.
- Add or update scripts only as needed to make the demo repeatable.
- If browser automation is practical in the repo, include a smoke check that the viewer mount is non-empty; otherwise document the manual check precisely.

Verification:
- Run the M10 acceptance baseline, web regressions, web crate tests, and sw-checklist.
- Confirm existing M7-M9 demos remain green.

Commit message: test: verify 3d viewer demo