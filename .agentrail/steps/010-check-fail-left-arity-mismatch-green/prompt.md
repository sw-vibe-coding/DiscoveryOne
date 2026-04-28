# Step: check-fail-left-arity-mismatch-green

Implement the minimal checker behavior that turns
`d1_check_fail_left_arity_mismatch` GREEN with stable error code
`E007`.

## Source-of-truth
- `docs/design.md` sections 6 and 13 for checker output and
  diagnostic shape.
- `docs/plan.md` M3 sequence.
- Existing `d1_check_pass_power` GREEN baseline.
- Existing `tests/check/left_arity_mismatch.d1` RED fixture.

## Work
1. Extend `d1-check` only enough to detect this fixture's
   front-input versus left-input arity mismatch.
2. Preserve `d1_check_pass_power` success output.
3. Rebase `d1_check_fail_left_arity_mismatch` from the generic
   scaffold error to a stable `E007` checker diagnostic.
4. Do not implement unrelated checker rules or additional
   negative cases in this step.
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
`check: report left arity mismatch`
Then push and `agentrail complete` with reward 1.
