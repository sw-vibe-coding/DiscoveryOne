# Step: reg-rs-smoke-baseline

Capture the first reg-rs baseline (`d1_smoke_cli_help`), prove
the full pre-commit gate is green, mark the saga complete with
`--done`, and update `docs/status.md` so the next session knows
M1 is the next saga to start.

## Source-of-truth

- `docs/plan.md` section 2 -- M0 exit criteria.
- `docs/design.md` section 11 -- reg-rs corpus naming.
- `../tuplet/work/reg-rs/tuplet_build_skeleton.rgt` -- baseline
  format reference.

## Work

1. Create `work/reg-rs/d1_smoke_cli_help.rgt`:
   ```toml
   command = "/Users/<user>/github/sw-vibe-coding/DiscoveryOne/scripts/run-fixture.sh --help"
   timeout = 60
   exit_code = 0
   desc = "d1 smoke: --help banner is stable"
   ```
   Use `$PWD`-relative paths if reg-rs supports them; otherwise
   absolute paths matching the existing tuplet pattern.
2. Run:
   ```bash
   REG_RS_DATA_DIR=work/reg-rs reg-rs create -t d1_smoke_cli_help \
     -c 'scripts/run-fixture.sh --help'
   ```
   This captures the baseline `.out` from the current `d1
   --help` output.
3. Verify reproducibility:
   ```bash
   REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_smoke_cli_help
   ```
   Expect a clean pass.
4. Confirm `.gitignore` keeps `*.tdb` and `*.lock` out; verify
   only `.rgt` and `.out` are tracked.
5. Run the **complete** pre-commit gate (`docs/process.md`
   section "Pre-Commit Sequence", steps 1-7):
   - `cargo test`
   - `cargo clippy --all-targets --all-features -- -D warnings`
   - `cargo fmt --all` then `cargo fmt -- --check`
   - `markdown-checker -f "**/*.md"`
   - `git status` (no surprises)
   - `sw-checklist` (if installed; document any failures)
6. Update `docs/status.md`:
   - Move M0 from "Up next" to "Done".
   - List the deliverables shipped.
   - Note that M1 (`discoveryone-lex`) is the next saga to
     `agentrail init` after archiving this one.

## Pre-commit gates

- All gates from `docs/process.md`. No exceptions.

## Deliverables

- `work/reg-rs/d1_smoke_cli_help.rgt` (tracked).
- `work/reg-rs/d1_smoke_cli_help.out` (tracked).
- Updated `docs/status.md`.

## Commit / complete protocol

```bash
git add work/reg-rs/d1_smoke_cli_help.rgt \
        work/reg-rs/d1_smoke_cli_help.out \
        docs/status.md \
        .agentrail/
git commit -m "feat: d1_smoke_cli_help reg-rs baseline; close M0"
git push
agentrail complete \
  --summary "M0 closed: workspace builds, smoke baseline green, status updated" \
  --reward 1 \
  --actions "reg-rs create, pre-commit gate, status update" \
  --done
```

`--done` marks the saga complete in `saga.toml`. **After**
`agentrail complete --done`, run:

```bash
agentrail archive --reason "M0 scaffold complete; M1 (discoveryone-lex) is next"
git add .agentrail/ .agentrail-archive/
git commit -m "chore: archive discoveryone-scaffold saga"
git push
```

This moves the saga record into `.agentrail-archive/` and
commits both the cleared `.agentrail/` and the populated
`.agentrail-archive/` so the history survives in git.

## Risks

- `d1 --help` output drifting between machines: the baseline is
  captured on whatever box runs reg-rs first. If the help
  output contains absolute paths or timestamps, it won't be
  portable. The `clap` `about` string is a static `&'static
  str` so this should be safe; verify by reading the captured
  `.out` before committing.
- `sw-checklist` may flag missing CHANGES.md or similar.
  Document the failure in `docs/learnings.md` (per
  `docs/process.md` step 7) but do not block the saga on
  cosmetics; checklist warnings can be addressed in a follow-up
  housekeeping step if non-trivial.
