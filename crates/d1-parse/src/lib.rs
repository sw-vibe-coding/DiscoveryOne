//! DiscoveryOne parser. AST shape defined in
//! `docs/design.md` section 4.

mod ast;
mod dump;
mod parser;

pub use ast::{Aspect, Expr, Facet, Module, Name, Pattern, Stmt};
pub use dump::dump_module;
pub use parser::parse;

/// Crate version. Stable string used by the smoke baseline.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn parse_and_dump(source: &str) -> Result<String, String> {
    parse(source).map(|module| dump_module(&module))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_set() {
        assert!(!version().is_empty());
    }
}
