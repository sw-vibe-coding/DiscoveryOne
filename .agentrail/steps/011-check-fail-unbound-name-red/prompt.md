# Step: check-fail-unbound-name-red

Create the next M3 RED baseline for an unbound-name checker
failure.

## Source-of-truth
- `docs/plan.md` M3 sequence.
- `docs/design.md` sections 6 and 13 for checker output and
  stable error codes.
- Existing `d1_check_pass_power` and
  `d1_check_fail_left_arity_mismatch` GREEN checker baselines.
- Existing checker scaffold in `d1-check`.

## Work
1. Add a small fixture that references an unbound name in a front
   facet expression.
2. Add reg-rs case `d1_check_fail_unbound_name` that runs
   `d1 check` on that fixture.
3. Capture the current RED output from the checker scaffold.
4. Do not implement unbound-name checker behavior in this step.
5. Update `docs/status.md`.
6. Run `sw-checklist` before committing. Any checklist issue is
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
`test: add unbound name check baseline`
Then push and `agentrail complete` with reward 1.
