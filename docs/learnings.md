# DiscoveryOne -- Learnings

Operational notes from quality gates and agent runs.

## 2026-04-26 -- sw-checklist Cleanup Policy

What went wrong?

M0 allowed Software Wrighter packaging failures to accumulate
because they were treated as outside the scaffold step:

- `ui/web` has no `favicon.ico`.
- `d1 --help` does not yet have a longer AI-agent section.
- `d1 --version` does not yet include copyright, license,
  repository, build host, build commit, or build time fields.
- The `d1` binary has not been installed with `sw-install`.

These were later fixed in the M1 cleanup step. `sw-checklist`
now passes with zero failures and zero warnings.

Why was it not caught sooner?

The M0 plan intentionally deferred favicon and build-info
generation, and the CLI scaffold only promised a stable `--help`
banner for the first reg-rs baseline.

Process change:

Run `sw-checklist --verbose` before committing every step. New
failures or warnings are blockers for the current step unless the
user explicitly directs otherwise. Do not accumulate checklist
debt across feature steps.
