# Step: wasm-power-5-0

Add the second M5 WASM baseline for Power with n=5,e=0, expecting 1, and make it GREEN with the existing WASM run path.

## Work
1. Add reg-rs case d1_wasm_power_5_0_eq_1.
2. Extend the minimal WASM path only as needed.
3. Preserve all existing baselines.
4. Update docs/status.md.
5. Rebuild and reinstall d1 before fixture/reg-rs/checklist tests.
6. Run sw-checklist before committing.

## Commit / complete protocol
Commit deliverables plus .agentrail/ with message:
wasm: run power for zero exponent
Then push and agentrail complete with reward 1.