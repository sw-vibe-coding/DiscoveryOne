# DiscoveryOne M5 -- WASM Emit

Goal: emit WAT, assemble to WASM bytes, and run Power through d1 run so WASM outputs match the reference interpreter on the current corpus.

Reg-rs cases:
- d1_wasm_power_2_8_eq_256
- d1_wasm_power_5_0_eq_1
- d1_wasm_interp_parity_corpus

Every step must rebuild and reinstall d1 before fixture/reg-rs checks when d1 changes, keep sw-checklist clean with zero failures and zero warnings, commit .agentrail/ changes, and push.