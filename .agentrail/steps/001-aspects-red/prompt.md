# Step: aspects-red

Create the M11 RED acceptance baselines for aspect behavior.

Scope:
- Add small deterministic fixtures and reg-rs baselines for trace, profile, and on-error aspect behavior.
- Baselines should describe expected CLI/Web observable output before implementation.
- Keep the cases narrow and tied to known Power/small-loop/fault fixtures.
- Do not implement aspect parsing or runtime behavior yet.

Verification:
- Run the new reg-rs cases and confirm they fail for the expected missing behavior.
- Run current M10 acceptance and core web regressions to confirm existing demos remain green.

Commit message: test: add aspect red baselines