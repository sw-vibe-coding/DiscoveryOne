# Step: ir-power-dump-red

Create the first M4 RED baseline for dumping stack IR for
`examples/power.d1`.

## Source-of-truth
- `docs/plan.md` section 6.
- `docs/design.md` sections 5, 10, and 11.
- Existing CLI fixture and reg-rs patterns.
- Existing `examples/power.d1`.

## Work
1. Add reg-rs case `d1_ir_power_dump` that runs the CLI IR/lower
   command for `examples/power.d1`.
2. Capture the current RED output from the CLI scaffold.
3. Do not implement IR lowering in this step.
4. Update `docs/status.md`.
5. Run `sw-checklist` before committing. Any checklist issue is
   a blocker and must be fixed in this step.

## Pre-commit gates
- sw-checklist
- full reg-rs suite
- cargo test
- cargo clippy --all-targets --all-features -- -D warnings
- cargo fmt -- --check
- markdown-checker -f "**/*.md"

## Commit / complete protocol
Commit deliverables plus `.agentrail/` with message:
`test: add power IR dump baseline`
Then push and `agentrail complete` with reward 1.
