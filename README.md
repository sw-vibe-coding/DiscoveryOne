# DiscoveryOne

An experimental visual programming language whose source is a
positioned collection of glyphs in 3D space. The same definition
is **viewed from multiple facets** -- each surfacing one semantic
aspect of the same minted verb.

![DiscoveryOne badge](images/discovery-one-badge.png)

## Intro

DiscoveryOne is the successor to
[Tuplet](https://github.com/softwarewrighter/tuplet), a
2D-layout-sensitive language with first-class named tuples,
multi-output verbs, and user-mintable verbs/syntax via the `*`
operator. Tuplet runs on an OCaml-subset host and compiles to
Forth on the COR24 runtime.

DiscoveryOne keeps Tuplet's mechanism (`*` mints everything; the
kernel is tiny; control flow is library code) and extends it in
three ways:

- **Source becomes 3D.** Every glyph has an `(x, y, z)` coordinate
  and an aspect label (`@front`, `@left`, `@right`, `@top`,
  `@bottom`, `@rear`, or `@internal`). A definition is a small
  cube of meaning, not a flat block of text.
- **Host becomes Rust.** Lexer, parser, checker, IR, interpreter,
  and emitter are all native Rust crates -- compiled to native
  for the test CLI and to WebAssembly for the web app.
- **Runtime becomes WASM.** DiscoveryOne emits WAT, assembles to
  a `.wasm` module, and runs it in the browser (via the Yew SPA)
  or under `wasmtime` (for regression tests).

The web app is the authoring and inspection surface. The CLI
exists only to drive the regression-test corpus.

## Overview

A `*Power` definition might look like this:

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

  @internal
    @z 0  p <- 1
    @z 1  counter <- 0
    @z 2  while counter < e: p <- p * n; counter <- counter + 1
```

The Yew web app loads the file, projects it onto each facet, and
runs the WASM module in-page when the user clicks **Run**.

| Facet | Role |
|---|---|
| Front | Algorithm gist -- the readable story |
| Left | Inputs (arity, names, types) |
| Right | Outputs (arity, names, types) |
| Top | Preconditions |
| Bottom | Postconditions |
| Rear | Generated form (WAT or stack IR) |
| Internal | Implementation in spatially-separable layers |

Aspects (preconditions, postconditions, tracing, profiling, error
recovery) live in spatially separate layers so the front facet
stays uncluttered for reading.

## Status

**Pre-M0.** This repository holds the specification only. No
code has been written yet. See `docs/status.md` for the live
snapshot and `docs/plan.md` for the milestone arc.

The single demoable target -- the **minted module milestone**
(M7 in the plan) -- is a vertical slice in which the user
authors a `*Power` definition and a `*syntax do _ while _ end
expand` declaration, and runs both inside the web app, with
every step baselined by [`reg-rs`](https://github.com/softwarewrighter/reg-rs).

## Quickstart (planned)

Once M0 lands, the workflow will be:

```bash
# 1. Build everything (CLI + WASM runtime + Yew web app).
./scripts/build-all.sh

# 2. Run the dev server.
./scripts/run-web.sh 9735

# 3. Open http://localhost:9735/
```

Pick a definition from the dropdown, click through the facet
buttons (Front / Left / Right / Top / Bottom / Rear), then click
**Run** in the Run panel.

The CLI is the regression-test driver:

```bash
d1 lex examples/power.d1
d1 parse examples/power.d1
d1 face examples/power.d1 --face Front
d1 check examples/power.d1
d1 lower examples/power.d1
d1 run examples/power.d1 --inputs n=2 e=8     # prints 256
```

## Architecture

- `crates/d1-source` -- voxel loader; `.d1` and `.d1.json` round-trip.
- `crates/d1-geom` -- projection, occlusion, aspect picking.
- `crates/d1-lex` -- Rust port of Tuplet's lexer.
- `crates/d1-parse` -- AST per `docs/design.md`.
- `crates/d1-check` -- name resolution, arity, facet consistency.
- `crates/d1-ir` -- normalized stack IR.
- `crates/d1-interp` -- reference Rust interpreter (test oracle).
- `crates/d1-emit-wasm` -- WAT emitter + assembler.
- `crates/d1-cli` -- the `d1` CLI; the only crate that owns I/O.
- `ui/web` -- Yew SPA (Trunk-built) hosting the WASM runtime in-page.

See `docs/architecture.md` for the full picture and
`docs/design.md` for component-level detail.

## Tests

```bash
cargo test
REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_smoke_cli_help
```

Reg-rs baselines live under `work/reg-rs/*.rgt` and `*.out`
(both tracked in git); `*.tdb` and `*.lock` files are
gitignored. Every milestone in `docs/plan.md` adds a named
subset of cases; no milestone closes until its baselines are
green.

## Documentation

- [`docs/architecture.md`](docs/architecture.md) -- system architecture and pipeline.
- [`docs/prd.md`](docs/prd.md) -- product requirements, personas, use cases.
- [`docs/design.md`](docs/design.md) -- file formats, AST, IR, CLI, web components.
- [`docs/plan.md`](docs/plan.md) -- TDD/demo milestone arc M0-M11.
- [`docs/status.md`](docs/status.md) -- current snapshot.
- [`docs/process.md`](docs/process.md) -- TDD red/green/refactor and pre-commit gates.
- [`docs/research.txt`](docs/research.txt) -- the source vision packet.

## Related projects

- [Tuplet](https://github.com/softwarewrighter/tuplet) -- the
  2D ancestor; OCaml host + Forth runtime on COR24.
- [reg-rs](https://github.com/softwarewrighter/reg-rs) -- the
  golden-output regression harness used throughout.
- [web-sw-cor24-ocaml](https://github.com/sw-embed/web-sw-cor24-ocaml)
  -- the Yew + Trunk + COR24 web-app pattern this project
  mirrors for build, serve, and Pages deploy scripts.

## Links

- Blog: [Software Wrighter Lab](https://software-wrighter-lab.github.io/)
- Discord: [Join the community](https://discord.com/invite/Ctzk5uHggZ)
- YouTube: [Software Wrighter](https://www.youtube.com/@SoftwareWrighter)

## Copyright

Copyright (c) 2026 Michael A. Wright

## License

MIT. (C) 2026 Michael A Wright. See [LICENSE](LICENSE).
