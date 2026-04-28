# Step: parse-facet-blocks-green

Implement the minimal parser scaffold needed to make `d1_parse_facet_blocks` green.

## Source-of-truth
- `docs/plan.md` M3 sequence.
- `docs/design.md` section 4 for AST direction.
- `tests/parser/facet_blocks.input` and existing RED baseline `d1_parse_facet_blocks`.
- Existing GREEN parser baselines for mint init and mint signature.
- Existing lexer support for aspect tags.

## Work
1. Extend the parser AST/dump shape just enough to represent multiple `@`-tagged facet blocks for one definition.
2. Parse `tests/parser/facet_blocks.input` into a deterministic AST dump.
3. Rebase `d1_parse_facet_blocks` to GREEN.
4. Preserve `d1_parse_mint_init`, `d1_parse_mint_signature`, and all M1/M2 baselines.
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
- REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_parse_mint_signature
- REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_parse_facet_blocks
- cargo test
- cargo clippy --all-targets --all-features -- -D warnings
- cargo fmt -- --check
- markdown-checker -f "**/*.md"

## Commit / complete protocol
Commit deliverables plus `.agentrail/` with message:
`test: green facet blocks parse baseline`
Then push and `agentrail complete` with reward 1.
