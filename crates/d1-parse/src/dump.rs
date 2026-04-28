use crate::{Aspect, Expr, Facet, Module, Pattern, Stmt};

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
        Stmt::Assign(Pattern::Name(name), Expr::Int(value)) => {
            format!("(assign (name {name}) (int {value}))")
        }
    }
}
