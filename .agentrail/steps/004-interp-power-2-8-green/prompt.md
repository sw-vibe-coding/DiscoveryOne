# Step: interp-power-2-8-green

Implement the minimal reference interpreter path that turns
`d1_interp_power_2_8_eq_256` GREEN.

## Work
1. Wire `d1 interp examples/power.d1 --inputs n=2 e=8` through
   the interpreter scaffold.
2. Preserve `d1_ir_power_dump` and all earlier baselines.
3. Rebase `d1_interp_power_2_8_eq_256`.
4. Update `docs/status.md`.
5. Run `sw-checklist` before committing.

## Commit / complete protocol
Commit deliverables plus `.agentrail/` with message:
`interp: run power for 2 8`
Then push and `agentrail complete` with reward 1.
