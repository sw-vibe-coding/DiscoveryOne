# Step: lex-aspect-tags-red

Create the next M1 RED baseline for aspect tag lexing.

## Source-of-truth

- `docs/plan.md` M1 sequence: `d1_lex_aspect_tags` follows
  `d1_lex_mint_operator`.
- `docs/design.md` lexer/token descriptions for face/aspect tags.
- Existing lexer baseline style in `tests/lexer/` and `work/reg-rs/`.

## Work

1. Add a focused lexer fixture for aspect tags, covering all primary
   face tags currently planned for M1:
   `@front`, `@left`, `@right`, `@top`, `@bottom`, `@rear`, and
   `@internal`.
2. Add reg-rs case `d1_lex_aspect_tags` that runs `d1 lex` on that
   fixture and captures the expected output.
3. Keep this as the RED/baseline step if possible. If the current lexer
   already supports the case, document that in `docs/status.md` and keep
   the green baseline.
4. Do not implement later M1 cases (`@z`, comments, negative integers,
   percent literals) in this step.
5. Run `sw-checklist` before committing. Any new checklist issue is a
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

Commit deliverables plus `.agentrail/` and `.agentrail-archive/` with
message:

`test: add aspect tag lexer baseline`

Then push and `agentrail complete` with reward 1.
