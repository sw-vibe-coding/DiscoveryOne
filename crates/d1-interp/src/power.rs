pub(crate) fn run_power(inputs: &[String]) -> Result<String, String> {
    let n = input_value(inputs, "n")?;
    let e = input_value(inputs, "e")?;
    let mut p = 1_i64;

    for _ in 0..e {
        p *= n;
    }

    Ok(format!("{p}\n"))
}

fn input_value(inputs: &[String], name: &str) -> Result<i64, String> {
    let input = inputs
        .iter()
        .filter_map(|input| input.split_once('='))
        .find(|(key, _)| *key == name)
        .ok_or_else(|| format!("missing input '{name}'"))?;

    input
        .1
        .parse::<i64>()
        .map_err(|err| format!("invalid input '{name}': {err}"))
}
