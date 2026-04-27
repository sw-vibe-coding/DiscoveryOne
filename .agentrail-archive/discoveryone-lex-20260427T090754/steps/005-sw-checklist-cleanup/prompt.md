# Step: sw-checklist-cleanup

Pay down the Software Wrighter checklist failures before more
feature work.

## Source-of-truth

- User instruction: fix `sw-checklist` issues as soon as they
  are reported; do not accumulate tech debt.
- `sw-checklist --verbose` output.
- Existing M0/M1 behavior baselines.

## Work

1. Fix current `sw-checklist` failures where practical:
   favicon file for the web app, CLI long help, CLI AI-agent
   instructions, version metadata, and web metadata warnings.
2. Keep existing CLI behavior baselines green. If help output
   intentionally changes, update/rebase `d1_smoke_cli_help`.
3. Run `sw-install --project` if needed for binary freshness.
4. Update `docs/status.md` and `docs/learnings.md` so the
   cleanup policy is explicit.
5. Do not continue M1 lexer feature work in this step.

## Pre-commit gates

- `sw-checklist`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_smoke_cli_help`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_arity_suffix`
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_mint_operator`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo fmt -- --check`
- `markdown-checker -f "**/*.md"`

## Commit / complete protocol

Commit deliverables plus `.agentrail/` with message:
`chore: fix sw-checklist metadata`.

Then push and `agentrail complete` with reward 1.
