# Saga: discoveryone-scaffold

## Goal

Lay down the minimum DiscoveryOne project skeleton so every later
saga has a workspace to land in, a CLI to invoke, a Yew app to
extend, and a reg-rs harness to baseline against. End state is
the M0 milestone in `docs/plan.md` section 2: workspace builds
clean, one reg-rs smoke test (`d1_smoke_cli_help`) is green, and
the full pre-commit gate (`cargo test`, `cargo clippy
--all-targets --all-features -- -D warnings`, `cargo fmt --
--check`, `markdown-checker -f "**/*.md"`) passes.

## Source of truth

- `docs/architecture.md` -- crate layout (section 5), UI
  architecture (section 6), test harness (section 11).
- `docs/design.md` -- CLI surface (section 9), web components
  (section 10), build pipeline (section 12).
- `docs/plan.md` -- M0 entrance / exit criteria (section 2).
- `docs/process.md` -- TDD red/green/refactor and pre-commit
  gates.
- `~/github/sw-embed/web-sw-cor24-ocaml` -- Yew + Trunk + COR24
  pattern; `Trunk.toml`, `scripts/serve.sh`,
  `scripts/build-pages.sh`, `index.html`, `dist/` lock are
  worth mirroring.

## In scope

- Cargo workspace at the repository root.
- Empty library crates: `d1-source`, `d1-geom`, `d1-lex`,
  `d1-parse`, `d1-check`, `d1-ir`, `d1-interp`, `d1-emit-wasm`.
  Each is `lib`-only with one `pub fn version() -> &'static str`
  placeholder so the workspace builds.
- `d1-cli` binary crate. Subcommand surface from `docs/design.md`
  section 9 is **stubbed**; only `d1 --help` produces a stable
  banner this saga. Other subcommands print `not yet implemented`
  and exit 1.
- `ui/web` Yew SPA crate built via Trunk. Renders one page with
  the project name, build stamp, and a placeholder where the
  facet view will live.
- `scripts/build-all.sh`, `scripts/run-web.sh PORT`,
  `scripts/run-fixture.sh` per `docs/design.md` section 12.
- `work/reg-rs/d1_smoke_cli_help.rgt` and `.out` -- the first
  reg-rs baseline; runs `d1 --help` and matches the banner.
- `.gitignore` that excludes `target/`, `dist/`, `ui/web/dist/`,
  `ui/web/pkg/`, `*.tdb`, `*.lock`, `.DS_Store` -- and that
  **explicitly tracks** `.agentrail/` and `.agentrail-archive/`
  (with a comment explaining why).
- One sanity unit test per library crate (a single
  `#[test] fn version_is_set()` is enough).

## Out of scope

- Any actual lexing, parsing, IR, interpreter, or WASM emission
  logic. Those are saga-2 onward.
- Real Yew components for the facet viewer (placeholder only).
- Build-info.json generation (deferred; the build stamp can be
  a static string this saga).
- GitHub Pages deploy script. Useful eventually, but not
  blocking for M0.
- favicon. Use the `discovery-one-badge.png` for the page; no
  separate favicon yet.

## Hard rules

- **Do not edit outside this repo.**
- Push every commit in the same session.
- **Stage the full `.agentrail/` delta in every step's commit.**
  `.agentrail/` and `.agentrail-archive/` are tracked source-of-
  record; every `git add` for a step must include them.
- Work order is: do work -> stage code AND `.agentrail/` ->
  `git commit` -> `git push` -> `agentrail complete`. Per
  `CLAUDE.md` section 4 and section "Rules for `.agentrail/`".
- `markdown-checker` clean on every changed `.md`.
- `cargo fmt --check`, `cargo clippy --all-targets
  --all-features -- -D warnings`, and `cargo test` all green
  before commit.
- reg-rs baselines: track `*.rgt` and `*.out`; ignore `*.tdb`
  and `*.lock` (per existing convention).

## Phased breakdown

Phase 0.1 -- Cargo workspace + library crate stubs + `d1-cli`
            with stable `--help` banner.
Phase 0.2 -- `ui/web` Yew SPA shell built via Trunk; placeholder
            home page; `Trunk.toml`.
Phase 0.3 -- `scripts/build-all.sh`, `scripts/run-web.sh`,
            `scripts/run-fixture.sh` plus `.gitignore` policy.
Phase 0.4 -- `d1_smoke_cli_help` reg-rs baseline; full pre-commit
            gate green; `docs/status.md` updated to mark M0 done.

Each phase becomes its own step. Steps are added by
`agentrail add` and worked one at a time per the protocol in
`CLAUDE.md`.

## End state

- `cargo build` and `cargo test` are clean across the workspace.
- `cargo clippy --all-targets --all-features -- -D warnings`
  is clean.
- `cargo fmt -- --check` is clean.
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_smoke_cli_help`
  is green.
- `markdown-checker -f "**/*.md"` is clean.
- `.agentrail/` (saga.toml, plan.md, all step.toml, prompt.md,
  summary.md files) is tracked in git and committed in every
  step.
- `docs/status.md` lists M0 as done; M1 (`discoveryone-lex`) is
  the next saga to begin.
