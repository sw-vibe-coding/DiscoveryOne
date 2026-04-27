# Step: lex-comment-red

Create the next M1 RED baseline for comment lexing.

## Source-of-truth

- `docs/plan.md` M1 sequence: `d1_lex_comment` follows
  `d1_lex_zlayer_tags`.
- `docs/design.md` section 3 token enum: `Hash(&str)` for `#`
  comment text.
- Existing lexer fixture and reg-rs style under `tests/lexer/` and
  `work/reg-rs/`.

## Work

1. Add a focused lexer fixture for a `#` line comment.
2. Add `tests/lexer/comment.expected` documenting the intended future
   token dump.
3. Add reg-rs case `d1_lex_comment` that runs `d1 lex` on the new
   fixture and captures current RED output.
4. If the current lexer already supports the case, document that in
   `docs/status.md` and keep the green baseline.
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

`test: add comment lexer baseline`

Then push and `agentrail complete` with reward 1.
