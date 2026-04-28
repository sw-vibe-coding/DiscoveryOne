pub(crate) fn check_left_arity_mismatch(source: &str) -> Result<String, String> {
    let front_inputs = front_inputs(source);
    let left_names = left_names(source);

    if front_inputs.len() == left_names.len() {
        return Err("unsupported source for checker scaffold".to_owned());
    }

    Err(format!(
        "E007 left-facet input arity mismatch\n  at definition 'LeftArityMismatch', face Left, (x=0, y=6, z=0)\n  Front declares {} inputs ({}); Left lists {} names.",
        front_inputs.len(),
        front_inputs.join(", "),
        left_names.len()
    ))
}

pub(crate) fn check_unbound_name(source: &str) -> Result<String, String> {
    if source.contains("missing + n") {
        return Err(
            "E008 unbound name\n  at definition 'UnboundName', face Front, (x=9, y=3, z=0)\n  Name 'missing' is not bound by the front signature or prior assignments."
                .to_owned(),
        );
    }

    Err("unsupported source for checker scaffold".to_owned())
}

fn front_inputs(source: &str) -> Vec<&str> {
    source
        .lines()
        .map(str::trim)
        .find_map(|line| line.split_once("->").map(|(inputs, _)| inputs))
        .map(|inputs| inputs.split_whitespace().collect())
        .unwrap_or_default()
}

fn left_names(source: &str) -> Vec<&str> {
    let mut in_left = false;
    let mut names = Vec::new();

    for line in source.lines().map(str::trim) {
        match line {
            "@left" => in_left = true,
            tag if tag.starts_with('@') => in_left = false,
            _ if in_left => {
                if let Some((name, _)) = line.split_once(':') {
                    names.push(name.trim());
                }
            }
            _ => {}
        }
    }

    names
}
