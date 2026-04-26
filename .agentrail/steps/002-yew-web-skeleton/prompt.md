# Step: yew-web-skeleton

Stand up the Yew SPA crate that future sagas will extend. The
page renders a single placeholder; the goal is to prove the
Trunk + wasm-bindgen + Yew toolchain works in this repo, not to
ship UI.

## Source-of-truth

- `docs/architecture.md` section 6 (UI architecture).
- `docs/design.md` section 10 (web components).
- `~/github/sw-embed/web-sw-cor24-ocaml/Trunk.toml`,
  `index.html`, `src/lib.rs` -- mirror the pattern.

## Work

1. Create `ui/web/Cargo.toml` for crate `d1-web`. Dependencies:
   `yew = "0.21"` (or whichever the Trunk template defaults to),
   `wasm-bindgen`, `web-sys` as needed. `crate-type = ["cdylib"]`.
2. Create `ui/web/src/lib.rs` with a minimal `App` Yew component
   that renders `<h1>DiscoveryOne</h1>` plus a paragraph
   referencing `docs/architecture.md`.
3. Create `ui/web/index.html` and `ui/web/Trunk.toml` so
   `trunk serve` works from `ui/web/`. Reference
   `images/discovery-one-badge.png` as the page icon.
4. Add `ui/web` to the workspace's exclude list (it builds for
   `wasm32-unknown-unknown`, not host) OR list it as a workspace
   member with a `[[target]]`-conditional dependency block --
   pick whichever the existing reference repo
   (`web-sw-cor24-ocaml`) uses for the same crate type. Document
   the choice in a one-line `# ` comment in the root `Cargo.toml`.

## Pre-commit gates

- Root `cargo build` and `cargo test` still clean (no
  regression from step 001).
- `trunk build` from `ui/web/` produces a `dist/` (do not
  commit `dist/`).
- `cargo fmt -- --check` and clippy clean for the `d1-web`
  crate.

## Deliverables

- `ui/web/Cargo.toml`.
- `ui/web/src/lib.rs`.
- `ui/web/index.html`.
- `ui/web/Trunk.toml`.
- Updated root `Cargo.toml` reflecting the workspace decision.

## Commit / complete protocol

```bash
git add ui/ Cargo.toml Cargo.lock .agentrail/
git commit -m "feat: yew web skeleton via trunk"
git push
agentrail complete \
  --summary "Yew SPA shell builds via trunk; placeholder page" \
  --reward 1 \
  --actions "yew 0.21, trunk, wasm-bindgen"
```

`.agentrail/` is mandatory in the commit. Same rule as step 001.

## Risks

- Yew / Trunk version skew: pin both to the versions used by
  `web-sw-cor24-ocaml` for consistency across the
  Software Wrighter web app family.
- Workspace exclude vs member: getting this wrong breaks
  `cargo build` from the repo root. The reference repo is the
  authoritative pattern; do not invent a new layout.
