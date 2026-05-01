# Step: on-error-aspect

Implement the narrow on-error aspect demo.

Scope:
- Add deterministic handling for a deliberate fault fixture.
- Surface the caught error and fallback/result exactly as defined by the M11 baseline.
- Do not build a broad exception system beyond the fixture.

Verification:
- Make the on-error aspect baseline green.
- Run trace/profile baselines, M10 acceptance, and relevant runtime/web tests.

Commit message: aspects: add on-error handling