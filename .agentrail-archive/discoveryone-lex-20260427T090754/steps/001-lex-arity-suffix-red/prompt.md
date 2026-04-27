# Step: lex-arity-suffix-red

Create the first M1 RED baseline for the lexer.

## Source-of-truth

- `docs/plan.md` section 3, M1 TDD sequence item 1.
- `docs/design.md` section 3 for token names and lexical intent.
- Existing M0 reg-rs layout under `work/reg-rs/`.

## Work

1. Create `tests/lexer/arity_suffix.input` containing exactly:
   ```text
   coord2 -> (x y)
   ```
2. Create `tests/lexer/arity_suffix.expected` with the intended
   stable token dump for `d1 lex`. Use one token per line and
   keep it ASCII. This file documents the target behavior even
   though the CLI is still a stub.
3. Create reg-rs case `d1_lex_arity_suffix` using
   `REG_RS_DATA_DIR=work/reg-rs` and command
   `scripts/run-fixture.sh lex tests/lexer/arity_suffix.input`.
   Capture the current failing output as the RED baseline
   artifact if needed, but make it clear in `docs/status.md`
   that this step is red and the next step must implement
   `d1-lex` to replace/rebase the baseline to the expected token
   dump.
4. Run
   `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_arity_suffix`
   and confirm it demonstrates the missing implementation.
5. Run `cargo test`, `cargo clippy --all-targets --all-features
   -- -D warnings`, `cargo fmt -- --check`, and
   `markdown-checker -f "**/*.md"`. `sw-checklist` may still
   report the M0 metadata gaps documented in `docs/learnings.md`.

## Deliverables

- `tests/lexer/arity_suffix.input`
- `tests/lexer/arity_suffix.expected`
- `work/reg-rs/d1_lex_arity_suffix.rgt`
- `work/reg-rs/d1_lex_arity_suffix.out`
- `docs/status.md` updated to show M1 in flight and RED
  baseline captured

## Commit / complete protocol

Commit the deliverables plus `.agentrail/` with message:
`test: add red d1 lex arity suffix baseline`.

Then push and `agentrail complete` with reward 1.
