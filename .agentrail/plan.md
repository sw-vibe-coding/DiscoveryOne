# DiscoveryOne M6 -- Web Shell

Goal: Yew SPA loads a .d1 file, displays Power facets, switches between facets, and runs the loaded .d1 through the web-side runtime.

Reg-rs cases:
- d1_facet_snapshot_power_front
- d1_facet_snapshot_power_left
- d1_facet_snapshot_power_right
- d1_facet_snapshot_power_top
- d1_facet_snapshot_power_bottom
- d1_facet_snapshot_power_rear
- d1_web_facet_front_html_snapshot
- d1_web_run_power_2_8

Every step must rebuild and reinstall d1 before fixture/reg-rs checks when d1 changes, keep sw-checklist clean with zero failures and zero warnings, commit .agentrail/ changes, and push.