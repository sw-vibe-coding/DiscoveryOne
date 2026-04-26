# Step: cargo-workspace-and-cli-stub

Lay down the Cargo workspace and the empty crate skeletons that
later sagas will fill in. End the step with a stable `d1 --help`
banner that the next step's reg-rs baseline can lock in.

## Source-of-truth

- `docs/architecture.md` section 5 (crate layout).
- `docs/design.md` section 9 (CLI surface).

## Work

1. Create `Cargo.toml` at the repo root declaring a workspace
   with members `crates/d1-source`, `crates/d1-geom`,
   `crates/d1-lex`, `crates/d1-parse`, `crates/d1-check`,
   `crates/d1-ir`, `crates/d1-interp`, `crates/d1-emit-wasm`,
   `crates/d1-cli`. Pin `edition = "2024"` workspace-wide.
2. Each library crate gets `crates/<name>/Cargo.toml`
   (`name = "<name>"`, `edition.workspace = true`),
   `src/lib.rs` with one `pub fn version() -> &'static str { "0.1.0" }`
   and a `#[test] fn version_is_set()` that asserts non-empty.
3. `crates/d1-cli` is a `[[bin]]` crate with a `clap`-driven
   subcommand surface stubbed out per `docs/design.md`
   section 9. Only `--help` produces a stable banner this saga;
   subcommands print `not yet implemented` to stderr and exit 1.
4. Banner text must be deterministic (no version date / git
   sha) so the next step's reg-rs baseline is stable. Use a
   fixed `about` string and `clap`'s default `--help` rendering.

## Pre-commit gates

- `cargo build` clean.
- `cargo test` clean.
- `cargo clippy --all-targets --all-features -- -D warnings`
  clean.
- `cargo fmt -- --check` clean.

## Deliverables

- `Cargo.toml` (workspace).
- `crates/d1-{source,geom,lex,parse,check,ir,interp,emit-wasm}/`
  with `Cargo.toml` and `src/lib.rs`.
- `crates/d1-cli/Cargo.toml` and `crates/d1-cli/src/main.rs`.
- `Cargo.lock` (committed; binary crate convention).

## Commit / complete protocol

```bash
git add Cargo.toml Cargo.lock crates/ .agentrail/
git commit -m "feat: scaffold d1 workspace and cli stub"
git push
agentrail complete \
  --summary "Workspace builds; d1 --help banner stable" \
  --reward 1 \
  --actions "cargo workspace, clap stub, lib crate skeletons"
```

`.agentrail/` MUST be in the `git add`. Per CLAUDE.md, the
`commits` field of the step record is captured by
`agentrail complete`; if `.agentrail/` is missing from the
commit, the linkage breaks.

## Risks

- Clap version pinning: pin to a specific minor (e.g. `4.5`)
  to keep `--help` output stable across future dependency
  updates.
- Default banner ordering: `clap` sorts subcommands; keep the
  surface fixed for the rest of the saga to avoid baseline
  churn.
