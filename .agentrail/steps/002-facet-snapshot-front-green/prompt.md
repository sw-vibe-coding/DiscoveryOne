# Step: facet-snapshot-front-green

Implement the minimal CLI facet-snapshot path that turns d1_facet_snapshot_power_front GREEN.

## Work
1. Wire d1 facet-snapshot examples/power.d1 --face front through the existing source projection path.
2. Preserve face, normalize, parse/check, IR/interp, and WASM baselines.
3. Rebase d1_facet_snapshot_power_front to GREEN.
4. Update docs/status.md.
5. Rebuild and reinstall d1 before fixture/reg-rs/checklist tests.
6. Run sw-checklist before committing.

## Commit / complete protocol
Commit deliverables plus .agentrail/ with message:
web: snapshot front facet
Then push and agentrail complete with reward 1.