# Step: facet-snapshot-front-red

Create the first M6 RED baseline for the authoritative Power front facet snapshot.

## Work
1. Add reg-rs case d1_facet_snapshot_power_front for d1 facet-snapshot examples/power.d1 --face front.
2. Capture the current RED behavior without implementing facet-snapshot.
3. Preserve all existing baselines.
4. Update docs/status.md.
5. Run sw-checklist before committing.

## Commit / complete protocol
Commit deliverables plus .agentrail/ with message:
web: add front facet snapshot red baseline
Then push and agentrail complete with reward 1.