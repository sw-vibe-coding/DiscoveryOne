# DiscoveryOne -- Design

Low-level design for DiscoveryOne. Complements `docs/architecture.md`
(big picture) and `docs/prd.md` (what the product is).

## 1. Source file formats

DiscoveryOne accepts two equivalent source forms.

### 1.1 Layered text (`.d1`) -- human-edited

A header section names the definition, then aspect-tagged blocks
hold the content of each facet. Indent inside a block defines the
2D layout of the symbols on that face; an explicit `@z N` tag
inside the block sets the depth (Z-coordinate).

```text
*Power
  @front
    n e -> p
    p <- 1
    loop e times: p <- p * n

  @left
    n : Int
    e : Int

  @right
    p : Int

  @top
    e >= 0

  @bottom
    p == n^e

  @rear wasm
    (regenerated; do not hand-edit)

  @z 0 @internal
    p <- 1
  @z 1 @internal
    counter <- 0
  @z 2 @internal
    while counter < e:
      p <- p * n
      counter <- counter + 1
```

Rules:

- One file may contain multiple definitions; each begins with a
  mint operator (`*`) and a name on column 0.
- Aspect tags (`@front`, `@left`, `@right`, `@top`, `@bottom`,
  `@rear`, `@internal`) introduce a block. The content of the
  block extends until the next tag or the next `*name` header.
- `@z N` sets the Z-coordinate for the symbols that follow until
  another `@z` or end-of-block. The default is `@z 0`.
- Inside a block, columns map to X, lines map to Y; the loader
  trims leading indentation common to the block.
- Comments use `#` to end-of-line, identical to Tuplet.

### 1.2 Canonical JSON (`.d1.json`) -- machine form

```json
{
  "name": "Power",
  "definitions": [
    {
      "name": "Power",
      "symbols": [
        { "id": 1, "text": "Power", "x": 0, "y": 0, "z": 0,
          "aspect": "Front" },
        { "id": 2, "text": "n",     "x": 0, "y": 1, "z": 0,
          "aspect": "Left" },
        { "id": 3, "text": ":",     "x": 2, "y": 1, "z": 0,
          "aspect": "Left" },
        { "id": 4, "text": "Int",   "x": 4, "y": 1, "z": 0,
          "aspect": "Left" }
      ]
    }
  ]
}
```

The two formats are loss-free round-trippable. The CLI command
`d1 normalize` round-trips `.d1` -> `.d1.json` -> `.d1` and
diffs against the original; reg-rs baselines this.

## 2. Coordinates and projection

A symbol has an integer `(x, y, z)` triple plus an `aspect`
label. Coordinates are integers; there is no sub-cell positioning
in the MVP.

### 2.1 Projection by face

| Face | (u, v, depth) from (x, y, z) |
|---|---|
| Front | (x, y, z) |
| Rear | (-x, y, -z) -- flipped horizontally |
| Left | (z, y, -x) |
| Right | (-z, y, x) -- flipped horizontally |
| Top | (x, z, -y) |
| Bottom | (x, -z, y) -- flipped vertically |

`u`, `v` are the 2D facet coordinates; `depth` orders glyphs from
nearest (smallest) to farthest. Negative coordinates are allowed
internally; the renderer translates to a non-negative grid for
display.

### 2.2 Occlusion

Two symbols at the same `(u, v)` form a stack. The default rule
is **nearest wins**: only the symbol with the smallest `depth`
contributes to the parsed token stream for that face. The full
stack is preserved on the side and exposed to tooling for the
"show all layers" debug mode.

The checker emits a warning when occlusion is meaningful (the
hidden glyph is not whitespace and not an annotation).

## 3. Lexical rules

DiscoveryOne reuses Tuplet's lexer with minor adjustments. Refer
to `../tuplet/docs/grammar.md` and `../tuplet/docs/lexer.md` for
the inherited specification. The token set (Rust enum):

