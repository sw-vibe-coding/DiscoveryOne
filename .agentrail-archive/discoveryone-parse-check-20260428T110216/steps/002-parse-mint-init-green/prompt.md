# Step: parse-mint-init-green

Implement the minimal parser scaffold needed to make `d1_parse_mint_init` green.

## Source-of-truth
- `docs/plan.md` M3 sequence.
- `docs/design.md` section 4 for AST direction.
- `tests/parser/mint_init.input` and existing RED baseline `d1_parse_mint_init`.
- Existing lexer output from `d1-lex`.

## Work
1. Add a minimal AST representation in `d1-parse` for the mint/assignment form in `*n <- 0`.
2. Wire `d1 parse tests/parser/mint_init.input` through `d1-cli`.
3. Rebase `d1_parse_mint_init` to the deterministic AST dump.
4. Preserve all M1 lexer and M2 source-loader baselines.
5. Update `docs/status.md`.
6. Run `sw-checklist` before committing. Any checklist issue is a blocker and must be fixed in this step.

## Pre-commit gates
- sw-checklist
- REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_smoke_cli_help
- REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_arity_suffix
- REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_mint_operator
- REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_aspect_tags
- REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_zlayer_tags
- REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_comment
- REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_neg_int
- REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_percent
- REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_normalize_roundtrip_power
- REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_face_front_power
- REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_face_left_power
- REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_face_right_power
- REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_face_top_power
- REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_face_bottom_power
- REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_face_rear_power
- REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_parse_mint_init
- cargo test
- cargo clippy --all-targets --all-features -- -D warnings
- cargo fmt -- --check
- markdown-checker -f "**/*.md"

## Commit / complete protocol
Commit deliverables plus `.agentrail/` with message:
`test: green mint init parse baseline`
Then push and `agentrail complete` with reward 1.
