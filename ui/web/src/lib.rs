use yew::prelude::*;

mod components;
mod edit_state;
mod pipeline;
mod run_state;
mod runtime;
mod snapshots;

use components::{FacetView, LibraryGrid, RunPanel, TopBar};
use edit_state::use_edit_state;
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
pub(crate) const LIBRARY_ROWS: [LibraryRow; 6] = [
    LibraryRow {
        name: "Add",
        arity: "2 -> 1",
        category: "numeric",
        aspects: "front rear",
    },
    LibraryRow {
        name: "Clamp",
        arity: "3 -> 1",
        category: "numeric",
        aspects: "front top bottom",
    },
    LibraryRow {
        name: "Power",
        arity: "2 -> 1",
        category: "numeric",
        aspects: "front left right top bottom rear",
    },
    LibraryRow {
        name: "DowhileCounter",
        arity: "0 -> 1",
        category: "syntax-template",
        aspects: "front internal",
    },
    LibraryRow {
        name: "UnlessCounter",
        arity: "0 -> 1",
        category: "syntax-template",
        aspects: "front internal",
    },
    LibraryRow {
        name: "TraceValue",
        arity: "1 -> 1",
        category: "instrumentation",
        aspects: "front rear internal",
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LibraryRow {
    pub(crate) name: &'static str,
    pub(crate) arity: &'static str,
    pub(crate) category: &'static str,
    pub(crate) aspects: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LibrarySort {
    Name,
    Arity,
    Category,
    Aspects,
}

#[function_component(App)]
#[rustfmt::skip]
pub fn app() -> Html {
    let current_definition = use_state(|| DEFINITIONS[0]);
    let current_face = use_state(|| DEFINITIONS[0].selected_face);
    let library_sort = use_state(|| LibrarySort::Name);
    let edit_state = use_edit_state(current_definition.clone(), current_face.clone());
    let run_state = use_run_state(current_definition.clone(), current_face.clone(), edit_state.facet_text.clone());
    let on_face_select = Callback::from({
        let current_face = current_face.clone();
        move |face| current_face.set(face)
    });

    html! {
        <main class="app">
            <TopBar current_definition={*current_definition} current_face={*current_face} on_definition_select={run_state.on_definition_select} {on_face_select} />
            <section class="workspace">
                <FacetView definition={*current_definition} face={*current_face} is_editing={*edit_state.is_editing} facet_text={(*edit_state.facet_text).clone()} validation={(*edit_state.validation).clone()} on_toggle_edit={edit_state.on_toggle_edit} on_facet_input={edit_state.on_facet_input} />
                <RunPanel definition={*current_definition} n_value={(*run_state.n_input).clone()} e_value={(*run_state.e_input).clone()} output={(*run_state.output).clone()} on_n_input={run_state.on_n_input} on_e_input={run_state.on_e_input} on_run={run_state.on_run} />
            </section>
            <LibraryGrid rows={sorted_library_rows(*library_sort)} current_sort={*library_sort} on_sort={Callback::from(move |sort| library_sort.set(sort))} />
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

pub(crate) fn sorted_library_rows(sort: LibrarySort) -> Vec<LibraryRow> {
    let mut rows = LIBRARY_ROWS.to_vec();
    rows.sort_by(|left, right| library_grid_cmp(left, right, sort));
    rows
}

fn library_grid_cmp(left: &LibraryRow, right: &LibraryRow, sort: LibrarySort) -> std::cmp::Ordering {
    let primary = match sort {
        LibrarySort::Name => left.name.cmp(right.name),
        LibrarySort::Arity => left.arity.cmp(right.arity),
        LibrarySort::Category => left.category.cmp(right.category),
        LibrarySort::Aspects => left.aspects.cmp(right.aspects),
    };
    primary.then_with(|| left.name.cmp(right.name))
}

pub use pipeline::pipeline_power_output_html_snapshot;
pub use snapshots::{
    library_grid_html_snapshot, library_grid_html_snapshot_sorted, minted_run_html_snapshot,
    power_front_edit_run_html_snapshot, power_front_facet_html_snapshot,
    power_run_2_8_html_snapshot,
};
