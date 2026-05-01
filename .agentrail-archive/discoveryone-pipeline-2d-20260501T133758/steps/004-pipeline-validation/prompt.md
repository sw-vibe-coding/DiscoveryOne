# Step: pipeline-validation

Validate narrow 2D pipeline connections and surface errors.

Scope:
- Add validation for the bundled pipeline's required edge and compatible port shape.
- Produce deterministic inline validation output for missing or incompatible edges.
- Keep the validation narrow; do not build a general type system beyond the current demo needs.

Verification:
- Add focused tests/snapshots for valid and invalid pipeline validation.
- Run relevant web reg-rs cases and web crate tests.

Commit message: web: validate pipeline edges