# Step: lex-token-display-refactor

Refactor the first lexer implementation without changing behavior.

## Source-of-truth

- `docs/plan.md` section 3, M1 TDD sequence item 3.
- `docs/status.md` current M1 note.
- `tests/lexer/arity_suffix.expected` and
  `work/reg-rs/d1_lex_arity_suffix.*` as behavior locks.

## Work

1. Keep `d1_lex_arity_suffix` green.
2. Extract or reshape token display / lexing helpers so
   `d1-lex` is easier to extend for the next M1 case.
3. Eliminate the current `sw-checklist` warning for `d1-lex`
   function length if practical without broadening behavior.
4. Do not add mint/operator behavior in this step; that belongs
   to the next RED/GREEN cycle.
5. Update `docs/status.md` to say the arity-suffix implementation
   was refactored and the next step is the mint-operator RED
   baseline.

## Pre-commit gates

- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_arity_suffix`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_smoke_cli_help`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo fmt -- --check`
- `markdown-checker -f "**/*.md"`
- `sw-checklist` if installed; known project metadata failures
  are documented in `docs/learnings.md`.

## Deliverables

- Refactored `crates/d1-lex/src/lib.rs`
- Any needed test updates under `crates/d1-lex/tests/`
- Updated `docs/status.md`

## Commit / complete protocol

Commit the deliverables plus `.agentrail/` with message:
`refactor: clean up d1 lexer token display`.

Then push and `agentrail complete` with reward 1.
