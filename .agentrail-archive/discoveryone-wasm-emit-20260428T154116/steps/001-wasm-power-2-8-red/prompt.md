# Step: wasm-power-2-8-red

Create the first M5 RED baseline for running Power through the future WASM path.

## Work
1. Add reg-rs case d1_wasm_power_2_8_eq_256 for d1 run examples/power.d1 --inputs n=2,e=8, expecting 256.
2. Capture the current RED behavior without implementing d1 run.
3. Preserve all existing baselines.
4. Update docs/status.md.
5. Run sw-checklist before committing.

## Commit / complete protocol
Commit deliverables plus .agentrail/ with message:
wasm: add power run red baseline
Then push and agentrail complete with reward 1.