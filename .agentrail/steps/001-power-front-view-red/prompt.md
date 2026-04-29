# Step: power-front-view-red

Create the RED regression baseline for the Power front facet notation demo.

Scope:
- Inspect the existing Web UI/front facet snapshot flow for Power.
- Add or update the smallest reg-rs/snapshot test that captures the desired Power front view rendering.
- The expected target is:
                                      ⎧ 1 →
   • Power (n : ℤ  e : ℤ) → (p : ℤ) ← ⎨ loop e times    iff e is positive
                                      ⎩ n (×) →
- Do not implement the renderer in this step; leave the new test failing for the current plain-text front view.

Verification:
- Run only the targeted failing command and record the failure output.

Commit message: test: add power front view red baseline