# DiscoveryOne -- Learnings

Operational notes from quality gates and agent runs.

## 2026-04-26 -- M0 sw-checklist Gaps

What went wrong?

`sw-checklist` passes the Rust structure checks but fails M0 on
Software Wrighter packaging expectations that were outside the
scaffold step:

- `ui/web` has no `favicon.ico`.
- `d1 --help` does not yet have a longer AI-agent section.
- `d1 --version` does not yet include copyright, license,
  repository, build host, build commit, or build time fields.
- The `d1` binary has not been installed with `sw-install`.

Why was it not caught sooner?

The M0 plan intentionally deferred favicon and build-info
generation, and the CLI scaffold only promised a stable `--help`
banner for the first reg-rs baseline.

Process change:

When a future saga touches CLI metadata, web packaging, or release
installation, run `sw-checklist --verbose` early and either satisfy
the Software Wrighter metadata contract in that step or keep the
failure listed here with a clear deferral.
