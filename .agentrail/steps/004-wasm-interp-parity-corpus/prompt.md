# Step: wasm-interp-parity-corpus

Add a corpus parity watchdog that compares d1 interp and d1 run outputs for every current interpreter case.

## Work
1. Add reg-rs case d1_wasm_interp_parity_corpus.
2. Implement a deterministic parity harness using the existing fixture commands or a small project script, keeping scope to current Power cases.
3. Preserve all existing baselines.
4. Update docs/status.md.
5. Rebuild and reinstall d1 before fixture/reg-rs/checklist tests if d1 changes.
6. Run sw-checklist before committing.

## Commit / complete protocol
Commit deliverables plus .agentrail/ with message:
wasm: compare run and interp corpus
Then push and agentrail complete --done with reward 1.