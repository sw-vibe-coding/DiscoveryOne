//! DiscoveryOne reference interpreter. Walks the stack IR
//! over a `Vec<Cell>` operand stack. Existence of this crate
//! is the test-oracle hook for `docs/design.md` section 7.

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
