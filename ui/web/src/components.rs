use yew::prelude::*;

use crate::{DEFINITIONS, FACES, Face, facet_rows, power_run_2_8_output};

#[function_component(DefinitionPicker)]
pub(crate) fn definition_picker() -> Html {
    html! {
        <label class="definition-picker">
            <span>{ "Definition" }</span>
            <select aria-label="Definition">
                { for DEFINITIONS.iter().map(|definition| html! {
                    <option value={*definition}>{ *definition }</option>
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
    pub(crate) face: Face,
}

#[function_component(FacetView)]
pub(crate) fn facet_view(props: &FacetViewProps) -> Html {
    let rows = facet_rows(props.face);

    html! {
        <article class="facet-view" data-definition="Power" data-face={props.face.query}>
            <header class="facet-header">
                <span>{ "Power" }</span>
                <strong>{ props.face.label }</strong>
            </header>
            <pre class="facet-grid" aria-label={format!("Power {} facet", props.face.label)}>
                { rows.join("\n") }
            </pre>
        </article>
    }
}

#[function_component(RunPanel)]
pub(crate) fn run_panel() -> Html {
    let output = power_run_2_8_output();

    html! {
        <aside class="run-panel" data-definition="Power">
            <header class="run-header">
                <span>{ "RunPanel" }</span>
                <strong>{ "Power" }</strong>
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
