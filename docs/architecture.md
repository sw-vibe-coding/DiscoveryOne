# DiscoveryOne -- Architecture

> **Language and project name:** DiscoveryOne -- a 3D-spatial
> derivative of Tuplet ([../tuplet](../../tuplet)).
> **Status:** scaffolding; nothing implemented yet. This document
> records the architecture an AI coding agent should build toward.

## 1. One-paragraph summary

DiscoveryOne is a programming language whose **source is a positioned
collection of glyphs in 3D space**, projected into multiple 2D
"facets" that each surface one semantic aspect (signature, inputs,
outputs, preconditions, postconditions, generated form). It derives
from Tuplet, which is a 2D layout-sensitive language with first-class
named tuples and user-`*`-minted verbs/syntax. DiscoveryOne is the
reimplementation: **Rust** for lexer/parser/checker (replacing
Tuplet's OCaml host) and **Rust compiled to WebAssembly** for the
runtime (replacing Tuplet's COR24 Forth). The user-facing tool is a
**Yew/Rust/WASM web app** that renders facets in 2D first; a
three.js-based true-3D viewer is a later milestone. The CLI is not a
user-facing demo runner -- it is a `reg-rs`-driven regression-test
harness covering individual language features and deterministic web
demos.

## 2. Inheritance from Tuplet

What DiscoveryOne **keeps** from Tuplet:

- The mint operator `*` as the single mechanism for adding new
  bindings, verbs, and syntax. Everything user-extensible runs
  through `*`.
- First-class named tuple bundles with arity-suffixed names
  (`coord2`, `point4`).
- Multi-output verbs (`max2`, `min2`, `div2`).
- Call-site argument splicing.
- Static arity checking.
- `*syntax T expand E` template-expansion mechanism.
- Layout-bearing source: in Tuplet, 2D whitespace; in DiscoveryOne,
  3D positions plus aspect labels.

What DiscoveryOne **changes**:

| Concern | Tuplet | DiscoveryOne |
|---|---|---|
| Host language | OCaml (`sw-cor24-ocaml` subset) | Rust (stable, edition 2024) |
| Runtime target | DTC Forth on COR24 emulator | Rust compiled to WASM, run in browser or `wasmtime` |
| Source dimensionality | 2D layout (rows, columns) | 3D voxel positions + aspect labels |
| Primary UI | None (CLI / REPL) | Yew/WASM web app; CLI is test harness only |
| Test harness | reg-rs golden output | reg-rs golden output (kept) |
| Source format(s) | `.tup` text files | `.d1` layered-text and `.d1.json` canonical-coord forms |

## 3. Source object model

A DiscoveryOne source object is a set of positioned symbols:

```text
Symbol {
  id          stable identifier within the document
  text        glyph, word, operator, minted verb name, or annotation
  coord       (x, y, z) integers
  aspect      one of { Front, Left, Right, Top, Bottom, Rear,
                       Internal, Annotation }
  metadata    optional key/value bag
}
```

A **definition** is a bounded volume of symbols that mint a single
verb or syntax form. A **module** is a collection of definitions
plus their import/export surface.

### Aspect / face mapping (initial)

The six standard physical faces map to six standard semantic roles.
This mapping is the **default**; tools may project additional
aspects into the same six faces using a PCA-style relevance pick.

| Face | Default semantic role |
|---|---|
| Front | Algorithm gist -- the readable story |
| Left | Inputs (arity, names, types) |
| Right | Outputs (arity, names, types) |
| Top | Preconditions |
| Bottom | Postconditions |
| Rear | Expansion / lowered form / target IR |
| (Internal Z layers) | Implementation, error handling, instrumentation |
| (Annotation) | Comments, examples, contracts not surfaced on a face |

The same definition compiles via **one** canonical executable core;
the other faces are checked annotations or generated views. The
"six views are six independent programs" trap is explicitly avoided.

## 4. Compile pipeline

```text
.d1 source (layered text)              .d1.json (canonical coords)
    |                                       |
    +-------------- voxel loader -----------+
                       |
                       v
              positioned-symbol set
                       |
                       v
             projection engine (per face)
                       |
                       v
   +-------- per-face 2D token grid --------+
   | Front   Left   Right   Top   Bottom   Rear |
   +--------------------------------------------+
                       |
                       v
                Tuplet-style lexer (Rust)
                       |
                       v
                  parser (Rust)
                       |
                       v
            per-face AST fragments + registry
                       |
                       v
             semantic merge / facet checker
                       |
                       v
              normalized core IR (stack)
                       |
                +------+------+
                |             |
                v             v
       Rust interpreter   Rust -> WASM emitter
       (test oracle)      (runtime path)
                              |
                              v
                       executable .wasm module
```

Stages:

1. **Voxel loader.** Parses either layered text (`@z 0` / `@front`)
   or canonical JSON into a positioned-symbol set.
2. **Projection engine.** Given a face, computes the 2D token grid
   visible on that face under the chosen occlusion rule (default:
   nearest-Z wins; tooling has access to the full stack).
3. **Lexer.** Same alphabet as Tuplet (mint `*`, arrows `<-` `->`,
   parens, comma, identifiers with arity suffix, ints, percent
   literals). Reused across faces.
4. **Parser.** Builds AST fragments per face. The Front face is
   normative for the executable surface; other faces are
   annotation-checked against it.
5. **Semantic merge / facet checker.** Verifies cross-face
   consistency (e.g. Left face's input list matches Front face's
   signature; Bottom face's postconditions reference the declared
   outputs). See section 7.
6. **Core IR.** A stack-oriented IR (see `docs/design.md`) that
   sits between Tuplet's IR and Rust/WASM lowering.
7. **Backends.** A reference Rust interpreter for cross-checking,
   and a Rust-to-WASM emitter. Both produce identical observable
   output for a given input on the test corpus.

## 5. Rust crate layout (proposed)

```text
DiscoveryOne/
  Cargo.toml                workspace
  crates/
    d1-source/             voxel loader, layered-text and JSON formats
    d1-geom/               projection, occlusion, aspect picking
    d1-lex/                Tuplet-style lexer (Rust)
    d1-parse/              parser; produces per-face AST fragments
    d1-check/              name resolution, arity check, facet consistency
    d1-ir/                 normalized core IR
    d1-interp/             reference interpreter (test oracle)
    d1-emit-wasm/          WASM emitter (cargo-built dylib or wat-string)
    d1-cli/                reg-rs-friendly CLI: parse, check, lower, run
  ui/
    web/                    Yew app (wasm-pack target)
      Cargo.toml
      src/
      static/
      pkg/                  generated by wasm-pack (gitignored)
  tests/                    fixtures (.d1 source + expected outputs)
  work/
    reg-rs/                 *.rgt and *.out (tracked); *.tdb ignored
    generated/              transient build outputs (gitignored)
  docs/                     this directory
  scripts/                  build-all.sh, run-web.sh, run-fixture.sh
```

Workspace boundaries:

- `d1-source`, `d1-geom`, `d1-lex`, `d1-parse`, `d1-check`,
  `d1-ir`, `d1-interp`, `d1-emit-wasm` are pure library crates;
  no I/O, no CLI dependencies. They compile to native and to
  wasm32-unknown-unknown unchanged.
- `d1-cli` is the only crate that owns stdin/stdout, file I/O, and
  process exit codes. It is what `reg-rs` invokes.
- `ui/web` depends on the library crates and runs entirely in the
  browser via wasm-pack. No backend server is required for the
  MVP; static file hosting is enough.

## 6. UI architecture (Yew/Rust/WASM)

### Phase 1 (MVP): 2D facet viewer

```text
+---------------------------------------------------------+
| [definition: Power] [face: Front Left Right Top Bot Rear]|
+---------------------------------------------------------+
|                                                         |
|              ASCII-art-style 2D facet view              |
|                  (rendered in HTML/CSS;                 |
|                   monospace grid)                       |
|                                                         |
+---------------------------------------------------------+
| Z-layer slider:  [0]==[1]==[2]==[3]                     |
| Hidden-stack:    [showing nearest only | show all]      |
+---------------------------------------------------------+
| Run inputs: n=[2] e=[8]  [Run]                          |
| Output:     p = 256                                     |
+---------------------------------------------------------+
```

Web app components (Yew functional components):

- `App` -- root; owns the loaded module and the current
  definition selection.
- `DefinitionPicker` -- list of definitions in the module.
- `FaceSelector` -- six-button face switch.
- `FacetView` -- renders the 2D facet for the chosen face.
  Plain HTML/CSS grid; no canvas in the MVP.
- `ZLayerSlider` -- selects which Z-layer is "front" for
  display (does not change semantics).
- `RunPanel` -- collects inputs, calls into the in-page
  WASM runtime, displays the output.
- `LibraryGrid` (M8) -- 2D grid of definitions sortable by
  arity / type / aspect.
- `PipelinePanel` (M9) -- 2D node graph for composing minted
  verbs.

State management is local component state plus a shared
`AppContext` (Yew `ContextProvider`) holding the parsed module,
the active definition id, and the WASM runtime handle.

### Phase 2 (post-MVP): three.js 3D viewer

A separate JavaScript bundle alongside the Yew app uses three.js
to render the same positioned-symbol set as a 3D voxel scene.
The Yew app exposes the symbol set via a small JS-bridge function
(`extern "C"` over `wasm-bindgen`). The 2D facet viewer remains
the canonical authoring surface; the 3D viewer is for inspection
and demonstration.

## 7. Facet consistency model

A definition is **valid** only if its facets agree. The checker
runs after parsing and before IR generation. Inconsistencies are
reported with face identifiers and source coordinates.

Consistency rules (initial set):

| Rule | Meaning |
|---|---|
| **R1 input arity** | Front face's signature input arity equals the count of named inputs on Left. |
| **R2 output arity** | Front face's signature output arity equals the count of named outputs on Right. |
| **R3 reachability** | Every input named on Left appears in the implementation; every output named on Right is produced. |
| **R4 precondition shape** | Top-face conditions reference only inputs (Left-named) or pure constants. |
| **R5 postcondition shape** | Bottom-face conditions reference outputs (Right-named) and may reference inputs. |
| **R6 rear lowering** | If Rear contains generated code, that code is recomputable from the implementation; tooling regenerates and diffs. |
| **R7 occlusion** | If two glyphs project to the same (u, v) on a face, the nearest wins for parsing; tooling warns when occlusion is meaningful. |

R1-R3 are **mandatory** in the MVP. R4-R6 are checked when the
relevant face is present; absent faces produce no warnings. R7 is
a tooling concern, not a hard error.

## 8. Aspect-oriented programming hook

Aspects (preconditions, postconditions, tracing, profiling, error
recovery, debug breakpoints) are **separate Z-layers or named
internal aspects** attached to a definition. The compiler weaves
selected aspects into the IR via configurable passes. The user
can toggle aspects on or off without touching the front facet.

Aspect categories (initial):

```text
pre        compile-time check + runtime guard
post       compile-time check + runtime guard
around     wrap call control flow
on-error   error handling block
trace      logging / execution trace
debug      breakpoints / watchpoints
profile    timing / counters
macro      compile-time rewriting
emit       target-code generation hint
test       examples and assertions
```

## 9. Library / dictionary view

A library is a directory of `.d1` files plus a manifest. The
Yew app renders the library as a sortable grid in M8 / M9. Sort
keys: input arity, output arity, input type, output type, aspect
category, name. Filter keys: presence of preconditions, presence
of postconditions, ability to compose with a chosen output type.

## 10. Pipeline / network model (post-MVP)

A pipeline is a graph whose nodes are minted verbs and whose
edges carry tuples between them. DiscoveryOne adds time as a fourth
dimension for execution traces (which node fired, which tuple
flowed, which precondition failed). The MVP renders pipelines as
2D node graphs (M9). True-3D pipeline visualization is a later
milestone alongside the three.js viewer.

## 11. Test harness

`reg-rs` (already on the user's PATH) is the spine. Every
language feature, every CLI subcommand, and every deterministic
web demo gets a `*.rgt` definition and a `*.out` baseline
committed under `work/reg-rs/`. The CLI is intentionally
non-interactive so that every test is a single invocation that
produces deterministic stdout.

Web demos are tested by the CLI driving wasmtime against the same
emitted WASM modules the browser would run, so the runtime path
is regression-tested without a browser. Browser-side end-to-end
tests use Playwright MCP (already documented in
`docs/ai_agent_instructions.md`) and are layered on top of the
core reg-rs corpus, not in place of it.

## 12. Non-architecture (deferred)

These are deliberately out of scope for the architecture spec.
They will get their own design notes when they become near-term:

- Hindley-Milner type inference (DiscoveryOne inherits Tuplet's nominal
  / positional / arity-only typing).
- Garbage collection. Values are stack cells.
- Modules and namespaces as a first-class language feature.
- Hygienic macros (template expansion uses Tuplet's longest-match
  rule for the MVP).
- Self-hosting (Rust hosts the parser; DiscoveryOne hosting itself is
  a far-future saga).
- Persistent server-side state. The MVP runs entirely client-side.

## 13. Cross-references

- `docs/prd.md` -- product requirements, personas, MVP scope.
- `docs/design.md` -- data structures, file formats, IR, web UI
  components in detail.
- `docs/plan.md` -- TDD/demo milestones with reg-rs baselines.
- `docs/status.md` -- current snapshot.
- `docs/research.txt` -- the source vision packet from the user.
- `../tuplet/docs/` -- Tuplet's PRD, design, grammar, lowering;
  consult before changing equivalent DiscoveryOne semantics.
