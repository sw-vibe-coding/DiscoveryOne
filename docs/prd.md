# DiscoveryOne -- Product Requirements

## 1. Vision

DiscoveryOne is an experimental visual programming language whose
source is a positioned collection of glyphs in 3D space. The same
definition can be **viewed from multiple facets**, each surfacing
one semantic aspect:

| Facet | What it shows |
|---|---|
| Front | Algorithm gist -- the readable story |
| Left | Inputs (arity, names, types) |
| Right | Outputs (arity, names, types) |
| Top | Preconditions |
| Bottom | Postconditions |
| Rear | Generated form (target IR or transpiled output) |
| Internal Z layers | Implementation, error handling, instrumentation |

The user authors a definition once and the language tool projects
it into the relevant facets. This makes programs **explainable**:
the front facet stays uncluttered for reading while preconditions,
postconditions, error handling, tracing, and lowered code all live
in spatially-separable layers.

DiscoveryOne is the successor to Tuplet (in `../tuplet`). Tuplet
proved that user-mintable verbs and syntax (`*` operator,
`*syntax T expand E`) work on a 2D layout-sensitive surface with
an OCaml host and a Forth runtime. DiscoveryOne lifts the source
model to 3D, replaces OCaml with Rust, replaces Forth with Rust
compiled to WASM, and ships an authoring/inspection web app
written in Yew.

## 2. Goals

- **Spatial source.** Definitions are positioned-symbol sets in 3D.
- **Multi-facet authoring and viewing.** Six standard facets in the
  MVP, with capacity for more aspects mapped via PCA-style relevance.
- **User-minted everything.** The `*` mint operator from Tuplet
  drives all new bindings, verbs, and syntax. Even `if/then/else`
  is user-minted, not built into the kernel.
- **Web-first tooling.** Yew/Rust/WASM browser app is the primary
  authoring and inspection surface.
- **Test-driven from day one.** Every feature ships with reg-rs
  baselines covering parser, checker, IR, interpreter, WASM
  runtime, and deterministic web demos.
- **Cross-checked execution.** A reference Rust interpreter and a
  Rust-to-WASM emitter produce identical output on the test
  corpus; reg-rs catches drift.
- **No backend server.** The MVP runs entirely client-side.

## 3. Non-goals (initial milestones)

- Hindley-Milner type inference. DiscoveryOne keeps Tuplet's
  nominal/positional/arity-only typing.
- Modules, functors, namespaces as a first-class language feature.
- Exceptions or effect tracking.
- Garbage-collected heap. Values are stack cells.
- Floating point beyond percent literals (`50%` -> 0.5 internally).
- Polymorphism, typeclasses, implicits.
- True-3D rendering (three.js) -- explicit deferred goal; the
  Yew/2D facet viewer is the MVP.
- Persistent server-side state.
- Self-hosting parser.

## 4. Personas

**P1 -- Language designer.** Mints new verbs and syntax,
experiments with aspect mappings, writes the prelude. Cares about
parser correctness, IR consistency, and the ability to add a new
construct without touching the compiler.

**P2 -- DSL author.** Builds a domain language on top of
DiscoveryOne (e.g. data plotting, control synthesis). Cares about
the library/dictionary view, sortable definitions, and stable
WASM runtime semantics.

**P3 -- Application author.** Composes minted verbs into
pipelines to build a small program. Cares about pipeline
visualization, debugging traces, and being able to inspect the
rear/lowered facet to understand what runs.

**P4 -- Educator / explainer.** Uses DiscoveryOne to demonstrate
programming-language concepts. Cares about facet-rotation as a
teaching device: front shows the story, top shows assumptions,
bottom shows guarantees, rear shows the machine form.

## 5. Use cases

### UC-A. Author a single minted definition

A user opens the web app, picks "new definition", types a name
(`*Power`), authors the front facet (algorithm gist), the left
facet (`n : Int, e : Int`), the right facet (`p : Int`), the top
facet (`e >= 0`), the bottom facet (`p == n^e`), and an
implementation in an internal Z layer. The web app saves the
definition as a `.d1` file.