```rust
pub enum Token<'src> {
    Hash(&'src str),         // # comment text
    Ident(&'src str),
    Int(i64),
    Pct(u8),                 // 0..=100
    Mint,                    // *
    LArrow,                  // <-
    RArrow,                  // ->
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Underscore,
    Minus,
    AspectTag(AspectKind),   // @front, @left, ...
    ZTag(i32),               // @z N
    Eof,
    Unknown(u8),
}
```

Differences from Tuplet:

- `AspectTag` and `ZTag` are new and only valid at the start of a
  line in `.d1` source.
- The lexer is byte-oriented but uses `&str` slices because Rust
  has zero-cost UTF-8 string handling, unlike the OCaml-subset
  host. Unicode aliases from Tuplet (`U+2022` for `*`, `U+2190`
  for `<-`, etc.) are still supported and folded to the ASCII
  spelling before the parser sees them.

## 4. AST

```rust
pub type Name = String;

pub enum Op { Add, Sub, Mul, Max, Min, Div, Max2, Min2, Div2 }

pub enum Expr {
    Int(i64),
    Bool(bool),
    Pct(u8),
    Symbol(String),
    Name(Name),
    Binary(Box<Expr>, Op, Box<Expr>),
    Call(Name, Vec<Expr>),
    Tuple(Vec<Expr>),
}

pub enum Pattern { Name(Name), Tuple(Vec<Name>) }

pub struct Field { pub name: Name }

pub struct SigDecl {
    pub name: Name,
    pub inputs: Vec<Field>,
    pub outputs: Vec<Field>,
}

pub struct TupDecl {
    pub name: Name,
    pub fields: Vec<Field>,
}

pub enum Stmt {
    Decl(TupDecl),
    Signature(SigDecl),
    Assign(Pattern, Expr),
    Expr(Expr),
    Comment(String),
}

pub struct Facet {
    pub aspect: Aspect,
    pub stmts: Vec<Stmt>,
    pub coords: Vec<Coord>,   // per-statement source coords
}

pub struct Definition {
    pub name: Name,
    pub facets: HashMap<Aspect, Facet>,
    pub symbols: Vec<Symbol>, // raw positioned-symbol set
}

pub struct Module {
    pub source_path: PathBuf,
    pub definitions: Vec<Definition>,
    pub registry: SyntaxRegistry,
}
```

The AST is the same shape as Tuplet's (`../tuplet/docs/design.md`),
plus a `facets` map keyed by `Aspect` and per-statement source
coordinates so error messages can carry `(x, y, z, aspect)`.

## 5. Stack IR

Reuses Tuplet's IR with added `IPrimWasm` for raw escapes into
hand-written WAT. `IPrimForth` is dropped; DiscoveryOne does not
target Forth.

```rust
pub enum Instr {
    PushInt(i64),
    PushBool(bool),
    PushPct(u8),
    PushSymbol(String),
    Load(Name),
    Store(Name),
    LoadTuple(Name, u32),
    StoreTuple(Name, u32),
    Call(Name, u32),
    BinOp(Op),
    PrimWasm(String),       // raw WAT escape
    NoNameStart,            // anonymous-verb thunk start
    NoNameEnd,
}
```

Stack effects mirror Tuplet's table in `../tuplet/docs/design.md`.
The interpreter and the WASM emitter consume the same IR; a single
gold output is produced and reg-rs compares interpreter and WASM
runs.

## 6. Facet checker

Checks run after parsing, before IR generation. The checker
walks each definition's facets and applies the consistency rules
from `docs/architecture.md` section 7. Errors carry
`(definition, aspect, x, y, z)`.

Checker output (success): a `CheckedModule` carrying the AST plus
a per-definition `FacetReport` (which rules ran, which faces were
present, any warnings).

Checker output (failure): a list of `CheckError`, each with code
(`E001` ... `E099`), face, coordinate, message text. Error codes
are stable across releases so reg-rs baselines are robust.

