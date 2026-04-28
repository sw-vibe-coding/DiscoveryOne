# Step: source-loader-face-left-red

Create the next M2 RED baseline for projecting the Power source onto the
Left face.

## Source-of-truth

- `docs/plan.md` M2 sequence: add `d1 face --face F` baselines
  projecting Power.
- `docs/design.md` sections 1 and 2 for source format and projection.
- `examples/power.d1` and the existing source loader representation.

## Work

1. Add reg-rs case `d1_face_left_power` that runs
   `d1 face examples/power.d1 --face left`.
2. Capture the current RED output from the unsupported face path.
3. Do not implement left-face projection in this step.
4. Update `docs/status.md` to show left-face projection is the next RED
   M2 case.
5. Run `sw-checklist` before committing. Any checklist issue is a
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

`test: add left face projection baseline`

Then push and `agentrail complete` with reward 1.
