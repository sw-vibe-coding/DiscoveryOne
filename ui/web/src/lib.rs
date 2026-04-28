use yew::prelude::*;

mod components;
mod snapshots;

use components::{DefinitionPicker, FaceSelector, FacetView, RunPanel};

pub(crate) const POWER_SOURCE: &str = include_str!("../../../examples/power.d1");
pub(crate) const DEFINITIONS: &[&str] = &["Power"];
pub(crate) const FACES: [Face; 6] = [
    Face {
        label: "Front",
        query: "front",
    },
    Face {
        label: "Left",
        query: "left",
    },
    Face {
        label: "Right",
        query: "right",
    },
    Face {
        label: "Top",
        query: "top",
    },
    Face {
        label: "Bottom",
        query: "bottom",
    },
    Face {
        label: "Rear",
        query: "rear",
    },
];
pub(crate) const FRONT: Face = FACES[0];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Face {
    pub(crate) label: &'static str,
    pub(crate) query: &'static str,
}

#[function_component(App)]
pub fn app() -> Html {
    let current_face = use_state(|| FRONT);

    html! {
        <main class="app">
            <header class="topbar">
                <img class="badge" src="discovery-one-badge.png" alt="DiscoveryOne badge" />
                <div class="titleblock">
                    <h1>{ "DiscoveryOne" }</h1>
                    <span>{ "M6 web shell" }</span>
                </div>
                <DefinitionPicker />
                <FaceSelector current={*current_face} on_select={
                    let current_face = current_face.clone();
                    Callback::from(move |face| current_face.set(face))
                } />
            </header>
            <section class="workspace">
                <FacetView face={*current_face} />
                <RunPanel />
            </section>
            <BuildFooter />
        </main>
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

pub(crate) fn facet_rows(face: Face) -> Vec<String> {
    d1_source::emit_layered(POWER_SOURCE, Some(face.query))
        .expect("bundled Power fixture should project")
        .trim_end()
        .lines()
        .map(str::to_owned)
        .collect()
}

pub(crate) fn power_run_2_8_output() -> String {
    d1_interp::run_and_dump(POWER_SOURCE, &["n=2".to_owned(), "e=8".to_owned()])
        .expect("bundled Power fixture should run")
        .trim_end()
        .to_owned()
}

pub use snapshots::{power_front_facet_html_snapshot, power_run_2_8_html_snapshot};
