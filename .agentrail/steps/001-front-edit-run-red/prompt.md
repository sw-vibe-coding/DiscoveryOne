# Step: front-edit-run-red

Add the RED acceptance baseline for the Power Front edit-to-run demo.

Scope:
- Add a focused Web snapshot/reg-rs path that represents editing the visible Power Front facet base literal from 1 to 2.
- The intended demo is: edit Power Front top branch 1 to 2, run with n=5,e=0, and expect output 2 instead of the original 1.
- Keep this RED: add the failing acceptance/script/example only, without implementing edited execution semantics yet.
- Do not broaden into general reverse projection or persistence.

Verification:
- Run the new reg-rs case and confirm it fails for the expected reason.
- Run existing Power web acceptance to confirm the baseline demo remains green.

Commit message: test: add front edit execution red baseline