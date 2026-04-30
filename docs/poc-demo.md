# Front Edit Execution Demo

This is the current Web demo slice. It demonstrates that
DiscoveryOne can run the existing `Power` fixture and two user-minted
syntax fixtures, present a sortable library grid, and execute a narrow
Power Front 2D edit.

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
