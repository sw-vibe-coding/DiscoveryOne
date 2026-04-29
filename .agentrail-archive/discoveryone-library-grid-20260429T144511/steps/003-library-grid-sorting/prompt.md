# Step: library-grid-sorting

Add stable library grid sorting for M8.

Scope:
- Add sort controls for arity, type/category, aspect/facet summary, and name.
- Use definition name as the stable secondary key where needed.
- Add or extend snapshot/reg-rs coverage for stable ordering.
- Preserve existing web demo behavior.

Verification:
- Run targeted sort snapshots/reg-rs cases, existing web reg-rs cases, and web crate tests.

Commit message: web: sort library grid stably