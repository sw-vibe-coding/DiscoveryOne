# Step: facet-snapshot-all-faces

Add GREEN facet-snapshot baselines for the remaining Power faces using the existing facet-snapshot path.

## Work
1. Add reg-rs cases d1_facet_snapshot_power_left, d1_facet_snapshot_power_right, d1_facet_snapshot_power_top, d1_facet_snapshot_power_bottom, and d1_facet_snapshot_power_rear.
2. Extend facet-snapshot only as needed for face aliases or stable output.
3. Preserve all existing baselines.
4. Update docs/status.md.
5. Rebuild and reinstall d1 before fixture/reg-rs/checklist tests if d1 changes.
6. Run sw-checklist before committing.

## Commit / complete protocol
Commit deliverables plus .agentrail/ with message:
web: snapshot all power facets
Then push and agentrail complete with reward 1.