use yew::prelude::*;

use crate::runtime::validate_source;
use crate::Definition;

pub(crate) struct EditState {
    pub(crate) is_editing: UseStateHandle<bool>,
    pub(crate) source_text: UseStateHandle<String>,
    pub(crate) validation: UseStateHandle<String>,
    pub(crate) on_toggle_edit: Callback<()>,
    pub(crate) on_source_input: Callback<String>,
}

#[hook]
pub(crate) fn use_edit_state(current_definition: UseStateHandle<Definition>) -> EditState {
    let is_editing = use_state(|| false);
    let source_text = use_state(|| current_definition.source.to_owned());
    let validation = use_state(|| validate_source(current_definition.source));
    {
        let source_text = source_text.clone();
        let validation = validation.clone();
        use_effect_with(*current_definition, move |definition| {
            source_text.set(definition.source.to_owned());
            validation.set(validate_source(definition.source));
        });
    }
    let on_toggle_edit = Callback::from({
        let is_editing = is_editing.clone();
        let source_text = source_text.clone();
        let validation = validation.clone();
        let current_definition = current_definition.clone();
        move |_| {
            if !*is_editing {
                source_text.set(current_definition.source.to_owned());
                validation.set(validate_source(current_definition.source));
            }
            is_editing.set(!*is_editing);
        }
    });
    let on_source_input = Callback::from({
        let source_text = source_text.clone();
        let validation = validation.clone();
        move |source: String| {
            validation.set(validate_source(&source));
            source_text.set(source);
        }
    });

    EditState {
        is_editing,
        source_text,
        validation,
        on_toggle_edit,
        on_source_input,
    }
}
