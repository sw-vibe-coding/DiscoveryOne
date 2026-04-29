# Step: library-grid-fixtures

Extend M8 grid fixtures/metadata toward the planned roughly six-definition library snapshot.

Scope:
- Add lightweight bundled definition metadata or fixtures sufficient for a richer library grid.
- Keep parser/checker/runtime scope unchanged unless an existing fixture already supports the path.
- Ensure the grid still sorts stably and existing Power/minted demos remain green.

Verification:
- Run library grid snapshots/reg-rs cases, existing web reg-rs cases, and web crate tests.

Commit message: web: expand library grid fixtures