## 7. Reference interpreter

`d1-interp` walks the IR over a `Vec<Cell>` operand stack. `Cell`
is `i64` for the MVP. The interpreter's only purpose is to be a
test oracle: every program in the corpus runs through the
interpreter and the WASM emitter, and the outputs must agree
modulo a documented divergence policy (none for the MVP).

## 8. WASM emitter

Two strategies considered; the MVP picks (a):

(a) **Emit WAT, assemble with `wat` crate.** The emitter produces
    a `String` of WebAssembly Text Format; the `wat` crate
    converts it to bytes. Pros: easy to inspect (the Rear facet
    can show the WAT directly), no FFI complexity. Cons: slower
    than direct binary emission.

(b) Emit binary directly via `wasm-encoder`. Faster but harder to
    diff and visualize. Defer until the WAT path is too slow.

Stack-IR-to-WAT lowering rules (initial sketch):

| IR | WAT |
|---|---|
| `PushInt n` | `i64.const n` |
| `PushBool b` | `i32.const 1` (true) or `i32.const 0` |
| `PushPct p` | `i64.const p` |
| `Load x` | `local.get $x` |
| `Store x` | `local.set $x` |
| `LoadTuple t,n` | `local.get $t_0 ... local.get $t_{n-1}` |
| `StoreTuple t,n` | `local.set $t_{n-1} ... local.set $t_0` |
| `BinOp Add` | `i64.add` |
| `BinOp Mul` | `i64.mul` |
| `Call v,n` | `call $v` |
| `PrimWasm s` | inline `s` verbatim |

Anonymous-verb thunks become separate `(func ...)` definitions
named via a counter; the thunk's index is pushed via `i32.const`
and called via `call_indirect`.

## 9. CLI surface

The `d1` CLI is the regression-test driver. Subcommands:

| Subcommand | Purpose |
|---|---|
| `d1 lex FILE` | Dump token stream to stdout (one token per line). |
| `d1 parse FILE` | Dump AST as deterministic s-expression. |
| `d1 face FILE --face F` | Project to face F and dump 2D token grid. |
| `d1 check FILE` | Run facet checker; print pass/fail and any errors. |
| `d1 lower FILE` | Print stack IR. |
| `d1 emit-wasm FILE -o OUT.wasm` | Emit WASM module. |
| `d1 run FILE [--inputs ...]` | Execute via WASM (wasmtime); print output. |
| `d1 interp FILE [--inputs ...]` | Execute via reference interpreter. |
| `d1 normalize FILE` | Round-trip `.d1` -> `.d1.json` -> `.d1`; print or diff. |
| `d1 facet-snapshot FILE --face F` | Render one facet to deterministic ASCII (for reg-rs). |

All subcommands take input files via positional arguments and
print to stdout. None prompt interactively. None write to the
filesystem unless `-o` is given.

## 10. Web app components (Yew)

Component tree (MVP):

```text
App
+- TopBar
|   +- DefinitionPicker
|   +- FaceSelector
+- MainPane
|   +- FacetView
|   +- ZLayerSlider
+- RunPanel
+- Footer (build stamp, version)
```

Props and events:

- `App` owns `Module`, `current_def: Name`, `current_face: Aspect`,
  `current_z: Option<i32>`. State changes flow via callback props.
- `FacetView` receives `(symbols, face, current_z)` and renders an
  HTML grid of monospace cells. Each cell is one glyph or empty.
- `RunPanel` calls `wasm_runtime::run(module, def, inputs)` and
  displays the result. The runtime is the same WASM module the
  CLI tests against.
- `Footer` displays a build stamp written by `build.rs` (mirrors
  the pattern in `~/github/sw-embed/web-sw-cor24-ocaml`).

State persistence: none in the MVP. Reload restarts.

## 11. Test corpus

