# Step: lex-neg-int-green

Implement the minimal M1 lexer support for negative integers.

## Source-of-truth

- `docs/plan.md` M1 sequence.
- `docs/design.md` section 3 token enum: `Int(i64)` and `Minus`.
- `tests/lexer/neg_int.input` and `tests/lexer/neg_int.expected`.
- Existing reg-rs case `d1_lex_neg_int` currently captures RED output.

## Work

1. Add integer token support sufficient for `-42` to dump as
   `INT    -42`.
2. Rebase `d1_lex_neg_int` to green.
3. Keep existing arity-suffix, mint-operator, aspect-tag, Z-layer, and
   comment behavior unchanged.
4. Do not implement percent literals in this step.
5. Run `sw-checklist` before committing. Any new checklist issue is a
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
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo fmt -- --check`
- `markdown-checker -f "**/*.md"`

## Commit / complete protocol

Commit deliverables plus `.agentrail/` with message:

`test: green negative integer lexer`

Then push and `agentrail complete` with reward 1.
