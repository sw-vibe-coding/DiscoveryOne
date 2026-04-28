use crate::{Aspect, Expr, Facet, Module, Pattern, Stmt};
use d1_lex::Token;

pub fn parse(source: &str) -> Result<Module, String> {
    let tokens = d1_lex::lex(source);
    match tokens.as_slice() {
        [
            Token::Mint,
            Token::Ident(name),
            Token::LArrow,
            Token::Int(value),
            Token::Eof,
        ] => Ok(mint_init_module(name, *value)),
        [Token::Mint, Token::Ident(_), found, ..] => Err(format!(
            "expected LARROW, found {}",
            describe_token(Some(*found))
        )),
        [Token::Mint, found, ..] => Err(format!(
            "expected IDENT, found {}",
            describe_token(Some(*found))
        )),
        [found, ..] => Err(format!(
            "expected MINT, found {}",
            describe_token(Some(*found))
        )),
        [] => Err("expected MINT, found nothing".to_owned()),
    }
}

fn mint_init_module(name: &str, value: i64) -> Module {
    Module {
        facets: vec![Facet {
            aspect: Aspect::Front,
            stmts: vec![Stmt::Assign(
                Pattern::Name(name.to_owned()),
                Expr::Int(value),
            )],
        }],
    }
}

fn describe_token(token: Option<Token<'_>>) -> String {
    match token {
        Some(Token::Mint) => "MINT".to_owned(),
        Some(Token::Ident(name)) => format!("IDENT {name}"),
        Some(Token::LArrow) => "LARROW".to_owned(),
        Some(Token::Int(value)) => format!("INT {value}"),
        Some(Token::Eof) => "EOF".to_owned(),
        Some(other) => format!("{other:?}"),
        None => "nothing".to_owned(),
    }
}
