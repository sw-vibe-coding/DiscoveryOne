# Front Edit Execution Demo

This is the current Web demo slice. It demonstrates that
DiscoveryOne can run the existing `Power` fixture and two user-minted
syntax fixtures, present a sortable library grid, execute a narrow
Power Front 2D edit, and run a bundled 2D `Power -> Output`
pipeline. It also includes the M10 Power 3D viewer inspection panel.

## Build

```bash
./scripts/build-all.sh
sw-install --project .
```

## CLI Slice

Run Power:

```bash
scripts/run-fixture.sh run examples/power.d1 --inputs n=2,e=8
```

Expected output:

```text
256
```

Run the minted dowhile syntax fixture:

```bash
scripts/run-fixture.sh run examples/dowhile.d1
```

Expected output:

```text
1 2 3
3
```

Run the alternate unless/do syntax fixture:

```bash
scripts/run-fixture.sh run examples/unless.d1
```

Expected output:

```text
1 2 3
3
```

## Web Demo

Start the web app:

```bash
./scripts/run-web.sh 1078
```

Open `http://localhost:1078/`.

Use the definition picker to select:

- `Power` for the Power front-view notation and RunPanel path.
- `DowhileCounter` for the `do _ while _ end` minted syntax demo.
- `UnlessCounter` for the alternate `unless _ do _ end` template.

For `Power`, select the Front facet. The facet renders the
structured notation:

```text
                                      ⎧ 1 →
   • Power (n : ℤ  e : ℤ) → (p : ℤ) ← ⎨ loop e times    iff e is positive
                                      ⎩ n (×) →
```

To demo executable Front editing:

1. Click `Edit` in the Power Front facet.
2. Change the top branch zero-case value from `1` to `2`.
3. Set RunPanel inputs to `n=5` and `e=0`.
4. Click `Run`.

Expected output:

```text
2
```

This edit support is intentionally narrow. The Web runtime reads the
edited Power Front zero-case integer for `e=0`; it does not yet
reverse-project arbitrary 2D facet edits back into full `.d1` source.
Malformed Power Front edits produce an explicit RunPanel error instead
of silently falling back to the bundled source.

For the minted syntax demos, the selected definition opens on the
Internal facet. The facet shows the syntax template declaration, and
the RunPanel renders:

```text
1 2 3
3
```

Use the Library grid below the facet and run panels to inspect the
bundled definitions. The M8 snapshot contains six rows:

- `Add`
- `Clamp`
- `Power`
- `DowhileCounter`
- `UnlessCounter`
- `TraceValue`

The grid can be sorted by `Name`, `Arity`, `Type`, and `Aspects`.
Each sort path is stable, with definition name as the tie-breaker.

## 2D Pipeline Demo

Below the library grid, the Web UI renders the bundled M9 pipeline:

```text
Power.p -> Output.value
```

The pipeline panel shows two positioned nodes, the `Power` output
port `p : Z`, the `Output` input port `value : Z`, and the inline
validation message:

```text
Valid: Power.p feeds Output.value.
```

The bundled fixture runs through the existing Web runtime with
inputs `n=2,e=8`, so the pipeline output is:

```text
256
```

This is intentionally a narrow demo fixture. The canvas is
deterministic and type-checked for the bundled edge, but it does not
yet support drag/drop authoring, persistent pipeline files, editable
connections, or general multi-node execution.

## 3D Viewer Demo

The Web UI renders a `3D Viewer` panel for `Power` between the
Library grid and the 2D pipeline canvas. The panel uses the same
positioned symbol data as the facet view. On load, the Yew app passes
the Power symbol export into `viewer3d.js`, which initializes a
three.js scene and renders one glyph block per exported symbol.

The exported data is deterministic and can be inspected without a
browser GPU:

```bash
scripts/snapshot-web-3d-symbols.sh
```

Expected data behavior:

```text
"definition": "Power"
"symbol_count": 54
{"text":"loop","face":"Front","x":0,"y":2,"z":0}
```

The browser demo path is:

1. Run `./scripts/run-web.sh 1078`.
2. Open `http://localhost:1078/`.
3. Find the `3D Viewer` panel.
4. Confirm the loading fallback is replaced by a non-empty canvas.

This viewer is inspection-only. It does not support 3D authoring,
selection editing, camera persistence, or pipeline editing in 3D.

## Regression Baseline

The full M7 acceptance case is:

```bash
reg-rs run -p d1_minted_module_acceptance
```

The Power front-view demo acceptance case is:

```bash
reg-rs run -p d1_power_front_view_acceptance
```

The executable Power Front edit acceptance case is:

```bash
reg-rs run -p d1_web_power_front_edit_run
```

The complete suite should report all cases green.

The M8 library-grid acceptance case is:

```bash
reg-rs run -p d1_library_grid_acceptance
```

The M9 pipeline acceptance case is:

```bash
reg-rs run -p d1_web_pipeline_power_output
```

The M10 3D viewer acceptance case is:

```bash
reg-rs run -p d1_3d_viewer_acceptance
```
