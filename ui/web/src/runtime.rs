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

pub(crate) fn validate_source(source: &str) -> String {
    if source.contains("*Power") {
        return match d1_check::check_and_dump(source) {
            Ok(_) => "Valid: parsed and checked.".to_owned(),
            Err(err) => format!("Error: check failed: {err}"),
        };
    }

    if let Err(err) = d1_parse::parse(source) {
        return format!("Error: parse failed: {err}");
    }

    "Valid: parsed. Checker scaffold is not available for this definition yet.".to_owned()
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
    fn validates_power_source() {
        assert_eq!(validate_source(DEFINITIONS[0].source), "Valid: parsed and checked.");
    }

    #[test]
    fn reports_parse_error_for_invalid_source() {
        assert!(validate_source("not d1").starts_with("Error: parse failed:"));
    }
}