**Success:** the file parses cleanly, the facet checker passes,
the WASM emitter produces a runnable module.

### UC-B. View a minted definition's facets

The user opens an existing `.d1` file. The web app shows the
default Front facet. The user clicks Left, Right, Top, Bottom,
Rear and sees each in turn. A Z-layer slider exposes hidden
implementation layers.

**Success:** facet switches are instantaneous; ASCII/CSS
rendering is deterministic enough that reg-rs can baseline an
HTML snapshot.

### UC-C. Run a minted definition with inputs

The user enters input values in the Run panel, clicks Run. The
WASM module loaded in the page executes; the Run panel shows
the output values.

**Success:** the WASM run output matches the reference Rust
interpreter for every test in the corpus.

### UC-D. Inspect generated / lowered form

The user rotates to the Rear facet. The app regenerates the
target form (initially: WASM Text Format / WAT, possibly with
a Forth-style stack-IR view alongside) from the canonical core
IR and displays it.

**Success:** the rear facet always reflects the current source;
no stale cache.

### UC-E. Browse a library of minted verbs

The user opens a directory of `.d1` files. The app renders a 2D
grid where each cell is a definition. The user sorts by input
arity, then by output type. The user filters to "definitions
with preconditions".

**Success:** sorting/filtering is fast for a library of ~100
definitions; the displayed metadata is computed from the parsed
AST, not from a separate manifest.

### UC-F. Compose a small pipeline

The user drags three minted verbs onto a 2D canvas and connects
their outputs to inputs. The app type-checks the connections
and refuses ill-typed wiring. The user runs the pipeline.

**Success:** invalid connections are rejected at the UI layer
before reaching the runtime; valid pipelines execute via the
same WASM runtime as single definitions.

### UC-G. Add a new aspect to an existing definition

The user adds a `trace` aspect to a definition. Without changing
the front facet, runs of the definition now emit a trace. The
user toggles the aspect off and runs without trace overhead.

**Success:** aspect toggles are reflected in regenerated WASM
without source edits to the front facet.

## 6. MVP scope

The MVP delivers the **minted module milestone**: a single
user-authored definition (the canonical example is `*Power`)
that round-trips authoring -> facet view -> facet check -> WASM
execution, with a regression-test baseline covering each feature
along the way. The web app surfaces the six standard facets and
runs the WASM module in-page.

In scope for the MVP:

- The `*` mint operator and the four kernel forms it gates:
  `*name <- expr`, `*name -> (fields)`, `*name(args) -> (outs) <- body`,
  `*syntax T expand E`.
- Six standard facets with the default semantic mapping (sec. 1).
- Layered-text source format (`.d1`) and canonical JSON
  (`.d1.json`) round-trip.
- Voxel loader, projection engine, lexer, parser, checker, IR,
  interpreter, WASM emitter -- the full pipeline -- but for the
  stripped-down feature set needed to express `*Power` and
  `*syntax do _ while _ end expand` (the Tuplet PoC analog).
- Yew web app with `App`, `DefinitionPicker`, `FaceSelector`,
  `FacetView`, `ZLayerSlider`, `RunPanel`.
- reg-rs corpus covering: lexer tokenization, parser AST dumps,
  checker pass/fail messages, IR dumps, interpreter outputs,
  WASM-via-wasmtime outputs, Playwright-driven web demo
  snapshots.

Out of MVP scope (deferred to later milestones):

- Library grid view (M8).
- Pipeline editor (M9).
- three.js 3D viewer (M10).
- Aspect weaving beyond pre/post (M11+).
- Tuple variables, multi-output verbs, percent literals --
  these arrive in M7 once the minted-module vertical slice is
  proven on the simpler kernel subset.

## 7. UX principles

1. **Front facet is the README.** Reading the front facet alone
   is enough to understand what a definition does. Other facets
   add depth without altering the reading experience.
