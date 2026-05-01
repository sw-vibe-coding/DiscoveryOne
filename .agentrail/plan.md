# Saga: discoveryone-aspects

Goal: implement M11: a narrow aspect-weaving demo where trace, profile, and on-error aspects can be represented, toggled, checked, and observed in deterministic CLI/Web outputs.

Milestones:
1. Add RED acceptance baselines for trace, profile, and on-error aspect behavior on small deterministic fixtures.
2. Model aspect declarations and toggles narrowly in the parser/source path without designing a full aspect language.
3. Implement trace output for a known Power-style run and make the trace baseline green.
4. Implement profile counters for a small loop and make the profile baseline green.
5. Implement on-error handling for a deliberate fault fixture and make the on-error baseline green.
6. Expose aspect toggles/status in the Web UI without disrupting M7-M10 demos.
7. Update docs/status for the M11 aspects demo and close the saga.

Non-goals for M11: no generalized macro/aspect compiler, no cross-module weaving, no production profiler, and no broad exception system beyond the demo fixtures.