# M7 PoC Demo

This is the M7 minted-module vertical slice. It demonstrates that
DiscoveryOne can run the existing `Power` fixture and two user-minted
syntax fixtures from both the CLI regression path and the Yew web demo
snapshot path.

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

- `Power` for the M6 power facet and RunPanel path.
- `DowhileCounter` for the `do _ while _ end` minted syntax demo.
- `UnlessCounter` for the alternate `unless _ do _ end` template.

For the minted syntax demos, the selected definition opens on the
Internal facet. The facet shows the syntax template declaration, and
the RunPanel renders:

```text
1 2 3
3
```

## Regression Baseline

The full M7 acceptance case is:

```bash
reg-rs run -p d1_minted_module_acceptance
```

The complete suite should report all cases green.
