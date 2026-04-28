# Step: wasm-power-2-8-green

Implement the minimal WASM run path that turns d1_wasm_power_2_8_eq_256 GREEN.

## Work
1. Wire d1 run examples/power.d1 --inputs n=2,e=8 through a minimal WAT/WASM emitter and runner path.
2. Use the planned M5 crates and dependencies conservatively, matching existing project patterns.
3. Preserve interpreter and IR baselines.
4. Rebase d1_wasm_power_2_8_eq_256 to GREEN.
5. Update docs/status.md.
6. Rebuild and reinstall d1 before fixture/reg-rs/checklist tests.
7. Run sw-checklist before committing.

## Commit / complete protocol
Commit deliverables plus .agentrail/ with message:
wasm: run power for 2 8
Then push and agentrail complete with reward 1.