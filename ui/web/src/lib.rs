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
            <section class="placeholder">
                <p>
                    { "Web SPA scaffold (M0 step 002). Facet viewer, \
                       definition picker, and run panel land in M6 per " }
                    <code>{ "docs/architecture.md" }</code>
                    { " section 6 and " }
                    <code>{ "docs/design.md" }</code>
                    { " section 10." }
                </p>
                <p>
                    { "See " }
                    <code>{ "docs/plan.md" }</code>
                    { " for the milestone arc." }
                </p>
            </section>
        </main>
    }
}
