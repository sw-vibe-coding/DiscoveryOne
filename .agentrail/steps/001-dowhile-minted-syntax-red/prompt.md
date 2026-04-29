# Step: dowhile-minted-syntax-red

Create the M7 RED baseline for minted syntax through the CLI.

## Work
1. Add examples/dowhile.d1 containing a *syntax do _ while _ end expand declaration plus a use site that counts 1, 2, 3 and returns 3.
2. Add reg-rs case d1_poc_dowhile_minted_syntax expecting the scripted d1 run output `1 2 3\n3\n`, with the current implementation failing deterministically.
3. Preserve all existing baselines.
4. Update docs/status.md to record the RED M7 baseline.
5. Run sw-checklist before committing.

## Commit / complete protocol
Commit deliverables plus .agentrail/ with message:
minted: add dowhile red baseline
Then push and agentrail complete with reward 1.