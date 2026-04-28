# Step: parse-facet-blocks-red

Create the next M3 RED baseline for parsing multiple facet blocks for one definition.

## Source-of-truth
- `docs/plan.md` M3 sequence.
- `docs/design.md` section 4 for AST direction.
- Existing parser scaffold and GREEN `d1_parse_mint_init` / `d1_parse_mint_signature` baselines.
- Existing lexer support for aspect tags.

## Work
1. Add a small parser fixture with one definition and multiple `@`-tagged facet blocks.
2. Add reg-rs case `d1_parse_facet_blocks` that runs `d1 parse` on that fixture.
3. Capture the current RED output from the parser scaffold.
4. Do not implement facet-block parser behavior in this step.
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
`test: add facet blocks parse baseline`
Then push and `agentrail complete` with reward 1.
