# Step: front-edit-parse

Parse the narrow Power Front 2D edit into preview semantics.

Scope:
- Add a small parser/extractor for the current visible Power Front 3-line notation.
- Extract the zero-exponent base literal from the top branch.
- Accept the current unedited value 1 and edited integer values such as 2.
- Return useful validation errors when the required Power Front tokens are missing or malformed.
- Keep this Power Front-specific; do not build general reverse projection.

Verification:
- Add focused unit tests for extracting 1 and 2 and rejecting malformed Power Front text.
- Keep existing web tests green.

Commit message: web: parse power front edit semantics