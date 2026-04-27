# Step: source-loader-roundtrip-power-red

Create the M2 RED baseline for layered source normalization.

## Source-of-truth

- `docs/plan.md` M2 sequence: `d1_normalize_roundtrip_power` starts
  `discoveryone-source-loader`.
- `docs/design.md` file-format and CLI sections.
- Existing fixture and reg-rs style under `tests/` and `work/reg-rs/`.

## Work

1. Add focused Power fixtures for the future source loader:
   `examples/power.d1` and `examples/power.d1.json`.
2. Add reg-rs case `d1_normalize_roundtrip_power` that runs the future
   normalize command and captures the current RED output.
3. If the current CLI already supports the case, document that in
   `docs/status.md` and keep the green baseline.
4. Do not implement the source loader or normalize command in this step.
5. Update `docs/status.md` to show M2 has started with this RED baseline.
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
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo fmt -- --check`
- `markdown-checker -f "**/*.md"`

## Commit / complete protocol

Commit deliverables plus `.agentrail/` with message:

`test: add source normalize red baseline`

Then push and `agentrail complete` with reward 1.
