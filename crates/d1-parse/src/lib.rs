//! DiscoveryOne parser. AST shape defined in
//! `docs/design.md` section 4.

mod ast;
mod dump;
mod tokens;

pub use ast::{Aspect, Expr, Facet, Field, Module, Name, Pattern, SigDecl, Stmt, SyntaxDecl};
pub use dump::dump_module;

/// Crate version. Stable string used by the smoke baseline.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn parse(source: &str) -> Result<Module, String> {
    if let Some((pattern, expansion)) = tokens::syntax_template(source) {
        return Ok(Module {
            facets: vec![Facet {
                aspect: Aspect::Internal,
                stmts: vec![Stmt::Syntax(SyntaxDecl { pattern, expansion })],
            }],
        });
    }

    let tokens = d1_lex::lex(source);
    tokens::parse_tokens(&tokens)
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
