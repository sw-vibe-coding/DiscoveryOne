use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;
use yew::TargetCast;

use crate::runtime::definition_by_name;
use crate::runtime::run_output_with_inputs;
use crate::{
    DEFINITIONS, Definition, FACES, Face, LibraryRow, LibrarySort, Pipeline, facet_rows,
    validate_pipeline,
};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn initDiscoveryOne3dViewer(mount: web_sys::HtmlElement, symbols_json: &str);
}

#[derive(Properties, PartialEq)]
pub(crate) struct TopBarProps {
    pub(crate) current_definition: Definition,
    pub(crate) current_face: Face,
    pub(crate) on_definition_select: Callback<Definition>,
    pub(crate) on_face_select: Callback<Face>,
}

#[function_component(TopBar)]
pub(crate) fn top_bar(props: &TopBarProps) -> Html {
    html! {
        <header class="topbar">
            <img class="badge" src="discovery-one-badge.png" alt="DiscoveryOne badge" />
            <div class="titleblock">
                <h1>{ "DiscoveryOne" }</h1>
                <span>{ "M7 minted module" }</span>
            </div>
            <DefinitionPicker
                current={props.current_definition}
                on_select={props.on_definition_select.clone()}
            />
            <FaceSelector current={props.current_face} on_select={props.on_face_select.clone()} />
        </header>
    }
}

#[derive(Properties, PartialEq)]
pub(crate) struct DefinitionPickerProps {
    pub(crate) current: Definition,
    pub(crate) on_select: Callback<Definition>,
}

#[function_component(DefinitionPicker)]
pub(crate) fn definition_picker(props: &DefinitionPickerProps) -> Html {
    html! {
        <label class="definition-picker">
            <span>{ "Definition" }</span>
            <select
                aria-label="Definition"
                value={props.current.name}
                onchange={
                    let on_select = props.on_select.clone();
                    Callback::from(move |event: Event| {
                        let select = event.target_unchecked_into::<HtmlSelectElement>();
                        if let Some(definition) = definition_by_name(&select.value()) {
                            on_select.emit(definition);
                        }
                    })
                }
            >
                { for DEFINITIONS.iter().map(|definition| html! {
                    <option
                        value={definition.name}
                        selected={*definition == props.current}
                    >
                        { definition.name }
                    </option>
                }) }
            </select>
        </label>
    }
}

#[derive(Properties, PartialEq)]
pub(crate) struct FaceSelectorProps {
    pub(crate) current: Face,
    pub(crate) on_select: Callback<Face>,
}

#[function_component(FaceSelector)]
pub(crate) fn face_selector(props: &FaceSelectorProps) -> Html {
    html! {
        <nav class="face-selector" aria-label="Face">
            { for FACES.iter().map(|face| {
                let face = *face;
                let selected = face == props.current;
                let on_select = props.on_select.clone();
                html! {
                    <button
                        type="button"
                        class={classes!("face-button", selected.then_some("selected"))}
                        aria-pressed={selected.to_string()}
                        onclick={Callback::from(move |_| on_select.emit(face))}
                    >
                        { face.label }
                    </button>
                }
            }) }
        </nav>
    }
}

#[derive(Properties, PartialEq)]
pub(crate) struct FacetViewProps {
    pub(crate) definition: Definition,
    pub(crate) face: Face,
    pub(crate) is_editing: bool,
    pub(crate) facet_text: String,
    pub(crate) validation: String,
    pub(crate) on_toggle_edit: Callback<()>,
    pub(crate) on_facet_input: Callback<String>,
}

