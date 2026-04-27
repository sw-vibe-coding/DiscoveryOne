//! DiscoveryOne web SPA -- minimal placeholder shell.
//!
//! Step 002 of the `discoveryone-scaffold` saga: prove the
//! Trunk + wasm-bindgen + Yew toolchain works in this repo.
//! Real components (`DefinitionPicker`, `FaceSelector`,
//! `FacetView`, `RunPanel`) arrive in milestone M6 per
//! `docs/plan.md`.

use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="app">
            <header class="banner">
                <img
                    class="badge"
                    src="discovery-one-badge.png"
                    alt="DiscoveryOne badge"
                />
                <h1>{ "DiscoveryOne" }</h1>
            </header>
            <Placeholder />
            <BuildFooter />
        </main>
    }
}

#[function_component(Placeholder)]
fn placeholder() -> Html {
    html! {
        <section class="placeholder">
            <p>
                { "Web SPA scaffold. Facet viewer, definition picker, and run panel land in M6 per " }
                <code>{ "docs/design.md" }</code>
                { " section 10." }
            </p>
            <p>{ "See " }<code>{ "docs/plan.md" }</code>{ " for the milestone arc." }</p>
        </section>
    }
}

#[function_component(BuildFooter)]
fn build_footer() -> Html {
    html! {
        <footer class="footer">
            <span>{ "Copyright: Copyright (c) 2026 Michael A Wright" }</span>
            <span>{ "License: MIT" }</span>
            <span>{ "Repository: https://github.com/sw-vibe-coding/DiscoveryOne" }</span>
            <span>{ "Build Host: unknown" }</span>
            <span>{ "Build Commit: unknown" }</span>
            <span>{ "Build Time: 1970-01-01 00:00:00 UTC" }</span>
        </footer>
    }
}
