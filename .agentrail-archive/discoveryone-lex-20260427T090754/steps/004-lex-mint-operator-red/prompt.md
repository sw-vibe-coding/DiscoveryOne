# Step: lex-mint-operator-red

Create the M1 RED baseline for the mint operator lexer case.

## Source-of-truth

- `docs/plan.md` section 3, M1 TDD sequence.
- `docs/design.md` section 3 for `Mint` token intent.
- Existing lexer fixture/baseline pattern:
  `tests/lexer/arity_suffix.*` and
  `work/reg-rs/d1_lex_arity_suffix.*`.

## Work

1. Create `tests/lexer/mint_operator.input` containing a small
   mint declaration using `*`, for example:
   ```text
   *coord2 -> (x y)
   ```
2. Create `tests/lexer/mint_operator.expected` with the intended
   stable token dump. It should begin with `MINT`, then match
   the arity-suffix token sequence.
3. Create reg-rs case `d1_lex_mint_operator` using
   `REG_RS_DATA_DIR=work/reg-rs` and command
   `scripts/run-fixture.sh lex tests/lexer/mint_operator.input`.
   This is a RED step: capture the current failing/mismatching
   behavior and make the gap explicit in `docs/status.md`.
4. Run `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p
   d1_lex_mint_operator` and confirm it records the current RED
   state.
5. Keep the existing `d1_lex_arity_suffix` baseline green.

## Pre-commit gates

- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_arity_suffix`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_mint_operator`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo fmt -- --check`
- `markdown-checker -f "**/*.md"`
- `sw-checklist` if installed; known project metadata failures
  are documented in `docs/learnings.md`.

## Deliverables

- `tests/lexer/mint_operator.input`
- `tests/lexer/mint_operator.expected`
- `work/reg-rs/d1_lex_mint_operator.rgt`
- `work/reg-rs/d1_lex_mint_operator.out`
- `work/reg-rs/d1_lex_mint_operator.err` if stderr is part of
  the RED baseline
- Updated `docs/status.md`

## Commit / complete protocol

Commit the deliverables plus `.agentrail/` with message:
`test: add red d1 lex mint operator baseline`.

Then push and `agentrail complete` with reward 1.
