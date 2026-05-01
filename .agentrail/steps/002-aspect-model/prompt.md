# Step: aspect-model

Add the narrow M11 aspect data model and fixture loading support.

Scope:
- Represent trace/profile/on-error aspect declarations or toggles for the M11 fixtures.
- Thread the data through the existing parser/source/check path only as far as needed by the baselines.
- Avoid a generalized aspect language or cross-module weaving design.

Verification:
- Add focused tests/snapshots for aspect fixture loading.
- Run the aspect RED baselines, current M10 acceptance, and relevant crate tests.

Commit message: aspects: model demo toggles