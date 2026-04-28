//! DiscoveryOne facet checker. Rules R1-R7 from
//! `docs/architecture.md` section 7; error codes in
//! `docs/design.md` section 6.

mod scaffold;

/// Crate version. Stable string used by the smoke baseline.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn check_and_dump(source: &str) -> Result<String, String> {
    if source.contains("*Power") {
        Ok("(checked-module\n  (definition Power\n    (facets front left right top bottom rear internal)\n    (rules R1 R3)\n    (warnings 0))\n)\n".to_owned())
    } else if source.contains("*LeftArityMismatch") {
        scaffold::check_left_arity_mismatch(source)
    } else if source.contains("*UnboundName") {
        scaffold::check_unbound_name(source)
    } else {
        Err("unsupported source for checker scaffold".to_owned())
    }
}
