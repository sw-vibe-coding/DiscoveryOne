# Step: unless-minted-syntax-red

Create the M7 RED substitution baseline proving minted syntax is general beyond dowhile.

## Work
1. Add an unless/do fixture using a different two-slot template that exercises the same expansion mechanism without code changes.
2. Add reg-rs case d1_poc_unless_minted_syntax expecting deterministic CLI output and failing deterministically until support is generalized.
3. Preserve all existing baselines.
4. Update docs/status.md.
5. Run sw-checklist before committing.

## Commit / complete protocol
Commit deliverables plus .agentrail/ with message:
minted: add unless red baseline
Then push and agentrail complete with reward 1.