# Step: build-and-test-scripts

Add the small shell helpers every later saga will rely on, plus
the `.gitignore` that keeps transient build output out of git
while explicitly tracking `.agentrail/`.

## Source-of-truth

- `docs/design.md` section 12 (build pipeline).
- `~/github/sw-embed/web-sw-cor24-ocaml/scripts/{serve.sh,
  build-pages.sh,vendor-artifacts.sh}` -- mirror the pattern,
  not the implementation.

## Work

1. `scripts/build-all.sh`. Builds the CLI release binary, then
   `trunk build --release` for `ui/web/`. Uses `set -euo
   pipefail`. Quotes all variables.
2. `scripts/run-web.sh PORT`. Starts `trunk serve --address
   127.0.0.1 --port "${1:-9735}"` from `ui/web/`. The dev
   server pattern from the reference repo includes an
   exclusive lock on `dist/` to prevent two devs racing -- adopt
   that.
3. `scripts/run-fixture.sh ARGS...`. One-line wrapper that
   invokes the locally-built `d1` binary
   (`target/release/d1`) with the given arguments. Used by
   reg-rs `*.rgt` `command = ` lines.
4. `chmod +x scripts/*.sh`.
5. Create `.gitignore`:
   - Ignore: `target/`, `dist/`, `ui/web/dist/`, `ui/web/pkg/`,
     `**/*.tdb`, `**/*.tdb.lock`, `.DS_Store`,
     `work/generated/`.
   - **Explicitly track** `.agentrail/` and
     `.agentrail-archive/` -- include a comment block in the
     `.gitignore` that says these are tracked-source-of-record
     and must NEVER be added to the ignore list, citing
     `CLAUDE.md` "Rules for `.agentrail/`".
6. Run the existing reg-rs smoke (none yet) plus the cargo
   gate. Confirm `git status` shows only the new scripts and
   `.gitignore`.

## Pre-commit gates

- `bash -n scripts/*.sh` (syntax check) clean.
- `shellcheck scripts/*.sh` clean if shellcheck is installed.
- `cargo test`, clippy, fmt all green.
- `markdown-checker -f "**/*.md"` clean.

## Deliverables

- `scripts/build-all.sh`, `scripts/run-web.sh`,
  `scripts/run-fixture.sh` (executable).
- `.gitignore` with the policy above.

## Commit / complete protocol

```bash
git add scripts/ .gitignore .agentrail/
git commit -m "feat: build/run scripts and gitignore policy"
git push
agentrail complete \
  --summary "build-all/run-web/run-fixture scripts; .agentrail tracked" \
  --reward 1 \
  --actions "bash scripts, gitignore policy"
```

## Risks

- `.gitignore` accidentally listing `.agentrail/`. The comment
  in the file is there to scream STOP at the next agent. Verify
  with `git check-ignore -v .agentrail/saga.toml` -- it must
  NOT report a match.
- `trunk` not installed on the host: `scripts/run-web.sh` should
  fail loudly with a hint to `cargo install trunk`, not silently.
