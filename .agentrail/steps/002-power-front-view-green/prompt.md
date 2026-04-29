# Step: power-front-view-green

Implement the Power front facet renderer needed to pass the RED baseline.

Scope:
- Update the existing Web UI/front facet rendering path for Power.
- Render the structured mathematical view:
                                      ⎧ 1 →
   • Power (n : ℤ  e : ℤ) → (p : ℤ) ← ⎨ loop e times    iff e is positive
                                      ⎩ n (×) →
- Keep the change scoped to the Power front-view demo path unless the existing renderer already has a shared abstraction that cleanly supports this.
- Preserve existing internal/run behavior for Power, DowhileCounter, and UnlessCounter.

Verification:
- Run the targeted Power front-view regression and relevant web tests.

Commit message: web: render power front view notation