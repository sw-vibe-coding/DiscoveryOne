use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yew::TargetCast;

use crate::runtime::definition_by_name;
use crate::{DEFINITIONS, Definition, FACES, Face, facet_rows};

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
}

#[function_component(FacetView)]
pub(crate) fn facet_view(props: &FacetViewProps) -> Html {
    let rows = facet_rows(props.definition, props.face);

    html! {
        <article class="facet-view" data-definition={props.definition.name} data-face={props.face.query}>
            <header class="facet-header">
                <span>{ props.definition.name }</span>
                <strong>{ props.face.label }</strong>
            </header>
            <pre class="facet-grid" aria-label={format!("{} {} facet", props.definition.name, props.face.label)}>
                { rows.join("\n") }
            </pre>
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
