# DiscoveryOne -- Status

Snapshot of where the project is. Updated as milestones close.

## Today (2026-04-26)

- **Phase:** M1 lexer started.
- **Code:** Cargo workspace, placeholder library crates, Yew web
  shell, build/run scripts, and first reg-rs smoke baseline are
  in place. `d1 lex` handles the first arity-suffix lexer
  fixture with a compact token display path.
- **Last meaningful commit:** M1 lexer display refactor.

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

## In flight

**M1 -- `discoveryone-lex`.** Current state is GREEN for the
first lexer case: `d1_lex_arity_suffix` passes and `d1 lex`
prints the expected token dump for `coord2 -> (x y)`. The lexer
surface has been refactored enough for the next case. The next
step should continue the M1 TDD sequence with the mint-operator
RED baseline.

The next session that picks up feature work should:

1. Read this file.
2. Read `docs/plan.md` section 3 (M1 work list).
3. Run `agentrail next` (per `CLAUDE.md`).
4. Continue the M1 lexer work.

## Up next

**M1 -- `discoveryone-lex`.** The first substantive feature is
the Rust port of Tuplet's lexer. See `docs/plan.md` section 3.

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
