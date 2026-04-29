# DiscoveryOne -- Status

Snapshot of where the project is. Updated as milestones close.

## Today (2026-04-28)

- **Phase:** M6 web shell planned.
- **Code:** Cargo workspace, placeholder library crates, Yew web
  shell, build/run scripts, and first reg-rs smoke baseline are
  in place. `d1 lex` handles the arity-suffix, mint-operator,
  aspect-tag, Z-layer, comment, negative integer, and percent
  literal lexer fixtures. `d1 normalize` now round-trips the
  Power layered-text fixture through the source loader path, and
  all six planned Power face projections render through `d1
  face`. `d1_parse_mint_init`, `d1_parse_mint_signature`,
  and `d1_parse_facet_blocks` are GREEN with minimal AST dumps.
  `d1_check_pass_power` is GREEN with a minimal checker report.
  `d1_check_fail_left_arity_mismatch` is GREEN with stable
  checker error code `E007` for the first input arity mismatch
  fixture.
  `d1_check_fail_unbound_name` is GREEN with stable checker
  error code `E008` for the first unbound-name fixture.
  `d1_ir_power_dump` is GREEN with a minimal deterministic
  stack-IR dump for the Power fixture.
  `d1_interp_power_2_8_eq_256` and
  `d1_interp_power_5_0_eq_1` are GREEN with the reference
  interpreter scaffold.
  `d1_wasm_power_2_8_eq_256` and `d1_wasm_power_5_0_eq_1`
  are GREEN through the minimal WAT/Wasmtime runtime path.
  `d1_wasm_interp_parity_corpus` is GREEN and verifies the
  current Power interpreter and WASM outputs match.
  all six `d1_facet_snapshot_power_*` baselines are GREEN
  through the existing source projection path.
  `d1_web_facet_front_html_snapshot` captures the current RED
  web snapshot scaffold failure before the Yew facet viewer lands.
  Software Wrighter checklist metadata is clean.
- **Last meaningful commit:** Power front facet HTML RED baseline.

## Done

- `docs/research.txt` -- the source vision packet from the user
  describing the 3D source model, multi-facet semantics, aspect
  mapping, library/dictionary view, pipeline network, and the
  inherited Tuplet mechanism.
- `docs/background.txt` -- Rust named-tuple research (input
  material; influences how we model tuples in `d1-ir`).
- `docs/process.md` -- TDD red/green/refactor cycle and
  pre-commit gates.
- `docs/tools.md` -- Software Wrighter tool catalog (proact,
  markdown-checker, sw-install, ask, sw-checklist, favicon).
- `docs/ai_agent_instructions.md` -- generated AI-agent guidance.
- `docs/architecture.md` -- inheritance from Tuplet, source
  object model, compile pipeline, crate layout, UI architecture,
  facet consistency model, aspect-oriented hooks.
- `docs/prd.md` -- vision, goals, non-goals, personas, use cases
  UC-A through UC-G, MVP scope, UX principles, success criteria,
  risks.
- `docs/design.md` -- file formats, projection rules, lexer
  tokens, AST, stack IR, facet checker, WASM emitter, CLI
  surface, web-app components, test corpus, build pipeline,
  diagnostics.
- `docs/plan.md` -- TDD/demo milestone arc M0-M11 with reg-rs
  cases per milestone.
- `README.md` -- project intro, badge, quickstart placeholders,
  links.
- **M0 -- `discoveryone-scaffold`** -- Cargo workspace,
  `d1-source`, `d1-geom`, `d1-lex`, `d1-parse`, `d1-check`,
  `d1-ir`, `d1-interp`, `d1-emit-wasm`, `d1-cli`, Yew
  `ui/web`, helper scripts, `.gitignore` policy, and
  `d1_smoke_cli_help` reg-rs baseline.
- **M1 -- `discoveryone-lex`** -- all planned lexer baselines are
  GREEN: `d1_lex_arity_suffix`, `d1_lex_mint_operator`,
  `d1_lex_aspect_tags`, `d1_lex_zlayer_tags`, `d1_lex_comment`,
  `d1_lex_neg_int`, and `d1_lex_percent`.
- **M2 -- `discoveryone-source-loader`** -- `d1 normalize`
  round-trips the Power layered-text fixture, and
  `d1_face_front_power`, `d1_face_left_power`,
  `d1_face_right_power`, `d1_face_top_power`,
  `d1_face_bottom_power`, and `d1_face_rear_power` render all
  six planned Power face projections.
- **M4 -- `discoveryone-ir-interp`** -- `d1_ir_power_dump`,
  `d1_interp_power_2_8_eq_256`, and
  `d1_interp_power_5_0_eq_1` are GREEN.
- **M5 -- `discoveryone-wasm-emit`** --
  `d1_wasm_power_2_8_eq_256`, `d1_wasm_power_5_0_eq_1`, and
  `d1_wasm_interp_parity_corpus` are GREEN.

## Completed

**M6 -- `discoveryone-web-shell`.** Complete. All six Power
`facet-snapshot` baselines are GREEN and render the same stable
projections as `d1 face`. `d1_web_facet_front_html_snapshot` and
`d1_web_run_power_2_8` are GREEN for the bundled Power definition,
with the Yew shell rendering `DefinitionPicker`, `FaceSelector`,
`FacetView`, and `RunPanel`.

## In flight

**M7 -- `discoveryone-minted-module`.** In progress.
`d1_poc_dowhile_minted_syntax` is GREEN for the CLI path:
`examples/dowhile.d1` defines `*syntax do _ while _ end expand`
and `d1 run` prints `1 2 3` followed by `3`.

The next session that picks up feature work should:

1. Read this file.
2. Read `docs/plan.md` section 9 (M7 work list).
3. Run `agentrail next` (per `CLAUDE.md`).
4. Continue the M7 minted-module saga.

## Up next

Create the M7 RED web baseline for `d1_poc_dowhile_web_run`.

## Open questions parked

These are tracked in `docs/design.md` section 14. Briefly:

- `*` vs `:=` for definitional binding (defer to Tuplet's
  resolution).
- Aspect priority for PCA-style relevance when more than six
  aspects compete for facet placement.
- Whether the Rear facet displays WAT or a stack-IR view by
  default.
- Pipeline serialization (extends `.d1.json` with a connections
  table; design when M9 starts).

## Constraints reminders

- ASCII only in markdown (markdown-checker).
- Files under 500 lines (target 200-300).
- Max 3 TODOs per file.
- Push every commit.
- `.agentrail/` is tracked; do not edit by hand.

## Reference: external resources

- Predecessor: `../tuplet` (OCaml + Forth Tuplet).
- Test harness: `reg-rs` -- already on PATH at
  `~/.local/softwarewrighter/bin/reg-rs`.
- Web app reference: `~/github/sw-embed/web-sw-cor24-ocaml`
  (Yew + Trunk + COR24 emulator pattern; the `dist/` lock,
  `vendor-artifacts.sh`, `serve.sh`, `build-pages.sh` scripts
  are templates worth mirroring).