reg-rs cases are organized by area. Naming convention:
`d1_<area>_<case>` where area is `lex`, `parse`, `face`, `check`,
`ir`, `interp`, `wasm`, `web`, `cli`.

Initial corpus (MVP):

```text
d1_lex_arity_suffix
d1_lex_mint_operator
d1_lex_aspect_tags
d1_lex_zlayer_tags
d1_parse_mint_init           (*n <- 0)
d1_parse_mint_signature      (*power(n e) -> (p) <- ...)
d1_parse_facet_blocks        (multiple @-tagged blocks)
d1_face_front_power
d1_face_left_power
d1_face_right_power
d1_face_top_power
d1_face_bottom_power
d1_face_rear_power
d1_check_pass_power
d1_check_fail_left_arity_mismatch
d1_check_fail_unbound_name
d1_ir_power_dump
d1_interp_power_2_8_eq_256
d1_wasm_power_2_8_eq_256
d1_wasm_power_5_0_eq_1
d1_normalize_roundtrip_power
d1_cli_face_snapshot_power_front
d1_web_facet_front_html_snapshot
d1_web_run_power_2_8
d1_poc_dowhile_minted_syntax
```

Every case has a `.rgt` definition and a `.out` golden file
under `work/reg-rs/`. The lock files (`*.tdb.lock`) and the
sqlite databases (`*.tdb`) are gitignored.

## 12. Build pipeline

`scripts/build-all.sh`:

1. `cargo build --release -p d1-cli`
2. `cargo build --release --target wasm32-unknown-unknown -p d1-runtime`
3. `wasm-pack build ui/web --target web --release`
4. Generate `dist/build-info.json` with git sha, build timestamp.
5. Stage `ui/web/pkg/*` into `dist/`.

`scripts/run-web.sh PORT`:

- Starts a static file server (Trunk in dev mode for hot reload,
  or `python3 -m http.server` for the simplest path) on PORT
  serving `dist/` (or `ui/web/dist/` under Trunk).

`scripts/run-fixture.sh PATH`:

- Invokes the CLI with the fixture's arguments and prints stdout.
- Intended to be the `command` line in a `.rgt` file.

## 13. Errors and diagnostics

Every error is `(code, severity, source coord, message)`.
Severity is `Error` or `Warning`. Errors with code `E001..E099`
are checker errors; `E100..E199` are parser errors; `E200..E299`
are emitter errors. Codes are stable across releases.

Example:

```text
E007 left-facet input arity mismatch
  at examples/power.d1, definition 'Power', face Left,
  (x=0, y=2, z=0)
  Front declares 2 inputs (n, e); Left lists 3 names.
```

## 14. Open design questions

- **`*` vs `:=` for definitional binding.** Tuplet has the same
  open question (`../tuplet/docs/dsl-examples.md` Q2). DiscoveryOne
  defers to Tuplet's eventual decision; for now `<-` is store and
  `*name(args) -> (outs) <- body` is the only definitional form.
- **Aspect priority for PCA-style relevance.** When more than six
  aspects are attached to a definition, which six surface on the
  faces? The MVP only ships the canonical six; the relevance
  algorithm is deferred.
- **WAT as Rear-facet content vs. a separate display surface.**
  WAT can be long. The MVP renders the first ~30 lines and offers
  a "show full" expansion; the canonical IR (stack form) is also
  selectable as the Rear payload.
- **Pipeline serialization.** Pipelines are not part of the MVP;
  their on-disk format will reuse `.d1.json` extended with
  `connections: [{from, to}]`.

## 15. Cross-references

- `docs/architecture.md` -- system-level architecture.
- `docs/prd.md` -- product requirements.
- `docs/plan.md` -- TDD/demo milestones, including which sections
  of this design land in which milestone.
- `../tuplet/docs/grammar.md`, `../tuplet/docs/design.md`,
  `../tuplet/docs/lowering.md` -- inherited specification.
