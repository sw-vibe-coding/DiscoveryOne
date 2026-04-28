# Step: cli-version-build-info

Add real build metadata to `d1 -V` / `d1 --version`.

## Work

1. Follow the local `../sw-cli-tools` CLI convention of using `build.rs`
   to capture build host, Git commit, and build time.
2. Replace the placeholder `unknown` build fields in `d1` version output.
3. Rebuild and install `d1` if `sw-checklist` reports binary freshness
   drift.
4. Verify `d1 -V` prints non-placeholder build metadata.

## Commit

Associated maintenance commit:

`fix: add d1 build version info`
