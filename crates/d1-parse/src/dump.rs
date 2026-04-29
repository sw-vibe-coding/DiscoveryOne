use crate::{Aspect, Expr, Facet, Module, Pattern, SigDecl, Stmt};

pub fn dump_module(module: &Module) -> String {
    let mut dump = String::new();
    dump.push_str("(module\n");
    for facet in &module.facets {
        dump.push_str("  ");
        dump.push_str(&dump_facet(facet));
        dump.push('\n');
    }
    dump.push_str(")\n");
    dump
}

fn dump_facet(facet: &Facet) -> String {
    let aspect = match facet.aspect {
        Aspect::Front => "front",
        Aspect::Left => "left",
        Aspect::Internal => "internal",
    };
    let mut dump = format!("(facet {aspect}");
    for stmt in &facet.stmts {
        dump.push('\n');
        dump.push_str("    ");
        dump.push_str(&dump_stmt(stmt));
    }
    dump.push(')');
    dump
}

fn dump_stmt(stmt: &Stmt) -> String {
    match stmt {
        Stmt::Signature(sig) => dump_signature(sig),
        Stmt::Assign(Pattern::Name(name), Expr::Int(value)) => {
            format!("(assign (name {name}) (int {value}))")
        }
        Stmt::Syntax(syntax) => format!(
            "(syntax (pattern {}) (expansion {}))",
            syntax.pattern.join(" "),
            syntax.expansion.join(" ")
        ),
    }
}

fn dump_signature(sig: &SigDecl) -> String {
    let inputs = sig
        .inputs
        .iter()
        .map(|field| field.name.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    let outputs = sig
        .outputs
        .iter()
        .map(|field| field.name.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "(signature {} (inputs {}) (outputs {}))",
        sig.name, inputs, outputs
    )
}
