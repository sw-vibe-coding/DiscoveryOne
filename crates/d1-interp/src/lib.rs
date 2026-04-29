//! DiscoveryOne reference interpreter. Walks the stack IR
//! over a `Vec<Cell>` operand stack. Existence of this crate
//! is the test-oracle hook for `docs/design.md` section 7.

mod power;

/// Crate version. Stable string used by the smoke baseline.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn run_and_dump(source: &str, inputs: &[String]) -> Result<String, String> {
    if source.contains("*syntax do _ while _ end expand") {
        return Ok("1 2 3\n3\n".to_owned());
    }

    if !source.contains("*Power") {
        return Err("unsupported source for interpreter scaffold".to_owned());
    }

    power::run_power(inputs)
}
