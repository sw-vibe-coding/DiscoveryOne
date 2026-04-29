# Step: web-browse-edit-validation

Add browse vs edit mode and invalid-source feedback for the Web demo.

Scope:
- Add a clear browse/read-only vs edit/read-write toggle for the source/facet editing surface.
- In edit mode, allow editing the visible source/facet text in a controlled way.
- When edited input is invalid, show useful error feedback and highlight the relevant source area as much as the current parser/checker supports.
- Keep the first implementation pragmatic; prefer parser/checker diagnostics already available in the repo over inventing a broad editor framework.

Verification:
- Add focused coverage for browse/edit state and invalid-source feedback where feasible.
- Run web crate tests and relevant reg-rs cases.

Commit message: web: add edit mode validation feedback