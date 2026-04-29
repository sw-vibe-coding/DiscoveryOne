# Step: dowhile-minted-syntax-green

Implement the minimum parser/runtime support for the dowhile minted syntax so d1_poc_dowhile_minted_syntax is GREEN.

## Work
1. Wire *syntax T expand E template registration in the parser for the dowhile fixture.
2. Apply longest-match-wins and first-declared-wins-on-ties for registered syntax templates where needed for this slice.
3. Extend the current CLI/runtime path just enough for examples/dowhile.d1 to print `1 2 3\n3\n`.
4. Preserve all existing baselines.
5. Update docs/status.md.
6. Rebuild/reinstall d1 before fixture/reg-rs checks if d1 changes, then run sw-checklist before committing.

## Commit / complete protocol
Commit deliverables plus .agentrail/ with message:
minted: run dowhile syntax
Then push and agentrail complete with reward 1.