# Step: pipeline-model

Add the narrow M9 pipeline data model.

Scope:
- Define a small serializable pipeline structure for nodes, ports, edges, and run inputs.
- Include a bundled Power -> Output fixture used by snapshots/tests.
- Keep persistence and drag/drop out of scope.
- Keep the model narrow enough for the M9 demo, but shape it so later UI and validation steps can consume it.

Verification:
- Add focused unit tests for fixture shape and deterministic ordering.
- Keep existing web tests green.

Commit message: web: add pipeline model fixture