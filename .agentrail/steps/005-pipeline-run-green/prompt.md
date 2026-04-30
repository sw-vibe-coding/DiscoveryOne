# Step: pipeline-run-green

Execute the small M9 pipeline and make the acceptance baseline green.

Scope:
- Route the bundled Power -> Output pipeline through the existing Web runtime path.
- Use inputs n=2,e=8 and output 256 for the acceptance case.
- Preserve existing Power, front-edit, minted, and library grid demos.
- Keep execution narrow to the bundled pipeline fixture.

Verification:
- Make the pipeline RED acceptance pass.
- Run existing front-edit, Power, minted, library grid, and web crate tests.

Commit message: web: run pipeline fixture