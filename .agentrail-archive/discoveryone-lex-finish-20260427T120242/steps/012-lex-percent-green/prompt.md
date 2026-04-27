# Step: lex-percent-green

Implement the minimal M1 lexer support for percent literals.

## Source-of-truth

- `docs/plan.md` M1 sequence.
- `docs/design.md` section 3 token enum: `Pct(u8)` for `0..=100`.
- `tests/lexer/percent.input` and `tests/lexer/percent.expected`.
- Existing reg-rs case `d1_lex_percent` currently captures RED output.

## Work

1. Add `Token::Pct(u8)` support for `N%` where `N` is `0..=100`.
2. Dump percent literals exactly as `PCT    <N>`, matching
   `tests/lexer/percent.expected`.
3. Rebase `d1_lex_percent` to green.
4. Keep existing arity-suffix, mint-operator, aspect-tag, Z-layer,
   comment, and negative integer behavior unchanged.
5. Update `docs/status.md`. If this completes the M1 lexer corpus, note
   that M1 lexer cases are green.
6. Run `sw-checklist` before committing. Any new checklist issue is a
   blocker and must be fixed in this step.

## Pre-commit gates

- `sw-checklist`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_smoke_cli_help`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_arity_suffix`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_mint_operator`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_aspect_tags`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_zlayer_tags`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_comment`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_neg_int`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_percent`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo fmt -- --check`
- `markdown-checker -f "**/*.md"`

## Commit / complete protocol

Commit deliverables plus `.agentrail/` with message:

`test: green percent lexer`

Then push and `agentrail complete` with reward 1. If this finishes M1,
archive only when the user says to archive or after a final M1 completion
step.
