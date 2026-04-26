# DiscoveryOne -- Plan

This is the demo / TDD milestone plan for DiscoveryOne. The
spine is `reg-rs`: every milestone adds a named subset of the
test corpus from `docs/design.md` section 11, and no milestone
is "done" until its baselines are green.

The single demoable target is the **minted module milestone**
(M7): a user-authored definition (`*Power` plus a user-minted
`*syntax do _ while _ end expand`) that round-trips authoring
-> facet view -> facet check -> WASM run inside the Yew web
app, fully baselined by `reg-rs`.

## 0. Working principles

- **Red-Green-Refactor.** Every feature begins with a failing
  reg-rs baseline (or a failing `cargo test`), then minimal code
  to pass, then refactor without breaking baselines.
- **One milestone, one PR (or one step).** Milestones do not
  bleed into each other. If work expands, file the spillover as
  a follow-up milestone.
- **Pre-commit gates apply.** `cargo test`, `cargo clippy
  --all-targets --all-features -- -D warnings`, `cargo fmt --
  --check`, `markdown-checker -f "**/*.md"`, and (if installed)
  `sw-checklist` must all pass before commit.
- **Push every commit.** Per `docs/process.md`.
- **Agentrail integration.** This project will adopt agentrail
  (per `CLAUDE.md`) once M0 lands. The first agentrail saga is
  `discoveryone-scaffold` covering M0-M1; subsequent sagas align
  one-to-one with milestones M2 and beyond. Commits go through
  `agentrail begin` / `git commit` / `agentrail complete`.
- **No three.js until M10.** True-3D rendering is deliberately
  deferred. The 2D facet viewer is the authoring surface for
  M0-M9.

## 1. Milestone arc

| #   | Milestone                          | Goal                                                                 | Demo / acceptance                              |
|-----|------------------------------------|----------------------------------------------------------------------|------------------------------------------------|
| M0  | `discoveryone-scaffold`            | Cargo workspace, crates skeleton, reg-rs harness, CI smoke.          | `cargo build` + 1 reg-rs smoke test.           |
| M1  | `discoveryone-lex`                 | Rust lexer for `.d1` token set incl. aspect/Z tags.                  | `d1 lex` reg-rs cases for arity, mint, tags.   |
| M2  | `discoveryone-source-loader`       | Parse layered text and JSON into positioned-symbol set.              | `d1 normalize` round-trip baseline.            |
| M3  | `discoveryone-parse-check`         | AST + facet checker (R1-R3 mandatory rules).                         | `d1 parse` + `d1 check` baselines.             |
| M4  | `discoveryone-ir-interp`           | Stack IR + reference Rust interpreter.                               | `d1 interp` for Power.                         |
| M5  | `discoveryone-wasm-emit`           | WAT emitter + wasmtime CLI runner.                                   | `d1 run` for Power matches `d1 interp`.        |
| M6  | `discoveryone-web-shell`           | Yew SPA: load a `.d1`, switch facets, render front facet.            | Playwright snapshot of front facet.            |
| M7  | **`discoveryone-minted-module`**   | Vertical slice: `*Power` + `*syntax do _ while _ end` end-to-end.    | Web app runs Power and dowhile demo, baselined.|
| M8  | `discoveryone-library-grid`        | 2D grid of definitions; sort by arity / type / aspect.               | Library snapshot reg-rs.                       |
| M9  | `discoveryone-pipeline-2d`         | 2D pipeline editor; type-checked connections.                        | Pipeline runs in browser; reg-rs baseline.     |
| M10 | `discoveryone-3d-viewer`           | three.js voxel scene reading the same symbol set.                    | 3D viewer renders Power; manual demo.          |
| M11 | `discoveryone-aspects`             | Aspect weaving: trace, profile, on-error toggles.                    | Toggle trace; reg-rs trace output.             |

The MVP success criteria from `docs/prd.md` section 8 are met at
the end of M7. M8-M11 are post-MVP polish and breadth.

## 2. M0 -- `discoveryone-scaffold`

**Goal.** Empty workspace that builds, runs one smoke test, and
exposes the test harness pattern every later milestone follows.

**Entrance criteria.** `docs/{architecture,prd,design,plan,
status}.md` and `README.md` committed.

**Work.**

1. `Cargo.toml` workspace; create empty crates: `d1-source`,
   `d1-geom`, `d1-lex`, `d1-parse`, `d1-check`, `d1-ir`,
   `d1-interp`, `d1-emit-wasm`, `d1-cli`.
