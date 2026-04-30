use yew::prelude::*;

use crate::{Definition, Face, facet_rows};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PowerFrontEdit {
    pub(crate) zero_case: i64,
}

pub(crate) struct EditState {
    pub(crate) is_editing: UseStateHandle<bool>,
    pub(crate) facet_text: UseStateHandle<String>,
    pub(crate) validation: UseStateHandle<String>,
    pub(crate) on_toggle_edit: Callback<()>,
    pub(crate) on_facet_input: Callback<String>,
}

#[hook]
pub(crate) fn use_edit_state(
    current_definition: UseStateHandle<Definition>,
    current_face: UseStateHandle<Face>,
) -> EditState {
    let is_editing = use_state(|| false);
    let facet_text = use_state(|| editable_facet_text(*current_definition, *current_face));
    let validation = use_state(|| validate_facet_edit(*current_definition, *current_face, &facet_text));
    {
        let facet_text = facet_text.clone();
        let validation = validation.clone();
        use_effect_with((*current_definition, *current_face), move |(definition, face)| {
            let text = editable_facet_text(*definition, *face);
            validation.set(validate_facet_edit(*definition, *face, &text));
            facet_text.set(text);
        });
    }
    let on_toggle_edit = Callback::from({
        let is_editing = is_editing.clone();
        let facet_text = facet_text.clone();
        let validation = validation.clone();
        let current_definition = current_definition.clone();
        let current_face = current_face.clone();
        move |_| {
            if !*is_editing {
                let text = editable_facet_text(*current_definition, *current_face);
                validation.set(validate_facet_edit(*current_definition, *current_face, &text));
                facet_text.set(text);
            }
            is_editing.set(!*is_editing);
        }
    });
    let on_facet_input = Callback::from({
        let facet_text = facet_text.clone();
        let validation = validation.clone();
        let current_definition = current_definition.clone();
        let current_face = current_face.clone();
        move |text: String| {
            validation.set(validate_facet_edit(*current_definition, *current_face, &text));
            facet_text.set(text);
        }
    });

    EditState {
        is_editing,
        facet_text,
        validation,
        on_toggle_edit,
        on_facet_input,
    }
}

pub(crate) fn editable_facet_text(definition: Definition, face: Face) -> String {
    facet_rows(definition, face).join("\n")
}

pub(crate) fn validate_facet_edit(definition: Definition, face: Face, text: &str) -> String {
    if text.trim().is_empty() {
        return "Error: facet text is empty.".to_owned();
    }

    if definition.name == "Power" && face.query == "front" {
        return match parse_power_front_edit(text) {
            Ok(_) => "Valid Power Front edit. Run uses this zero-case value for e=0; reverse projection to full source is not implemented yet.".to_owned(),
            Err(err) => format!("Error: {err}"),
        };
    }

    "Valid 2D facet text. Reverse projection to full source is not implemented yet; edits are preview-only.".to_owned()
}

pub(crate) fn parse_power_front_edit(text: &str) -> Result<PowerFrontEdit, String> {
    for required in ["Power", "loop e times", "n (×)"] {
        if !text.contains(required) {
            return Err(format!("Power Front edit is missing `{required}`."));
        }
    }

    let top_branch = text
        .lines()
        .find(|line| line.contains('⎧'))
        .ok_or_else(|| "Power Front edit is missing the top branch.".to_owned())?;
    let after_branch = top_branch
        .split_once('⎧')
        .map(|(_, rest)| rest)
        .ok_or_else(|| "Power Front edit is missing the top branch.".to_owned())?;
    let literal = after_branch
        .split_once('→')
        .map(|(value, _)| value.trim())
        .ok_or_else(|| "Power Front top branch is missing `→`.".to_owned())?;

    if literal.is_empty() {
        return Err("Power Front top branch is missing its zero-case integer.".to_owned());
    }

    let zero_case = literal.parse::<i64>().map_err(|_| {
        format!("Power Front top branch zero-case `{literal}` is not an integer.")
    })?;

    Ok(PowerFrontEdit { zero_case })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DEFINITIONS, FRONT};

    #[test]
    fn editable_power_front_uses_visible_2d_not_source() {
        let text = editable_facet_text(DEFINITIONS[0], FRONT);

        assert!(text.contains("• Power"));
        assert!(text.contains("loop e times"));
        assert!(text.contains("n (×)"));
        assert!(!text.contains("*Power"));
    }

    #[test]
    fn validates_preview_only_power_front_edits() {
        let text = editable_facet_text(DEFINITIONS[0], FRONT);

        assert_eq!(
            validate_facet_edit(DEFINITIONS[0], FRONT, &text),
            "Valid Power Front edit. Run uses this zero-case value for e=0; reverse projection to full source is not implemented yet."
        );
        assert!(validate_facet_edit(DEFINITIONS[0], FRONT, "").starts_with("Error:"));
        assert!(
            validate_facet_edit(DEFINITIONS[0], FRONT, "Power\nloop e times").starts_with("Error:")
        );
    }

    #[test]
    fn parses_power_front_zero_case_one() {
        let text = editable_facet_text(DEFINITIONS[0], FRONT);

        assert_eq!(
            parse_power_front_edit(&text),
            Ok(PowerFrontEdit { zero_case: 1 })
        );
    }

    #[test]
    fn parses_power_front_zero_case_two() {
        let text = editable_facet_text(DEFINITIONS[0], FRONT).replacen("1 →", "2 →", 1);

        assert_eq!(
            parse_power_front_edit(&text),
            Ok(PowerFrontEdit { zero_case: 2 })
        );
    }

    #[test]
    fn rejects_malformed_power_front_edits() {
        let missing_branch = "Power\nloop e times\nn (×)";
        let missing_arrow = editable_facet_text(DEFINITIONS[0], FRONT).replacen("1 →", "1", 1);
        let non_integer = editable_facet_text(DEFINITIONS[0], FRONT).replacen("1 →", "one →", 1);

        assert_eq!(
            parse_power_front_edit(missing_branch),
            Err("Power Front edit is missing the top branch.".to_owned())
        );
        assert_eq!(
            parse_power_front_edit(&missing_arrow),
            Err("Power Front top branch is missing `→`.".to_owned())
        );
        assert_eq!(
            parse_power_front_edit(&non_integer),
            Err("Power Front top branch zero-case `one` is not an integer.".to_owned())
        );
    }
}
