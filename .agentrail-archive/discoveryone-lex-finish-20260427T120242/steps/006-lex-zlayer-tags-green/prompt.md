# Step: lex-zlayer-tags-green

Implement the minimal M1 lexer support for Z-layer tags.

## Source-of-truth

- `docs/plan.md` M1 sequence.
- `docs/design.md` section 3 token enum: `ZTag(i32)` for `@z N`.
- `tests/lexer/zlayer_tags.input` and
  `tests/lexer/zlayer_tags.expected`.
- Existing reg-rs case `d1_lex_zlayer_tags` currently captures RED
  output.

## Work

1. Add `Token::ZTag(i32)` support for `@z N`.
2. Dump Z-layer tags exactly as `ZTAG   <N>`, matching
   `tests/lexer/zlayer_tags.expected`.
3. Rebase `d1_lex_zlayer_tags` to green.
4. Keep existing arity-suffix, mint-operator, and aspect-tag behavior
   unchanged.
5. Do not implement comments, negative integers, or percent literals in
   this step.
6. Run `sw-checklist` before committing. Any new checklist issue is a
   blocker and must be fixed in this step.

## Pre-commit gates

- `sw-checklist`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_smoke_cli_help`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_arity_suffix`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_mint_operator`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_aspect_tags`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_zlayer_tags`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo fmt -- --check`
- `markdown-checker -f "**/*.md"`

## Commit / complete protocol

Commit deliverables plus `.agentrail/` with message:

`test: green z-layer tag lexer`

Then push and `agentrail complete` with reward 1.
