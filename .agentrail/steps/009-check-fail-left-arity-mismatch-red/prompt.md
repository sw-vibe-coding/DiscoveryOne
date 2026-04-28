# Step: check-fail-left-arity-mismatch-red

Create the next M3 RED baseline for a left-facet arity mismatch checker failure.

## Source-of-truth
- `docs/plan.md` M3 sequence.
- `docs/design.md` section 6 for checker output and stable error codes.
- Existing `d1_check_pass_power` GREEN checker scaffold.
- Existing source-loader and parser fixtures.

## Work
1. Add a small fixture that intentionally mismatches the left facet input arity.
2. Add reg-rs case `d1_check_fail_left_arity_mismatch` that runs `d1 check` on that fixture.
3. Capture the current RED output from the checker scaffold.
4. Do not implement negative checker behavior in this step.
5. Update `docs/status.md`.
6. Run `sw-checklist` before committing. Any checklist issue is a blocker and must be fixed in this step.

## Pre-commit gates
- sw-checklist
- all prior gates plus d1_check_pass_power and d1_check_fail_left_arity_mismatch
- cargo test
- cargo clippy --all-targets --all-features -- -D warnings
- cargo fmt -- --check
- markdown-checker -f "**/*.md"

## Commit / complete protocol
Commit deliverables plus `.agentrail/` with message:
`test: add left arity mismatch check baseline`
Then push and `agentrail complete` with reward 1.