2. **No invisible magic.** If a facet is generated (Rear), the
   user can see exactly what produced it. If a facet is checked
   (Top, Bottom), failures point at source coordinates.
3. **Determinism over ergonomics.** The web app's rendering is
   stable enough that snapshot tests catch regressions.
4. **Spatial sorting beats alphabetical.** Library views default
   to arity / type / aspect axes, not alphabetical name lists.
5. **Errors carry coordinates.** Every error message references
   `(file, definition, facet, x, y, z)` not just a line number.

## 8. Success criteria

The MVP is met when **all** of the following hold:

1. A `*Power` definition authored in `.d1` form parses, checks,
   and lowers to WASM with no errors.
2. The web app loads the `.d1` file, displays Front / Left /
   Right / Top / Bottom / Rear correctly, and runs the WASM
   module in-page.
3. The user can also author the Tuplet PoC analog
   (`*syntax do _ while _ end expand`) inside DiscoveryOne and
   exercise the minted syntax in a second definition that uses
   it. This proves user-extensibility, mirroring Tuplet's
   `tuplet_poc_dowhile` baseline.
4. Reg-rs baselines exist for at least 12 features, named per
   the convention `d1_<area>_<case>`. Examples:
   `d1_lex_arity_suffix`, `d1_parse_mint_signature`,
   `d1_check_facet_left_arity_mismatch`, `d1_ir_power_dump`,
   `d1_run_power_2_8_eq_256`, `d1_web_facet_front_snapshot`.
5. Every reg-rs case is green; no `--allow-failures`.
6. `cargo test`, `cargo clippy --all-targets --all-features --
   -D warnings`, `cargo fmt --check`, and `markdown-checker -f
   "**/*.md"` are all clean.
7. The CLI `d1` (working name) supports the subcommands the
   regression harness needs: `parse`, `check`, `lower`, `run`,
   `emit-wasm`, `face` (face projection dump), `facet-snapshot`
   (deterministic HTML snapshot of a facet view).
8. `docs/status.md` is up to date and reflects what shipped.

## 9. Risks

- **Editor burden.** Hand-editing 3D coordinates is unworkable.
  Mitigation: layered-text `.d1` is the human-edited form; JSON
  is the canonical machine form; the web app is the eventual
  WYSIWYG editor (post-MVP).
- **Six views become six languages.** A definition with
  inconsistent facets is meaningless. Mitigation: the facet
  consistency checker (R1-R7 in `docs/architecture.md`) is part
  of the MVP, not deferred.
- **Visual cleverness over substance.** Mitigation: every facet
  has a job (sec. 1); rotation between facets is for separating
  concerns, not for decoration.
- **Rust + WASM + Yew compile times** can hurt iteration.
  Mitigation: keep crates small (`d1-lex`, `d1-parse`, etc.);
  the CLI invocation path stays native-only.
- **Snapshot brittleness.** HTML/CSS snapshots of facet views can
  drift on cosmetic changes. Mitigation: snapshot the rendered
  ASCII grid (semantic content), not the surrounding chrome.
- **Drift from Tuplet's mechanism.** If DiscoveryOne's `*` mint
  semantics quietly diverge from Tuplet's, users moving between
  the two will be confused. Mitigation: cross-reference Tuplet's
  `docs/grammar.md` and `docs/kernel.md`; record any intentional
  divergence in `docs/design.md`.
- **`reg-rs` corpus rot.** If baselines accumulate without
  curation, future regressions become hard to interpret.
  Mitigation: each milestone adds a stable, named subset; tests
  are renamed only with an explicit migration step.

## 10. Out-of-scope deflectors

When pressure arises to add the following before the MVP ships,
deflect to a later milestone:

- Type inference of any kind beyond arity.
- Self-hosting (DiscoveryOne hosting itself).
- A Rust-to-native code generator (the WASM target is enough).
- A package manager / module system.
- Any backend server, database, or auth system.
- A full visual editor for 3D coordinates (post-MVP polish).
- three.js anything (its own milestone).
