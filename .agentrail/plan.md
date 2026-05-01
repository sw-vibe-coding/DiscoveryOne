# Saga: discoveryone-3d-viewer

Goal: implement M10: a Yew/Rust/WASM web demo that exposes positioned DiscoveryOne symbols to a three.js inspection scene and renders the Power definition as an inspectable 3D view.

Milestones:
1. Add a RED acceptance baseline for a deterministic 3D viewer fixture and document the expected demo surface.
2. Export the same positioned symbol set used by the Yew facet view through a narrow wasm-bindgen/JSON boundary.
3. Add a three.js scene bundle that reads the exported symbol set and renders positioned glyph blocks for Power.
4. Embed the 3D viewer in the Yew Web UI with a controlled fallback and stable demo controls.
5. Add verification/smoke coverage for the exported scene data and viewer asset wiring, plus a manual demo script.
6. Update docs/status for the M10 3D viewer demo and close the saga.

Non-goals for M10: no 3D authoring, no pipeline editing in 3D, no arbitrary camera persistence, and no general replacement for the 2D facet editor.