2. Create `ui/web` Yew skeleton; `Trunk.toml`; `index.html`.
3. Create `scripts/build-all.sh`, `scripts/run-web.sh`,
   `scripts/run-fixture.sh`.
4. Create one reg-rs baseline: `d1_smoke_cli_help` --
   `d1 --help` outputs a stable banner.
5. `agentrail init --name discoveryone-scaffold` (CLAUDE.md
   protocol). Commit `.agentrail/`.

**Exit criteria.**

- `cargo build` is clean.
- `cargo clippy --all-targets --all-features -- -D warnings` is
  clean.
- `cargo fmt -- --check` is clean.
- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_smoke_cli_help`
  is green.
- `markdown-checker -f "**/*.md"` clean.

**TDD note.** The smoke baseline is intentionally trivial; its
purpose is to validate the test-harness wiring. The first
substantive test arrives in M1.

## 3. M1 -- `discoveryone-lex`

**Goal.** Rust port of Tuplet's lexer for the DiscoveryOne token
set, including `@front`, `@left`, ..., `@rear`, `@internal`,
`@z N`. ASCII-only at first; Unicode aliases (`U+2022`, `U+2190`,
etc.) follow Tuplet.

**TDD sequence.**

1. **RED.** Write `tests/lexer/arity_suffix.input` with
   `coord2 -> (x y)` and `tests/lexer/arity_suffix.expected`.
   Add reg-rs case `d1_lex_arity_suffix`. It fails: lexer is
   not implemented.
2. **GREEN.** Implement `d1-lex` minimum to pass the case.
3. **REFACTOR.** Extract token-display helper, run clippy clean.
4. Repeat for: mint operator, aspect tags, `@z` tags, comments,
   negative integers, percent literals.

**Reg-rs cases (this milestone).**

```text
d1_lex_arity_suffix
d1_lex_mint_operator
d1_lex_aspect_tags
d1_lex_zlayer_tags
d1_lex_comment
d1_lex_neg_int
d1_lex_percent
```

**Exit.** Every case green; `cargo test -p d1-lex` green; clippy
clean; pre-commit gates pass.

## 4. M2 -- `discoveryone-source-loader`

**Goal.** Parse `.d1` (layered text) into a positioned-symbol
set; round-trip to/from `.d1.json`.

**TDD sequence.**

1. **RED.** Hand-write `examples/power.d1.json` (canonical form)
   and `examples/power.d1` (layered text). Reg-rs case
   `d1_normalize_roundtrip_power` runs `d1 normalize` and
   expects byte-identical output for the layered text after a
   JSON round-trip.
2. **GREEN.** Implement loader and dumper.
3. **REFACTOR.** Add `d1 face --face F` baseline projecting Power.

**Reg-rs cases.**

```text
d1_normalize_roundtrip_power
d1_face_front_power
d1_face_left_power
d1_face_right_power
d1_face_top_power
d1_face_bottom_power
d1_face_rear_power
```

**Exit.** Round-trip is byte-exact; all six face projections
match expected token grids.

## 5. M3 -- `discoveryone-parse-check`

**Goal.** Build AST per `docs/design.md` section 4; run facet
consistency rules R1-R3 (mandatory).

**TDD sequence.**

1. **RED.** Reg-rs `d1_parse_mint_init` -> `*n <- 0` parses
   to one `Stmt::Assign(Pattern::Name("n"), Expr::Int(0))`
   inside a Front facet.
2. **GREEN.** Implement parser scaffold producing AST dump.
3. Add `d1_parse_mint_signature` (`*power(n e) -> (p) <- ...`).
4. Add `d1_parse_facet_blocks` (multiple @-tagged blocks for
   one definition).
5. Add `d1_check_pass_power` and `d1_check_fail_left_arity_
   mismatch`. Wire R1 (input arity) and R3 (reachability) first;
   R2 (output arity) follows.
6. Add `d1_check_fail_unbound_name`.

**Reg-rs cases.**

```text
d1_parse_mint_init
d1_parse_mint_signature
d1_parse_facet_blocks
d1_check_pass_power
d1_check_fail_left_arity_mismatch
d1_check_fail_unbound_name
```

**Exit.** Parser handles the kernel forms; checker rules R1-R3
pass for Power and fail with stable error codes for the negative
cases.

## 6. M4 -- `discoveryone-ir-interp`

**Goal.** Lower checked AST to stack IR; build a reference Rust
interpreter; produce expected outputs that the WASM emitter must
match.

**TDD sequence.**

1. **RED.** `d1_ir_power_dump` -- expected stack-IR listing for
   Power.
2. **GREEN.** Implement IR generation.
3. **RED.** `d1_interp_power_2_8_eq_256` -- run interpreter on
   Power with `n=2, e=8`; expect `256\n` on stdout.
4. **GREEN.** Implement interpreter; pass.
5. Add `d1_interp_power_5_0_eq_1`.

**Reg-rs cases.**

```text
d1_ir_power_dump
d1_interp_power_2_8_eq_256
d1_interp_power_5_0_eq_1
```

**Exit.** Interpreter is the test oracle for the next milestone.

## 7. M5 -- `discoveryone-wasm-emit`

**Goal.** Emit WAT, assemble to WASM bytes, run via `wasmtime` in
the CLI; outputs must match the interpreter on the corpus.

**TDD sequence.**

1. **RED.** `d1_wasm_power_2_8_eq_256` -- run `d1 run
   examples/power.d1 --inputs n=2 e=8`; expect `256\n`.
2. **GREEN.** Implement WAT emitter; assemble via `wat` crate;
   run via `wasmtime` crate.
3. Add `d1_wasm_power_5_0_eq_1`.
4. Add a divergence-watchdog test: run all corpus through
   interpreter and WASM; fail loudly on any mismatch.

**Reg-rs cases.**

```text
d1_wasm_power_2_8_eq_256
d1_wasm_power_5_0_eq_1
d1_wasm_interp_parity_corpus
```

**Exit.** Every interpreter case has a matching WASM case with
identical output.

## 8. M6 -- `discoveryone-web-shell`

**Goal.** Yew SPA loads a `.d1` file, displays the Front facet,
switches between facets, runs the loaded `.d1` via a web-side
WASM runtime.

**TDD sequence.**

1. **RED.** `d1_facet_snapshot_power_front` -- CLI command
   `d1 facet-snapshot examples/power.d1 --face Front` produces a
   deterministic ASCII grid; this is the authoritative snapshot.
2. **GREEN.** Implement `facet-snapshot`.
3. **RED.** Yew app under `ui/web` -- wire `App`,
   `DefinitionPicker`, `FaceSelector`, `FacetView`. The
   `FacetView` reuses the same projection code via
   `wasm-bindgen` from `d1-geom`.
4. **GREEN.** Add a Playwright test (via Playwright MCP) that
   loads the page, picks Power, clicks each face button, and
   asserts the rendered grid matches `d1 facet-snapshot` output.
5. Add `RunPanel`: enter `n=2, e=8`, click Run, expect output
   `256`. Browser snapshot baselines the result.

**Reg-rs cases.**

```text
d1_facet_snapshot_power_front
d1_facet_snapshot_power_left
d1_facet_snapshot_power_right
d1_facet_snapshot_power_top
d1_facet_snapshot_power_bottom
d1_facet_snapshot_power_rear
d1_web_facet_front_html_snapshot
d1_web_run_power_2_8
```

**Exit.** Web app round-trips Power source -> facet display ->
WASM run, with every facet snapshot baselined and matching the
CLI's snapshot.

## 9. M7 -- `discoveryone-minted-module` (the MVP milestone)

**Goal.** A user authors a `*Power` definition AND a
`*syntax do _ while _ end expand` declaration in the web app;
the second definition exercises the minted syntax. Both run in
the browser. Both are baselined by reg-rs.

This is the analog of Tuplet's `tuplet_poc_dowhile` PoC. The
distinguishing claim of DiscoveryOne is that the same vertical
slice runs through facet projection, facet checking, and the
Yew web app -- not just through a CLI/REPL.

**TDD sequence.**

1. **RED.** `d1_poc_dowhile_minted_syntax` -- a scripted
   session through `d1 run` that:
   - parses `examples/dowhile.d1` containing the `*syntax`
     declaration plus a use site (counts 1, 2, 3),
   - prints `1 2 3\n3\n`.
2. **GREEN.** Wire `*syntax T expand E` template registration
   in the parser; longest-match-wins, first-declared-wins-on-ties
   (per Tuplet's plan saga 2).
3. **RED.** Web demo: open `examples/dowhile.d1`, the FacetView
   shows the syntax declaration in the Internal Z layer, run
   produces the same `1 2 3` output. Playwright snapshot.
4. **GREEN.** Verify; capture baseline.
5. **RED.** Substitution test: replace `do/while` with `unless/do`
   (a different two-slot template); the web app accepts it and
   runs without code changes. This proves the mechanism is
   general, mirroring Tuplet's PoC item 5.
6. **GREEN.** Pass.

**Reg-rs cases.**

```text
d1_poc_dowhile_minted_syntax
d1_poc_dowhile_web_run
d1_poc_unless_minted_syntax
d1_poc_unless_web_run
d1_minted_module_acceptance
```

**Exit (= MVP exit per `docs/prd.md` section 8).**

- All M0-M7 reg-rs cases green.
- The `discoveryone` web app demo at `localhost:1078` (the
  default for `scripts/run-web.sh`; override with an arg)
  round-trips Power
  and dowhile interactively.
- `docs/status.md` updated.
- `docs/poc-demo.md` walks through the scenario.

## 10. M8 -- `discoveryone-library-grid`

**Goal.** 2D grid of definitions; sort by arity / type / aspect.

Reg-rs covers grid HTML snapshots for ~6 definitions; sort
operations produce stable orderings. Mostly UI work; the
underlying parser/checker is unchanged.

## 11. M9 -- `discoveryone-pipeline-2d`

**Goal.** Drag minted verbs onto a canvas; connect outputs to
inputs; type-check connections; run the pipeline. 2D rendering
only.

Reg-rs covers a small pipeline (e.g. `Power` -> `Add` -> output)
running end-to-end via WASM in the browser.

## 12. M10 -- `discoveryone-3d-viewer`

**Goal.** three.js voxel scene that reads the same positioned-
symbol set as the Yew app. The 3D viewer is for inspection and
demo; authoring still happens in the 2D facet viewer.

The interface boundary: `ui/web` exposes a `wasm-bindgen` global
with `get_symbols(def_name) -> JsValue` returning a JSON object
of the Definition's symbols. A separate JS bundle uses three.js
to render that scene.

Reg-rs cannot snapshot 3D rendering directly; this milestone has
fewer baselines and relies on a manual demo scenario plus a
`docs/3d-demo.md` walk-through.

## 13. M11 -- `discoveryone-aspects`

**Goal.** Aspect weaving: turn on / off `trace`, `profile`,
`on-error`. Pre/post conditions become runtime guards when their
aspect is enabled.

Reg-rs covers: trace output of a known program, profile counters
of a small loop, on-error catches a deliberate fault.

## 14. Cross-cutting concerns

These apply to every milestone. They are not restated per
milestone:

- Pre-commit gates per `docs/process.md`.
- Markdown ASCII-only via `markdown-checker`.
- File size: keep crates' `src/*.rs` under 500 lines, prefer
  200-300.
- TODO budget: max 3 per file; address within 2 sessions.
- Push every commit.
- Reg-rs baselines are the contract: never disable, always fix.
- `.agentrail/` is tracked in git; never edited by hand.
- Every error message carries source coordinates (file,
  definition, facet, x, y, z) where applicable.

## 15. Risks (per milestone)

| Milestone | Risk | Mitigation |
|---|---|---|
| M0 | Workspace bootstrap drift between crates. | One `Cargo.toml` workspace; pin Rust edition 2024 once. |
| M1 | Aspect/Z tag lexing collides with hash comments at column 0. | Tags require column 0 + immediate `@`; tested explicitly. |
| M2 | Layered-text indentation rules ambiguous. | Spec in `docs/design.md` 1.1; round-trip is the contract. |
| M3 | Facet-checker error messages too noisy on partial inputs. | Stable codes E001-E099; detailed text behind `--verbose`. |
| M4 | Stack effects diverge from Tuplet's table. | Cross-reference `../tuplet/docs/design.md`; same table. |
| M5 | WASM emitter slow to assemble large WAT. | Acceptable for MVP; M5+1 may switch to `wasm-encoder`. |
| M6 | Browser snapshot brittleness. | Snapshot ASCII grid content, not chrome; reuse CLI snapshot. |
| M7 | Template registry mutation during parse. | Single-pass parser; aborts on error; matches Tuplet's design. |
| M8 | Sort stability across runs. | Stable secondary key (definition name); document. |
| M9 | Connection type-checking errors hard to surface. | Display in-line on the offending edge; reg-rs error message. |
| M10 | three.js coordinate system mismatch with internal coords. | Document in `docs/3d-demo.md`; one transform, one direction. |
| M11 | Aspect weaving alters semantics silently. | Aspects opt-in; reg-rs runs with each enabled aspect set. |

## 16. Out-of-plan (deferred)

These are not on the roadmap. They will be considered only after
M11:

- Hindley-Milner typing.
- Self-hosting parser.
- Module / namespace system.
- Server-side persistence.
- Multi-user collaboration.
- Native code generation (we ship WASM).
- Mobile / iPad native apps.
- VR / AR rendering of 3D source (a fun thought; not on the
  roadmap).
