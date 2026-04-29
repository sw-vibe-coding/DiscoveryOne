use crate::{DEFINITIONS, Definition};

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
}
