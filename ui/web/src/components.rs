use yew::prelude::*;

use crate::{DEFINITIONS, Definition, FACES, Face, facet_rows, run_output};

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
            <select aria-label="Definition" value={props.current.name}>
                { for DEFINITIONS.iter().map(|definition| html! {
                    <option
                        value={definition.name}
                        selected={*definition == props.current}
                        onclick={
                            let on_select = props.on_select.clone();
                            let definition = *definition;
                            Callback::from(move |_| on_select.emit(definition))
                        }
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
}

#[function_component(RunPanel)]
pub(crate) fn run_panel(props: &RunPanelProps) -> Html {
    let output = run_output(props.definition);

    html! {
        <aside class="run-panel" data-definition={props.definition.name}>
            <header class="run-header">
                <span>{ "RunPanel" }</span>
                <strong>{ props.definition.name }</strong>
            </header>
            <div class="run-inputs" aria-label="Power inputs">
                <label><span>{ "n" }</span><input value="2" readonly=true /></label>
                <label><span>{ "e" }</span><input value="8" readonly=true /></label>
                <button type="button">{ "Run" }</button>
            </div>
            <output class="run-output" aria-label="Power output">{ output }</output>
        </aside>
    }
}
