# Step: facet-edit-surface

Fix Web Edit mode so it edits the visible 2D facet syntax, not the raw/internal source fixture text.

Scope:
- Change edit mode to initialize from the selected definition/face rendered facet text, starting with Power Front.
- Keep Browse mode rendering unchanged.
- Keep validation feedback visible, but make the messaging honest when a 2D facet edit cannot yet be reverse-projected into full source.
- Do not implement a broad source model or persistence layer in this step; this is an ergonomic correction for the edit surface.

Verification:
- Add focused web tests or snapshots where feasible.
- Run relevant web reg-rs cases and web crate tests.

Commit message: web: edit visible facet syntax