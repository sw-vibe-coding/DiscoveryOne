# Step: front-edit-validation

Harden visible feedback for invalid Power Front edits.

Scope:
- Show honest validation feedback when Power Front edited text cannot be parsed into the narrow executable semantics.
- Prevent or clearly fail Run when the current edit is invalid.
- Keep Browse mode unchanged and keep preview-only messaging for unsupported non-Power/non-Front edits.

Verification:
- Add focused tests or snapshots for invalid edit feedback.
- Run web crate tests and relevant web reg-rs cases.

Commit message: web: validate executable front edits