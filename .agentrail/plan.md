# Saga: discoveryone-power-front-view-demo

Goal: make the Web UI Power front facet demo render the structured mathematical view the user requested, then make the demo usable enough to inspect, edit, rerun, and diagnose source problems.

Target front view:
                                      ⎧ 1 →
   • Power (n : ℤ  e : ℤ) → (p : ℤ) ← ⎨ loop e times    iff e is positive
                                      ⎩ n (×) →

Milestones:
1. Capture the current Power front view as a failing regression baseline for the desired notation.
2. Implement the Power front facet renderer so the Web UI shows the structured notation.
3. Add snapshot/acceptance coverage for the rendered Web UI output.
4. Improve the demo layout so the facet/editor area uses horizontal space and the Front panel is not crowded.
5. Wire build metadata through build.rs so CLI -V and the Web footer expose the same useful build info.
6. Make the demo controls real: definition selection, editable n/e inputs, and rerun behavior.
7. Add browse vs edit mode for source/facet editing.
8. Add invalid-source feedback with visible error messages and source highlighting.
9. Update demo documentation/status and verify the localhost:1078 demo.