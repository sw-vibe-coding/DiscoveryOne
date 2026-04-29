# Step: front-edit-run-green

Route Web RunPanel execution through valid edited Power Front semantics.

Scope:
- Thread the current facet edit state into the RunPanel execution path.
- When Power Front edit text is valid, run Power using the edited zero-exponent base literal.
- Preserve unedited Power behavior: n=2,e=8 still outputs 256.
- Preserve minted demo behavior.
- Keep edited execution narrow to Power Front and the base literal demo.

Verification:
- Make the RED acceptance from step 001 pass.
- Run existing Power web, minted web, and web crate tests.

Commit message: web: run edited power front semantics