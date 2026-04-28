pub type Name = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Int(i64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    Name(Name),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Assign(Pattern, Expr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Facet {
    pub aspect: Aspect,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Aspect {
    Front,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub facets: Vec<Facet>,
}
