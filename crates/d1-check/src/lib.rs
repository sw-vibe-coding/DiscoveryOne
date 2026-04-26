//! DiscoveryOne facet checker. Rules R1-R7 from
//! `docs/architecture.md` section 7; error codes in
//! `docs/design.md` section 6.

/// Crate version. Stable string used by the smoke baseline.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_set() {
        assert!(!version().is_empty());
    }
}
