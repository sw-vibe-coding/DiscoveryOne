# DiscoveryOne M7 -- Minted Module

Goal: A user authors a *Power definition and a *syntax do _ while _ end expand declaration in the web app; the minted syntax runs in the browser and is baselined by reg-rs.

Reg-rs cases:
- d1_poc_dowhile_minted_syntax
- d1_poc_dowhile_web_run
- d1_poc_unless_minted_syntax
- d1_poc_unless_web_run
- d1_minted_module_acceptance

Exit:
- All M0-M7 reg-rs cases green.
- The discoveryone web app demo at localhost:1078 round-trips Power and dowhile interactively.
- docs/status.md updated.
- docs/poc-demo.md walks through the scenario.

Every step must keep sw-checklist clean with zero failures and zero warnings, commit .agentrail/ changes, and push.