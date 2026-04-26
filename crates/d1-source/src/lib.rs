//! Voxel loader for DiscoveryOne source files.
//!
//! Parses `.d1` (layered text) and `.d1.json` (canonical
//! coordinates) into a positioned-symbol set. See
//! `docs/design.md` section 1 for the file formats.

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
