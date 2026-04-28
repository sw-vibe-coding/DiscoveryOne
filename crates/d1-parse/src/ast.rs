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
    Signature(SigDecl),
    Assign(Pattern, Expr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: Name,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigDecl {
    pub name: Name,
    pub inputs: Vec<Field>,
    pub outputs: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Facet {
    pub aspect: Aspect,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Aspect {
    Front,
    Left,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub facets: Vec<Facet>,
}
