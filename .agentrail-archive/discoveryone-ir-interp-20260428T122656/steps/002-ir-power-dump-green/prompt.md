# Step: ir-power-dump-green

Implement the minimal IR dump scaffold that turns
`d1_ir_power_dump` GREEN for `examples/power.d1`.

## Work
1. Wire the CLI IR/lower command to `d1-ir` for Power.
2. Produce deterministic stack-IR text for the current Power
   fixture.
3. Preserve all existing lex, normalize, face, parse, and check
   baselines.
4. Rebase `d1_ir_power_dump`.
5. Update `docs/status.md`.
6. Run `sw-checklist` before committing.

## Commit / complete protocol
Commit deliverables plus `.agentrail/` with message:
`ir: dump power stack IR`
Then push and `agentrail complete` with reward 1.
