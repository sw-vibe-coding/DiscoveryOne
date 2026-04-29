use yew::prelude::*;

mod components;
mod run_state;
mod runtime;
mod snapshots;

use components::{FacetView, RunPanel, TopBar};
use run_state::use_run_state;

pub(crate) const POWER_SOURCE: &str = include_str!("../../../examples/power.d1");
pub(crate) const DOWHILE_SOURCE: &str = include_str!("../../../examples/dowhile.d1");
pub(crate) const UNLESS_SOURCE: &str = include_str!("../../../examples/unless.d1");
const POWER_FRONT_NOTATION: [&str; 3] = [
    "                                      ⎧ 1 →",
    "   • Power (n : ℤ  e : ℤ) → (p : ℤ) ← ⎨ loop e times    iff e is positive",
    "                                      ⎩ n (×) →",
];
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
    let run_state = use_run_state(current_definition.clone(), current_face.clone());
    let on_face_select = Callback::from({
        let current_face = current_face.clone();
        move |face| current_face.set(face)
    });

    html! {
        <main class="app">
            <TopBar current_definition={*current_definition} current_face={*current_face} on_definition_select={run_state.on_definition_select} {on_face_select} />
            <section class="workspace">
                <FacetView definition={*current_definition} face={*current_face} />
                <RunPanel definition={*current_definition} n_value={(*run_state.n_input).clone()} e_value={(*run_state.e_input).clone()} output={(*run_state.output).clone()} on_n_input={run_state.on_n_input} on_e_input={run_state.on_e_input} on_run={run_state.on_run} />
            </section>
            <BuildFooter />
        </main>
    }
}

#[function_component(BuildFooter)]
fn build_footer() -> Html {
    html! {
        <footer class="footer">
            <span>{ concat!("Version: ", env!("CARGO_PKG_VERSION")) }</span>
            <span>{ "Copyright: Copyright (c) 2026 Michael A Wright" }</span>
            <span>{ "License: MIT" }</span>
            <span>{ concat!("Repository: ", env!("CARGO_PKG_REPOSITORY")) }</span>
            <span>{ concat!("Build Host: ", env!("BUILD_HOST")) }</span>
            <span>{ concat!("Build Commit: ", env!("BUILD_COMMIT")) }</span>
            <span>{ concat!("Build Time: ", env!("BUILD_TIME")) }</span>
        </footer>
    }
}

pub(crate) fn facet_rows(definition: Definition, face: Face) -> Vec<String> {
    if definition.name == "Power" && face == FRONT {
        return POWER_FRONT_NOTATION.iter().map(|row| row.to_string()).collect();
    }

    d1_source::emit_layered(definition.source, Some(face.query))
        .expect("bundled fixture should project")
        .trim_end()
        .lines()
        .map(str::to_owned)
        .collect()
}

pub use snapshots::{
    minted_run_html_snapshot, power_front_facet_html_snapshot, power_run_2_8_html_snapshot,
};
