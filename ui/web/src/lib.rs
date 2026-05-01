use yew::prelude::*;

mod components;
mod edit_state;
mod run_state;
mod runtime;
mod snapshots;

use components::{FacetView, LibraryGrid, PipelineCanvas, RunPanel, TopBar};
use edit_state::use_edit_state;
use runtime::run_output_with_inputs;
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Pipeline { pub(crate) name: &'static str, pub(crate) inputs: PipelineInputs, pub(crate) nodes: &'static [PipelineNode], pub(crate) edges: &'static [PipelineEdge] }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PipelineInputs { pub(crate) n: &'static str, pub(crate) e: &'static str }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PipelineNode { pub(crate) id: &'static str, pub(crate) label: &'static str, pub(crate) kind: &'static str, pub(crate) position: PipelinePosition, pub(crate) inputs: &'static [PipelinePort], pub(crate) outputs: &'static [PipelinePort] }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PipelinePosition { pub(crate) x: u8, pub(crate) y: u8 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PipelinePort { pub(crate) id: &'static str, pub(crate) label: &'static str, pub(crate) value_type: &'static str }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PipelineEdge { pub(crate) from_node: &'static str, pub(crate) from_port: &'static str, pub(crate) to_node: &'static str, pub(crate) to_port: &'static str }

pub(crate) const VALID_PIPELINE_TEXT: &str = "Valid: Power.p feeds Output.value.";
const POWER_INPUTS: [PipelinePort; 2] = [
    PipelinePort { id: "n", label: "n", value_type: "Z" },
    PipelinePort { id: "e", label: "e", value_type: "Z" },
];
const POWER_OUTPUTS: [PipelinePort; 1] = [PipelinePort { id: "p", label: "p", value_type: "Z" }];
const OUTPUT_INPUTS: [PipelinePort; 1] =
    [PipelinePort { id: "value", label: "value", value_type: "Z" }];
const OUTPUT_OUTPUTS: [PipelinePort; 0] = [];
const POWER_OUTPUT_NODES: [PipelineNode; 2] = [
    PipelineNode {
        id: "power",
        label: "Power",
        kind: "numeric",
        position: PipelinePosition { x: 0, y: 0 },
        inputs: &POWER_INPUTS,
        outputs: &POWER_OUTPUTS,
    },
    PipelineNode {
        id: "output",
        label: "Output",
        kind: "sink",
        position: PipelinePosition { x: 1, y: 0 },
        inputs: &OUTPUT_INPUTS,
        outputs: &OUTPUT_OUTPUTS,
    },
];
const POWER_OUTPUT_EDGES: [PipelineEdge; 1] = [PipelineEdge {
    from_node: "power",
    from_port: "p",
    to_node: "output",
    to_port: "value",
}];
pub(crate) const POWER_OUTPUT_PIPELINE: Pipeline = Pipeline {
    name: "Power to Output",
    inputs: PipelineInputs { n: "2", e: "8" },
    nodes: &POWER_OUTPUT_NODES,
    edges: &POWER_OUTPUT_EDGES,
};

pub(crate) fn validate_pipeline(pipeline: Pipeline) -> String {
    if pipeline.edges.len() != 1 {
        return "Error: pipeline requires one Power-to-Output edge.".to_owned();
    }
    let edge = pipeline.edges[0];
    let from = pipeline.nodes.iter().find(|node| node.id == edge.from_node);
    let to = pipeline.nodes.iter().find(|node| node.id == edge.to_node);
    let Some(from) = from else { return format!("Error: missing source node `{}`.", edge.from_node); };
    let Some(to) = to else { return format!("Error: missing target node `{}`.", edge.to_node); };
    let from_port = from.outputs.iter().find(|port| port.id == edge.from_port);
    let to_port = to.inputs.iter().find(|port| port.id == edge.to_port);
    let Some(from_port) = from_port else { return format!("Error: missing output port `{}.{}`.", from.id, edge.from_port); };
    let Some(to_port) = to_port else { return format!("Error: missing input port `{}.{}`.", to.id, edge.to_port); };
    if from_port.value_type != to_port.value_type {
        return format!("Error: incompatible edge types {} -> {}.", from_port.value_type, to_port.value_type);
    }
    VALID_PIPELINE_TEXT.to_owned()
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
            <PipelineCanvas pipeline={POWER_OUTPUT_PIPELINE} />
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

pub fn pipeline_power_output_html_snapshot() -> String {
    let nodes = POWER_OUTPUT_PIPELINE.nodes.iter().map(|node| {
        let inputs = node.inputs.iter().map(|port| format!(r#"<span data-port="{}">{} : {}</span>"#, port.id, port.label, port.value_type)).collect::<Vec<_>>().join("");
        let outputs = node.outputs.iter().map(|port| format!(r#"<span data-port="{}">{} : {}</span>"#, port.id, port.label, port.value_type)).collect::<Vec<_>>().join("");
        if outputs.is_empty() {
            return format!(r#"    <article class="pipeline-node" data-node="{}" style="--x: {}; --y: {}">
      <header><span>{}</span><strong>{}</strong></header>
      <div class="ports inputs">{}</div>
    </article>"#, node.id, node.position.x, node.position.y, node.label, node.kind, inputs);
        }
        format!(r#"    <article class="pipeline-node" data-node="{}" style="--x: {}; --y: {}">
      <header><span>{}</span><strong>{}</strong></header>
      <div class="ports inputs">{}</div>
      <div class="ports outputs">{}</div>
    </article>"#, node.id, node.position.x, node.position.y, node.label, node.kind, inputs, outputs)
    }).collect::<Vec<_>>().join("\n");
    let edge = POWER_OUTPUT_PIPELINE.edges[0];
    format!(
        r#"<section class="pipeline-canvas" aria-label="Pipeline">
  <header class="pipeline-header"><span>Pipeline</span><strong>{}</strong></header>
  <div class="pipeline-nodes">
{}
  </div>
  <div class="pipeline-edges" aria-label="Pipeline edges">
    <span data-edge="{}.{}->{}.{}">Power.{} -> Output.{}</span>
  </div>
  <output class="pipeline-validation" aria-label="Pipeline validation">{}</output>
  <output class="pipeline-run-output" aria-label="Pipeline output">{}</output>
</section>
"#,
        POWER_OUTPUT_PIPELINE.name,
        nodes,
        edge.from_node,
        edge.from_port,
        edge.to_node,
        edge.to_port,
        edge.from_port,
        edge.to_port,
        validate_pipeline(POWER_OUTPUT_PIPELINE),
        run_output_with_inputs(
            DEFINITIONS[0],
            POWER_OUTPUT_PIPELINE.inputs.n,
            POWER_OUTPUT_PIPELINE.inputs.e
        )
    )
}

pub use snapshots::{library_grid_html_snapshot, library_grid_html_snapshot_sorted, minted_run_html_snapshot, power_3d_symbols_json_snapshot, power_front_edit_run_html_snapshot, power_front_facet_html_snapshot, power_run_2_8_html_snapshot};

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;
    const NO_EDGES: [PipelineEdge; 0] = [];
    const BAD_EDGE: [PipelineEdge; 1] = [PipelineEdge { from_node: "power", from_port: "p", to_node: "output", to_port: "missing" }];
    const TEXT_INPUTS: [PipelinePort; 1] = [PipelinePort { id: "value", label: "value", value_type: "Text" }];
    const TEXT_NODES: [PipelineNode; 2] = [POWER_OUTPUT_NODES[0], PipelineNode { inputs: &TEXT_INPUTS, ..POWER_OUTPUT_NODES[1] }];

    #[test]
    fn validates_demo_pipeline_edges() {
        assert_eq!(validate_pipeline(POWER_OUTPUT_PIPELINE), VALID_PIPELINE_TEXT);
        assert_eq!(validate_pipeline(Pipeline { edges: &NO_EDGES, ..POWER_OUTPUT_PIPELINE }), "Error: pipeline requires one Power-to-Output edge.");
        assert_eq!(validate_pipeline(Pipeline { edges: &BAD_EDGE, ..POWER_OUTPUT_PIPELINE }), "Error: missing input port `output.missing`.");
        assert_eq!(validate_pipeline(Pipeline { nodes: &TEXT_NODES, ..POWER_OUTPUT_PIPELINE }), "Error: incompatible edge types Z -> Text.");
    }
}
