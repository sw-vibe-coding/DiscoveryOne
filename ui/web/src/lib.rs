use yew::prelude::*;

mod components;
mod snapshots;

use components::{FacetView, RunPanel, TopBar};

pub(crate) const POWER_SOURCE: &str = include_str!("../../../examples/power.d1");
pub(crate) const DOWHILE_SOURCE: &str = include_str!("../../../examples/dowhile.d1");
pub(crate) const UNLESS_SOURCE: &str = include_str!("../../../examples/unless.d1");
pub(crate) const DEFINITIONS: [Definition; 3] = [
    Definition {
        name: "Power",
        source: POWER_SOURCE,
        selected_face: FRONT,
    },
    Definition {
        name: "DowhileCounter",
        source: DOWHILE_SOURCE,
        selected_face: INTERNAL,
    },
    Definition {
        name: "UnlessCounter",
        source: UNLESS_SOURCE,
        selected_face: INTERNAL,
    },
];
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
pub(crate) const INTERNAL: Face = Face {
    label: "Internal",
    query: "internal",
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Definition {
    pub(crate) name: &'static str,
    pub(crate) source: &'static str,
    pub(crate) selected_face: Face,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Face {
    pub(crate) label: &'static str,
    pub(crate) query: &'static str,
}

#[function_component(App)]
#[rustfmt::skip]
pub fn app() -> Html {
    let current_definition = use_state(|| DEFINITIONS[0]);
    let current_face = use_state(|| DEFINITIONS[0].selected_face);
    let on_definition_select = Callback::from({ let current_definition = current_definition.clone(); let current_face = current_face.clone(); move |definition: Definition| {
        current_face.set(definition.selected_face); current_definition.set(definition);
    }});
    let on_face_select = Callback::from({ let current_face = current_face.clone(); move |face| current_face.set(face) });
    html! {
        <main class="app">
            <TopBar current_definition={*current_definition} current_face={*current_face} {on_definition_select} {on_face_select} />
            <section class="workspace">
                <FacetView definition={*current_definition} face={*current_face} />
                <RunPanel definition={*current_definition} />
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

pub(crate) fn facet_rows(definition: Definition, face: Face) -> Vec<String> {
    d1_source::emit_layered(definition.source, Some(face.query))
        .expect("bundled fixture should project")
        .trim_end()
        .lines()
        .map(str::to_owned)
        .collect()
}

pub(crate) fn run_output(definition: Definition) -> String {
    d1_interp::run_and_dump(definition.source, &["n=2".to_owned(), "e=8".to_owned()])
        .expect("bundled fixture should run")
        .trim_end()
        .to_owned()
}

pub use snapshots::{
    minted_run_html_snapshot, power_front_facet_html_snapshot, power_run_2_8_html_snapshot,
};
