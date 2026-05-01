# DiscoveryOne -- Status

Snapshot of where the project is. Updated as milestones close.

## Today (2026-05-01)

- **Phase:** M10 3D viewer demo complete; choosing the next saga.
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
  `d1_web_facet_front_html_snapshot` and
  `d1_power_front_view_acceptance` are GREEN for the structured
  Power Front notation shown in the web demo. The M8 library grid
  is GREEN with six bundled rows and stable sort snapshots for
  name, arity, type, and aspects. `d1_web_power_front_edit_run`
  is GREEN for the narrow executable edit demo: edit the Power
  Front zero-case value from `1` to `2`, run `n=5,e=0`, and
  observe output `2`.
  `d1_web_pipeline_power_output` is GREEN for the M9 pipeline
  demo: the Web UI renders `Power -> Output`, validates
  `Power.p` feeding `Output.value`, and runs the bundled fixture
  to output `256`.
  `d1_3d_viewer_acceptance` is GREEN for the M10 3D viewer demo:
  the Web UI exposes 54 positioned Power symbols, copies the
  `viewer3d.js` three.js scene bundle, renders the `3D Viewer`
  panel, and documents the localhost manual canvas check.
  Software Wrighter checklist metadata is clean.
- **Last meaningful commit:** M10 3D viewer demo.

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

## Completed

**M7 -- `discoveryone-minted-module`.** Complete.
`d1_poc_dowhile_minted_syntax` is GREEN for the CLI path:
`examples/dowhile.d1` defines `*syntax do _ while _ end expand`
and `d1 run` prints `1 2 3` followed by `3`.
`d1_poc_dowhile_web_run` is GREEN for showing that declaration
in the Internal Z layer and rendering the web RunPanel output.
`d1_poc_unless_minted_syntax` is GREEN for a different
`unless _ do _ end` two-slot template, proving the current M7
syntax-template scaffold is not dowhile-only.
`d1_poc_unless_web_run` is GREEN through the same web demo path
used by the dowhile fixture.
`d1_minted_module_acceptance` is GREEN and `docs/poc-demo.md`
walks through the localhost:1078 demo scenario.

## Completed

**Power front-view demo usability.** The Web UI now renders the
requested structured Power Front notation:

```text
                                      ⎧ 1 →
   • Power (n : ℤ  e : ℤ) → (p : ℤ) ← ⎨ loop e times    iff e is positive
                                      ⎩ n (×) →
```

`d1_web_facet_front_html_snapshot` and
`d1_power_front_view_acceptance` are GREEN. Layout, real build
metadata, run controls, browse/edit mode, validation feedback,
and visible 2D facet editing are implemented. Edit mode now
edits the selected facet text rather than raw/internal source
text. For Power Front, changing the top-branch zero-case integer
is executable for `e=0`; malformed Power Front executable edits
fail the RunPanel explicitly. General reverse projection from 2D
facet edits back into full `.d1` source remains out of scope.

## Completed

**Power Front executable edit demo.** Complete.
`d1_web_power_front_edit_run` is GREEN for the demo path: edit
the Power Front zero-case value from `1` to `2`, run with
`n=5,e=0`, and observe output `2`. The support is intentionally
narrow and Power Front-specific; positive-exponent Power runs and
minted syntax demos remain unchanged.

## Completed

**M8 -- `discoveryone-library-grid`.** Complete. The Web UI renders
a Library grid with six bundled metadata rows: `Add`, `Clamp`,
`Power`, `DowhileCounter`, `UnlessCounter`, and `TraceValue`.
`d1_web_library_grid_snapshot` is GREEN for the default order, and
`d1_web_library_grid_sort_name`,
`d1_web_library_grid_sort_arity`,
`d1_web_library_grid_sort_type`, and
`d1_web_library_grid_sort_aspects` are GREEN for stable sort paths.
`d1_library_grid_acceptance` aggregates the M8 demo acceptance path.

## Completed

**M9 -- `discoveryone-pipeline-2d`.** Complete. The Web UI renders
a deterministic 2D `Power -> Output` pipeline with two positioned
nodes, typed ports, the `Power.p -> Output.value` edge, inline
validation, and runtime output. `d1_web_pipeline_power_output` is
GREEN and shows `Valid: Power.p feeds Output.value.` followed by
pipeline output `256` for bundled inputs `n=2,e=8`.

The implementation is intentionally narrow. The canvas is a bundled
fixture, not a drag/drop authoring surface. It validates the required
edge shape and compatible `Z -> Z` ports, but it does not yet provide
editable connections, pipeline persistence, arbitrary node graphs,
or general multi-node scheduling beyond the `Power -> Output` demo.

## Completed

**M10 -- `discoveryone-3d-viewer`.** Complete. The Web UI renders
a `3D Viewer` panel for Power and wires it to a copied
`viewer3d.js` three.js scene bundle. The panel starts with a stable
loading fallback and, in the browser, passes the deterministic
Power symbol export into `window.initDiscoveryOne3dViewer`. The
scene module publishes `window.discoveryOne3dSymbols`, creates one
glyph block per exported symbol, and renders the scene into the
panel mount.

`d1_web_3d_symbols_power` is GREEN for the JSON-friendly Power
symbol export: 54 symbols with text, face, and x/y/z coordinates.
`d1_web_3d_viewer_power` is GREEN for the deterministic viewer
panel snapshot. `d1_3d_viewer_acceptance` is GREEN and covers the
symbol export, viewer mount, Trunk asset copy hook, JS scene hook,
and manual localhost demo instruction.

The implementation remains inspection-only. It does not provide
3D authoring, selection editing, camera persistence, WebGL
pixel-level automation, or pipeline editing in 3D.

The next session that picks up feature work should:

1. Read this file.
2. Read `docs/poc-demo.md` for the current localhost:1078 demo.
3. Run `agentrail next` (per `CLAUDE.md`).
4. Start the selected next saga.

## Up next

Choose the next saga. The default roadmap choice is M10 --
`discoveryone-aspects`, covering trace/profile/on-error aspect
weaving. Other reasonable follow-ups are to deepen M9 with editable
pipeline wiring, add browser automation for the M10 canvas, or
expand Power Front editing beyond the current zero-case slice, but
M11 is the next planned milestone.

## Open questions parked

These are tracked in `docs/design.md` section 14. Briefly:

- `*` vs `:=` for definitional binding (defer to Tuplet's
  resolution).
- Aspect priority for PCA-style relevance when more than six
  aspects compete for facet placement.
- Whether the Rear facet displays WAT or a stack-IR view by
  default.
- Pipeline serialization (extends `.d1.json` with a connections
  table; still open after the narrow M9 fixture).

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
