# Step: interp-power-5-0

Add the second M4 interpreter baseline for Power with `n=5,
e=0`, expecting `1`, and make it GREEN with the existing
interpreter path.

## Work
1. Add reg-rs case `d1_interp_power_5_0_eq_1`.
2. Extend the minimal interpreter path only as needed.
3. Preserve all existing baselines.
4. Update `docs/status.md`.
5. Run `sw-checklist` before committing.

## Commit / complete protocol
Commit deliverables plus `.agentrail/` with message:
`interp: run power for zero exponent`
Then push and `agentrail complete --done` with reward 1.
