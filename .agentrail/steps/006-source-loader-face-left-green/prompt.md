# Step: source-loader-face-left-green

Implement the minimal left-face projection needed to make
`d1_face_left_power` green.

## Source-of-truth

- `docs/plan.md` M2 sequence.
- `docs/design.md` section 2 projection rules.
- `examples/power.d1` and the existing `d1-source` representation.
- Existing RED reg-rs case `d1_face_left_power`.

## Work

1. Add enough `d1-source` projection/rendering support to render the
   Left face of the Power fixture as a deterministic ASCII grid.
2. Wire `d1 face examples/power.d1 --face left` through the source
   loader.
3. Rebase `d1_face_left_power` to green.
4. Preserve `d1_normalize_roundtrip_power`, `d1_face_front_power`, and
   all M1 lexer baselines.
5. Update `docs/status.md`.
6. Run `sw-checklist` before committing. Any checklist issue is a
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
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_percent`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_normalize_roundtrip_power`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_face_front_power`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_face_left_power`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo fmt -- --check`
- `markdown-checker -f "**/*.md"`

## Commit / complete protocol

Commit deliverables plus `.agentrail/` with message:

`test: green left face projection`

Then push and `agentrail complete` with reward 1.
