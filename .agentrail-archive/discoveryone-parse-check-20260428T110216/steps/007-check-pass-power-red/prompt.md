# Step: check-pass-power-red

Create the next M3 RED baseline for the first facet checker pass case.

## Source-of-truth
- `docs/plan.md` M3 sequence.
- `docs/design.md` section 6 for facet checker direction.
- Existing `examples/power.d1` source-loader fixture.
- Existing GREEN parser baselines.

## Work
1. Add reg-rs case `d1_check_pass_power` that runs `d1 check examples/power.d1`.
2. Capture the current RED output from the not-implemented check path.
3. Do not implement checker behavior in this step.
4. Update `docs/status.md`.
5. Run `sw-checklist` before committing. Any checklist issue is a blocker and must be fixed in this step.

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
- REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_check_pass_power
- cargo test
- cargo clippy --all-targets --all-features -- -D warnings
- cargo fmt -- --check
- markdown-checker -f "**/*.md"

## Commit / complete protocol
Commit deliverables plus `.agentrail/` with message:
`test: add power check pass baseline`
Then push and `agentrail complete` with reward 1.
