# Step: web-runnable-controls

Make Web demo controls functional for definition selection, editable inputs, and rerun.

Scope:
- Fix the definition dropdown so selecting Power, DowhileCounter, or UnlessCounter changes the active definition.
- Make n/e fields editable where relevant.
- Make the Run button rerun the active definition with current inputs and update output.
- Preserve existing static snapshot helpers or update them deliberately with coverage.

Verification:
- Add/extend focused tests or snapshots for changed controls where feasible.
- Run web crate tests and relevant reg-rs cases.

Commit message: web: make run controls interactive