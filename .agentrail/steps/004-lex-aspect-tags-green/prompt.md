# Step: lex-aspect-tags-green

Implement the minimal M1 lexer support for primary aspect tags.

## Source-of-truth

- `docs/plan.md` M1 sequence.
- `docs/design.md` section 3 token enum:
  `AspectTag(AspectKind)` for `@front`, `@left`, `@right`, `@top`,
  `@bottom`, `@rear`, and `@internal`.
- `tests/lexer/aspect_tags.input` and
  `tests/lexer/aspect_tags.expected`.
- Existing reg-rs case `d1_lex_aspect_tags` currently captures RED
  output.

## Work

1. Add an `AspectKind` enum covering `front`, `left`, `right`, `top`,
   `bottom`, `rear`, and `internal`.
2. Update `d1-lex` so those `@...` spellings lex as
   `Token::AspectTag`.
3. Dump aspect tags exactly as `ASPECT <name>`, matching
   `tests/lexer/aspect_tags.expected`.
4. Rebase `d1_lex_aspect_tags` to green.
5. Keep existing arity-suffix and mint-operator behavior unchanged.
6. Do not implement `@z`, comments, negative integers, or percent
   literals in this step.
7. Run `sw-checklist` before committing. Any new checklist issue is a
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

`test: green aspect tag lexer`

Then push and `agentrail complete` with reward 1.
