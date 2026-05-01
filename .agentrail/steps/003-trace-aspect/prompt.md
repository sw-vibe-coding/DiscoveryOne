# Step: trace-aspect

Implement the narrow trace aspect demo.

Scope:
- Make the trace fixture produce deterministic step/value output for a known Power-style run.
- Keep formatting stable for reg-rs and reuse existing runtime paths where practical.
- Do not implement general tracing for arbitrary programs beyond the demo fixture.

Verification:
- Make the trace aspect baseline green.
- Run profile/on-error baselines to confirm they remain at their expected state, plus relevant runtime/web tests.

Commit message: aspects: add trace output