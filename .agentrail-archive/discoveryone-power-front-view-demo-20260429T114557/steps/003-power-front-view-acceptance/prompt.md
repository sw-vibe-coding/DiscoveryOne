# Step: power-front-view-acceptance

Add Web acceptance coverage for the completed Power front-view demo.

Scope:
- Add or extend a reg-rs case that snapshots the Power front facet from the local/static Web output path.
- Ensure the acceptance output includes the structured Power notation and that existing M7 minted module acceptance remains green.
- Avoid broad UI redesign; this step is about durable demo proof.

Verification:
- Run the new acceptance case.
- Run all reg-rs cases if the targeted case passes.

Commit message: test: cover power front view demo