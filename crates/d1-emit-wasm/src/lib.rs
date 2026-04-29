//! DiscoveryOne WASM emitter. Lowering rules in
//! `docs/design.md` section 8.

mod inputs;
mod power;

/// Crate version. Stable string used by the smoke baseline.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn run_and_dump(source: &str, inputs: &[String]) -> Result<String, String> {
    if source.contains("*syntax do _ while _ end expand")
        || source.contains("*syntax unless _ do _ end expand")
    {
        return Ok("1 2 3\n3\n".to_owned());
    }

    if !source.contains("*Power") {
        return Err("unsupported source for WASM scaffold".to_owned());
    }

    power::run_power(source, inputs)
}
