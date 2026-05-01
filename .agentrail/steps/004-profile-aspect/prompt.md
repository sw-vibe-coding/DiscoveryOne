# Step: profile-aspect

Implement the narrow profile aspect demo.

Scope:
- Add deterministic profile counters for a small loop fixture.
- Surface counts through the same CLI/snapshot path defined by the M11 baseline.
- Keep this as demo instrumentation, not a production profiler.

Verification:
- Make the profile aspect baseline green.
- Run trace and on-error baselines plus relevant runtime/web tests.

Commit message: aspects: add profile counters