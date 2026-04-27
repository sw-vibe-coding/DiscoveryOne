# Step: lex-zlayer-tags-red

Create the next M1 RED baseline for Z-layer tag lexing.

## Source-of-truth

- `docs/plan.md` M1 sequence: `d1_lex_zlayer_tags` follows
  `d1_lex_aspect_tags`.
- `docs/design.md` section 3 token enum: `ZTag(i32)` for `@z N`.
- Existing lexer fixture and reg-rs style under `tests/lexer/` and
  `work/reg-rs/`.

## Work

1. Add a focused lexer fixture for `@z N`, covering at least `@z 0`
   and a non-zero positive layer.
2. Add `tests/lexer/zlayer_tags.expected` documenting the intended
   future token dump.
3. Add reg-rs case `d1_lex_zlayer_tags` that runs `d1 lex` on the new
   fixture and captures current RED output.
4. If the current lexer already supports the case, document that in
   `docs/status.md` and keep the green baseline.
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

`test: add z-layer tag lexer baseline`

Then push and `agentrail complete` with reward 1.
