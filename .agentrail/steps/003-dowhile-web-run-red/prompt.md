# Step: dowhile-web-run-red

Create the M7 RED web baseline for running the dowhile minted syntax demo.

## Work
1. Add reg-rs case d1_poc_dowhile_web_run for the web demo path opening examples/dowhile.d1, showing the syntax declaration in the Internal Z layer, and producing `1 2 3\n3\n`.
2. Add or update a deterministic web snapshot script for this case, failing deterministically until the web path exists.
3. Preserve all existing baselines.
4. Update docs/status.md.
5. Run sw-checklist before committing.

## Commit / complete protocol
Commit deliverables plus .agentrail/ with message:
web: add dowhile red baseline
Then push and agentrail complete with reward 1.