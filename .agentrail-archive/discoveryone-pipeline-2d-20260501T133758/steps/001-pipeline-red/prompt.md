# Step: pipeline-red

Create the M9 RED acceptance baseline for a small 2D pipeline demo.

Scope:
- Add a focused reg-rs/web snapshot path for a deterministic 2D pipeline view.
- Start with the smallest useful demo: Power node feeding an Output node, with inputs n=2,e=8 and expected output 256.
- The baseline should describe/render nodes, ports, edge, and run output enough to lock the demo contract.
- Keep this RED: add the failing acceptance/script/example only, without implementing the pipeline model or UI yet.

Verification:
- Run the new reg-rs case and confirm it fails for the expected missing implementation reason.
- Run existing front-edit acceptance to confirm the current demo remains green.

Commit message: test: add pipeline 2d red baseline