#[function_component(FacetView)]
pub(crate) fn facet_view(props: &FacetViewProps) -> Html {
    let rows = facet_rows(props.definition, props.face);

    html! {
        <article class="facet-view" data-definition={props.definition.name} data-face={props.face.query}>
            <header class="facet-header">
                <span>{ props.definition.name }</span>
                <div class="facet-tools">
                    <strong>{ props.face.label }</strong>
                    <button type="button" class="mode-toggle" onclick={
                        let on_toggle_edit = props.on_toggle_edit.clone();
                        Callback::from(move |_| on_toggle_edit.emit(()))
                    }>
                        { if props.is_editing { "Browse" } else { "Edit" } }
                    </button>
                </div>
            </header>
            if props.is_editing {
                <textarea
                    class={classes!("facet-editor", props.validation.starts_with("Error:").then_some("invalid"))}
                    aria-label={format!("{} {} facet editor", props.definition.name, props.face.label)}
                    value={props.facet_text.clone()}
                    oninput={
                        let on_facet_input = props.on_facet_input.clone();
                        Callback::from(move |event: InputEvent| {
                            let textarea = event.target_unchecked_into::<HtmlTextAreaElement>();
                            on_facet_input.emit(textarea.value());
                        })
                    }
                />
                <output
                    class={classes!("validation-output", props.validation.starts_with("Error:").then_some("invalid"))}
                    aria-label="Facet validation"
                >
                    { &props.validation }
                </output>
            } else {
                <pre class="facet-grid" aria-label={format!("{} {} facet", props.definition.name, props.face.label)}>
                    { rows.join("\n") }
                </pre>
            }
        </article>
    }
}

#[derive(Properties, PartialEq)]
pub(crate) struct RunPanelProps {
    pub(crate) definition: Definition,
    pub(crate) n_value: String,
    pub(crate) e_value: String,
    pub(crate) output: String,
    pub(crate) on_n_input: Callback<String>,
    pub(crate) on_e_input: Callback<String>,
    pub(crate) on_run: Callback<()>,
}

#[function_component(RunPanel)]
pub(crate) fn run_panel(props: &RunPanelProps) -> Html {
    html! {
        <aside class="run-panel" data-definition={props.definition.name}>
            <header class="run-header">
                <span>{ "RunPanel" }</span>
                <strong>{ props.definition.name }</strong>
            </header>
            <div class="run-inputs" aria-label="Power inputs">
                <label>
                    <span>{ "n" }</span>
                    <input
                        value={props.n_value.clone()}
                        oninput={
                            let on_n_input = props.on_n_input.clone();
                            Callback::from(move |event: InputEvent| {
                                let input = event.target_unchecked_into::<HtmlInputElement>();
                                on_n_input.emit(input.value());
                            })
                        }
                    />
                </label>
                <label>
                    <span>{ "e" }</span>
                    <input
                        value={props.e_value.clone()}
                        oninput={
                            let on_e_input = props.on_e_input.clone();
                            Callback::from(move |event: InputEvent| {
                                let input = event.target_unchecked_into::<HtmlInputElement>();
                                on_e_input.emit(input.value());
                            })
                        }
                    />
                </label>
                <button
                    type="button"
                    onclick={
                        let on_run = props.on_run.clone();
                        Callback::from(move |_| on_run.emit(()))
                    }
                >
                    { "Run" }
                </button>
            </div>
            <output class="run-output" aria-label="Power output">{ &props.output }</output>
        </aside>
    }
}

#[derive(Properties, PartialEq)]
pub(crate) struct LibraryGridProps {
    pub(crate) rows: Vec<LibraryRow>,
    pub(crate) current_sort: LibrarySort,
    pub(crate) on_sort: Callback<LibrarySort>,
}

