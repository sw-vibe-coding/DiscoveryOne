# Step: unless-minted-syntax-green

Generalize minted syntax support so the unless/do CLI baseline is GREEN.

## Work
1. Remove any dowhile-only assumptions from syntax template matching/expansion needed by the unless/do fixture.
2. Make d1_poc_unless_minted_syntax GREEN while preserving d1_poc_dowhile_minted_syntax.
3. Preserve all existing baselines.
4. Update docs/status.md.
5. Rebuild/reinstall d1 before fixture/reg-rs checks if d1 changes, then run sw-checklist before committing.

## Commit / complete protocol
Commit deliverables plus .agentrail/ with message:
minted: generalize syntax templates
Then push and agentrail complete with reward 1.