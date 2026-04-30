# Step: pipeline-canvas

Render the deterministic 2D pipeline canvas in the Web UI.

Scope:
- Add a Pipeline view/section that renders the bundled Power -> Output fixture with nodes, ports, and the connecting edge.
- Use stable layout and accessible labels suitable for HTML snapshots.
- Keep drag/drop, arbitrary editing, and persistence out of scope.
- Preserve existing facet, run, library, and front-edit demos.

Verification:
- Make the pipeline HTML snapshot portion of the RED baseline progress as appropriate.
- Run relevant web reg-rs cases and web crate tests.

Commit message: web: render pipeline canvas