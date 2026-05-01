# Step: symbol-export

Expose positioned DiscoveryOne symbols for the 3D viewer.

Scope:
- Add a narrow exported data shape for the Power definition symbols using the same source/projection path as the Yew facet view.
- Provide deterministic JSON-friendly fields for symbol text, face/layer, and coordinates needed by three.js.
- Keep the boundary small and specific to the M10 demo; do not design a full scene graph format.

Verification:
- Add focused tests or snapshots for the exported Power symbol data.
- Run the 3D viewer RED baseline, existing Power/front-edit web regressions, and web crate tests.

Commit message: web: export 3d symbol data