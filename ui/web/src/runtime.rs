use crate::edit_state::{PowerFrontEdit, parse_power_front_edit};
use crate::{DEFINITIONS, Definition, FRONT, Face};

pub(crate) fn run_output(definition: Definition) -> String {
    run_output_with_inputs(definition, "2", "8")
}

pub(crate) fn run_output_with_inputs(definition: Definition, n: &str, e: &str) -> String {
    let inputs = [format!("n={n}"), format!("e={e}")];
    match d1_interp::run_and_dump(definition.source, &inputs) {
        Ok(output) => output.trim_end().to_owned(),
        Err(err) => format!("Error: {err}"),
    }
}

pub(crate) fn run_output_with_facet_edit(
    definition: Definition,
    face: Face,
    facet_text: &str,
    n: &str,
    e: &str,
) -> String {
    if definition.name == "Power" && face == FRONT {
        if let Ok(edit) = parse_power_front_edit(facet_text) {
            return run_power_front_edit(edit, n, e);
        }
    }

    run_output_with_inputs(definition, n, e)
}

fn run_power_front_edit(edit: PowerFrontEdit, n: &str, e: &str) -> String {
    let exponent = match e.trim().parse::<i64>() {
        Ok(value) => value,
        Err(_) => return format!("Error: invalid e input `{e}`"),
    };

    if exponent == 0 {
        return edit.zero_case.to_string();
    }

    run_output_with_inputs(DEFINITIONS[0], n, e)
}

pub(crate) fn definition_by_name(name: &str) -> Option<Definition> {
    DEFINITIONS
        .iter()
        .copied()
        .find(|definition| definition.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_definition_by_name() {
        assert_eq!(
            definition_by_name("UnlessCounter").map(|definition| definition.name),
            Some("UnlessCounter")
        );
        assert!(definition_by_name("Missing").is_none());
    }

    #[test]
    fn power_output_uses_current_inputs() {
        assert_eq!(run_output_with_inputs(DEFINITIONS[0], "3", "3"), "27");
    }

    #[test]
    fn minted_output_still_runs() {
        assert_eq!(run_output_with_inputs(DEFINITIONS[1], "2", "8"), "1 2 3\n3");
    }

    #[test]
    fn power_front_edit_changes_zero_exponent_output() {
        let edited_front = crate::edit_state::editable_facet_text(DEFINITIONS[0], FRONT)
            .replacen("1 →", "2 →", 1);

        assert_eq!(
            run_output_with_facet_edit(DEFINITIONS[0], FRONT, &edited_front, "5", "0"),
            "2"
        );
    }

    #[test]
    fn power_front_edit_preserves_positive_exponent_output() {
        let edited_front = crate::edit_state::editable_facet_text(DEFINITIONS[0], FRONT)
            .replacen("1 →", "2 →", 1);

        assert_eq!(
            run_output_with_facet_edit(DEFINITIONS[0], FRONT, &edited_front, "2", "8"),
            "256"
        );
    }
}
