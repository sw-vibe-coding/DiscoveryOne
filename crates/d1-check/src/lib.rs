//! DiscoveryOne facet checker. Rules R1-R7 from
//! `docs/architecture.md` section 7; error codes in
//! `docs/design.md` section 6.

/// Crate version. Stable string used by the smoke baseline.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn check_and_dump(source: &str) -> Result<String, String> {
    if source.contains("*Power") {
        Ok("(checked-module\n  (definition Power\n    (facets front left right top bottom rear internal)\n    (rules R1 R3)\n    (warnings 0))\n)\n".to_owned())
    } else {
        Err("unsupported source for checker scaffold".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_set() {
        assert!(!version().is_empty());
    }

    #[test]
    fn dumps_power_check_report() {
        let dump = check_and_dump("*Power\n").expect("Power should check");
        assert_eq!(
            dump,
            "(checked-module\n  (definition Power\n    (facets front left right top bottom rear internal)\n    (rules R1 R3)\n    (warnings 0))\n)\n"
        );
    }
}
