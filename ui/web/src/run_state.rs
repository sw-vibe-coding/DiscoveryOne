use yew::prelude::*;

use crate::runtime::{run_output_with_facet_edit, run_output_with_inputs};
use crate::{DEFINITIONS, Definition, Face};

pub(crate) struct RunState {
    pub(crate) n_input: UseStateHandle<String>,
    pub(crate) e_input: UseStateHandle<String>,
    pub(crate) output: UseStateHandle<String>,
    pub(crate) on_definition_select: Callback<Definition>,
    pub(crate) on_n_input: Callback<String>,
    pub(crate) on_e_input: Callback<String>,
    pub(crate) on_run: Callback<()>,
}

#[hook]
pub(crate) fn use_run_state(
    current_definition: UseStateHandle<Definition>,
    current_face: UseStateHandle<Face>,
    facet_text: UseStateHandle<String>,
) -> RunState {
    let n_input = use_state(|| "2".to_owned());
    let e_input = use_state(|| "8".to_owned());
    let output = use_state(|| run_output_with_inputs(DEFINITIONS[0], "2", "8"));
    let on_definition_select = Callback::from({
        let current_definition = current_definition.clone();
        let current_face = current_face.clone();
        let n_input = n_input.clone();
        let e_input = e_input.clone();
        let output = output.clone();
        move |definition: Definition| {
            current_face.set(definition.selected_face);
            current_definition.set(definition);
            output.set(run_output_with_inputs(definition, &n_input, &e_input));
        }
    });
    let on_n_input = Callback::from({
        let n_input = n_input.clone();
        move |value: String| n_input.set(value)
    });
    let on_e_input = Callback::from({
        let e_input = e_input.clone();
        move |value: String| e_input.set(value)
    });
    let on_run = Callback::from({
        let current_definition = current_definition.clone();
        let current_face = current_face.clone();
        let facet_text = facet_text.clone();
        let n_input = n_input.clone();
        let e_input = e_input.clone();
        let output = output.clone();
        move |_| {
            output.set(run_output_with_facet_edit(
                *current_definition,
                *current_face,
                &facet_text,
                &n_input,
                &e_input,
            ))
        }
    });

    RunState {
        n_input,
        e_input,
        output,
        on_definition_select,
        on_n_input,
        on_e_input,
        on_run,
    }
}
