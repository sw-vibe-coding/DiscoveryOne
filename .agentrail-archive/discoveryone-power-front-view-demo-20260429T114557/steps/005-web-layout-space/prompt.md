# Step: web-layout-space

Improve the Web UI layout so the demo uses available horizontal space and the Power Front panel is not crowded.

Scope:
- Inspect existing UI CSS/components and local patterns.
- Reduce excessive left/right page margins and make the primary facet/editor area larger.
- Keep the visual style restrained and consistent with the existing app.
- Preserve current Power, DowhileCounter, and UnlessCounter rendering and run behavior.

Verification:
- Run the relevant web snapshot/reg-rs cases and web crate tests.
- Rebuild/restart the local server if needed for manual demo inspection.

Commit message: web: expand demo workspace layout