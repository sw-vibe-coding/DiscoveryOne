# Step: lex-comment-green

Implement the minimal M1 lexer support for `#` line comments.

## Source-of-truth

- `docs/plan.md` M1 sequence.
- `docs/design.md` section 3 token enum: `Hash(&str)` for `#`
  comment text.
- `tests/lexer/comment.input` and `tests/lexer/comment.expected`.
- Existing reg-rs case `d1_lex_comment` currently captures RED output.

## Work

1. Add `Token::Hash(&str)` support for `#` through end-of-line.
2. Dump comments exactly as `HASH   <comment>`, matching
   `tests/lexer/comment.expected`.
3. Rebase `d1_lex_comment` to green.
4. Keep existing arity-suffix, mint-operator, aspect-tag, and Z-layer
   behavior unchanged.
5. Do not implement negative integers or percent literals in this step.
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
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo fmt -- --check`
- `markdown-checker -f "**/*.md"`

## Commit / complete protocol

Commit deliverables plus `.agentrail/` with message:

`test: green comment lexer`

Then push and `agentrail complete` with reward 1.
