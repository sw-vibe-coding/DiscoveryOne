# Step: lex-mint-operator-green

Implement the minimal M1 lexer support for the mint operator.

## Source-of-truth

- `docs/plan.md` M1 sequence.
- `docs/design.md` section 3 token enum: Mint is the ASCII `*`
  token.
- `tests/lexer/mint_operator.input` and
  `tests/lexer/mint_operator.expected`.
- Existing reg-rs case `d1_lex_mint_operator` currently captures RED
  output.

## Work

1. Update `d1-lex` so `*` lexes as `Token::Mint` and dumps as
   `MINT`.
2. Keep existing arity-suffix behavior unchanged.
3. Rebase `d1_lex_mint_operator` to the intended green output.
4. Update `docs/status.md` for the new M1 state.
5. Do not implement aspect tags, `@z` tags, comments, negative
   integers, or percent literals in this step.
6. Run `sw-checklist` before committing. Any new checklist issue is a
   blocker and must be fixed in this step.

## Pre-commit gates

- `sw-checklist`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_smoke_cli_help`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_arity_suffix`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_mint_operator`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_aspect_tags`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo fmt -- --check`
- `markdown-checker -f "**/*.md"`

## Commit / complete protocol

Commit deliverables plus `.agentrail/` with message:

`test: green mint operator lexer`

Then push and `agentrail complete` with reward 1.
