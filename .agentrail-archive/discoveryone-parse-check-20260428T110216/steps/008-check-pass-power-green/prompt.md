# Step: check-pass-power-green

Implement the minimal checker scaffold needed to make `d1_check_pass_power` green.

## Source-of-truth
- `docs/plan.md` M3 sequence.
- `docs/design.md` section 6 for facet checker direction.
- Existing `examples/power.d1` source-loader fixture.
- Existing RED baseline `d1_check_pass_power`.

## Work
1. Wire `d1 check examples/power.d1` through `d1-check` with a deterministic success output.
2. Keep the checker scaffold minimal; do not implement negative checker cases in this step.
3. Rebase `d1_check_pass_power` to GREEN.
4. Preserve all parser, lexer, normalize, and face baselines.
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
- REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_check_pass_power
- cargo test
- cargo clippy --all-targets --all-features -- -D warnings
- cargo fmt -- --check
- markdown-checker -f "**/*.md"

## Commit / complete protocol
Commit deliverables plus `.agentrail/` with message:
`test: green power check pass baseline`
Then push and `agentrail complete` with reward 1.
