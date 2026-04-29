# Step: web-build-info-footer

Wire real build metadata into the Web footer and align it with CLI version output.

Scope:
- Inspect other local web/CLI projects for build.rs patterns that generate git/build metadata.
- Inspect DiscoveryOne CLI -V/current build.rs behavior.
- Implement shared or parallel build metadata so the Web UI footer shows real build host, commit, and build time instead of placeholders.
- Ensure CLI -V and Web footer expose consistent metadata where practical.

Verification:
- Run CLI version/build checks, web crate tests, and relevant snapshots.

Commit message: web: show real build metadata