#[function_component(LibraryGrid)]
pub(crate) fn library_grid(props: &LibraryGridProps) -> Html {
    html! {
        <section class="library-grid" aria-label="Library definitions">
            <header class="library-header">
                <span>{ "Library" }</span>
                <div class="sort-controls" aria-label="Library sort">
                    { for [
                        (LibrarySort::Name, "Name"),
                        (LibrarySort::Arity, "Arity"),
                        (LibrarySort::Category, "Type"),
                        (LibrarySort::Aspects, "Aspects"),
                    ].into_iter().map(|(sort, label)| {
                        let selected = props.current_sort == sort;
                        let on_sort = props.on_sort.clone();
                        html! {
                            <button
                                type="button"
                                class={classes!("sort-button", selected.then_some("selected"))}
                                aria-pressed={selected.to_string()}
                                onclick={Callback::from(move |_| on_sort.emit(sort))}
                            >
                                { label }
                            </button>
                        }
                    }) }
                </div>
            </header>
            <table>
                <thead><tr><th>{ "Name" }</th><th>{ "Arity" }</th><th>{ "Type" }</th><th>{ "Aspects" }</th></tr></thead>
                <tbody>
                    { for props.rows.iter().map(|row| html! {
                        <tr data-definition={row.name}>
                            <td>{ row.name }</td>
                            <td>{ row.arity }</td>
                            <td>{ row.category }</td>
                            <td>{ row.aspects }</td>
                        </tr>
                    }) }
                </tbody>
            </table>
        </section>
    }
}

#[derive(Properties, PartialEq)]
pub(crate) struct PipelineCanvasProps {
    pub(crate) pipeline: Pipeline,
}

#[function_component(PipelineCanvas)]
pub(crate) fn pipeline_canvas(props: &PipelineCanvasProps) -> Html {
    html! {
        <section class="pipeline-canvas" aria-label="Pipeline">
            <header class="pipeline-header">
                <span>{ "Pipeline" }</span>
                <strong>{ props.pipeline.name }</strong>
            </header>
            <div class="pipeline-nodes">
                { for props.pipeline.nodes.iter().map(|node| html! {
                    <article class="pipeline-node" data-node={node.id} style={format!("--x: {}; --y: {}", node.position.x, node.position.y)}>
                        <header><span>{ node.label }</span><strong>{ node.kind }</strong></header>
                        <div class="ports inputs">
                            { for node.inputs.iter().map(|port| html! {
                                <span data-port={port.id}>{ format!("{} : {}", port.label, port.value_type) }</span>
                            }) }
                        </div>
                        if !node.outputs.is_empty() {
                            <div class="ports outputs">
                                { for node.outputs.iter().map(|port| html! {
                                    <span data-port={port.id}>{ format!("{} : {}", port.label, port.value_type) }</span>
                                }) }
                            </div>
                        }
                    </article>
                }) }
            </div>
            <div class="pipeline-edges" aria-label="Pipeline edges">
                { for props.pipeline.edges.iter().map(|edge| html! {
                    <span data-edge={format!("{}.{}->{}.{}", edge.from_node, edge.from_port, edge.to_node, edge.to_port)}>
                        { format!("{}.{} -> {}.{}", if edge.from_node == "power" { "Power" } else { edge.from_node }, edge.from_port, if edge.to_node == "output" { "Output" } else { edge.to_node }, edge.to_port) }
                    </span>
                }) }
            </div>
            <output class="pipeline-validation" aria-label="Pipeline validation">{ validate_pipeline(props.pipeline) }</output>
            <output class="pipeline-run-output" aria-label="Pipeline output">
                { run_output_with_inputs(DEFINITIONS[0], props.pipeline.inputs.n, props.pipeline.inputs.e) }
            </output>
        </section>
    }
}

#[function_component(Viewer3dPanel)]
pub(crate) fn viewer_3d_panel() -> Html {
    let mount = use_node_ref();
    #[cfg(target_arch = "wasm32")]
    {
        let mount = mount.clone();
        use_effect_with((), move |_| {
            if let Some(element) = mount.cast::<web_sys::HtmlElement>() {
                initDiscoveryOne3dViewer(element, &crate::power_3d_symbols_json_snapshot());
            }
            || ()
        });
    }
    html! { <section class="viewer-3d" aria-label="3D viewer">
        <header class="viewer-3d-header"><span>{ "3D Viewer" }</span><strong>{ "Power" }</strong></header>
        <div class="viewer-3d-mount" ref={mount} data-entry="viewer3d" data-export="window.discoveryOne3dSymbols">
            <output class="viewer-3d-status" aria-label="3D viewer status">{ "Loading: Power symbol export is wired to the viewer3d three.js scene bundle." }</output>
        </div>
    </section> }
}
