# DiscoveryOne M4 -- IR / Interpreter

Goal: lower checked AST to stack IR; build a reference Rust
interpreter; produce expected outputs that the WASM emitter must
match.

Reg-rs cases:
- `d1_ir_power_dump`
- `d1_interp_power_2_8_eq_256`
- `d1_interp_power_5_0_eq_1`

Every step must keep `sw-checklist` clean with zero failures and
zero warnings, commit `.agentrail/` changes, and push.
