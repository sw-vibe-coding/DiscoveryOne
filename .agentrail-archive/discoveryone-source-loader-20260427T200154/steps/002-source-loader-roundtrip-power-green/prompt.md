# Step: source-loader-roundtrip-power-green

Implement the minimal M2 source loader and dumper needed to make
`d1_normalize_roundtrip_power` green.

## Source-of-truth

- `docs/plan.md` M2 sequence.
- `docs/design.md` section 1 source file formats.
- `examples/power.d1` and `examples/power.d1.json`.
- Existing RED reg-rs case `d1_normalize_roundtrip_power`.

## Work

1. Implement enough `d1-source` support to load the Power layered text
   fixture into a deterministic positioned-symbol representation.
2. Implement enough dumping/normalization for `d1 normalize
   examples/power.d1` to emit the canonical layered text exactly as
   `examples/power.d1`.
3. Wire the `d1 normalize` CLI command through the source loader.
4. Rebase `d1_normalize_roundtrip_power` to green.
5. Preserve all M1 lexer baselines.
6. Update `docs/status.md`.
7. Run `sw-checklist` before committing. Any checklist issue is a
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
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo fmt -- --check`
- `markdown-checker -f "**/*.md"`

## Commit / complete protocol

Commit deliverables plus `.agentrail/` with message:

`test: green source normalize roundtrip`

Then push and `agentrail complete` with reward 1.
