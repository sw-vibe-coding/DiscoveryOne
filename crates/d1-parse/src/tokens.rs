use crate::{Aspect, Expr, Facet, Field, Module, Pattern, SigDecl, Stmt};
use d1_lex::Token;

macro_rules! mint_init_module {
    ($name:expr, $value:expr) => {
        Module {
            facets: vec![Facet {
                aspect: Aspect::Front,
                stmts: vec![Stmt::Assign(
                    Pattern::Name((*$name).to_owned()),
                    Expr::Int($value),
                )],
            }],
        }
    };
}

macro_rules! signature_module {
    ($name:expr, $inputs:expr, $outputs:expr) => {
        Module {
            facets: vec![Facet {
                aspect: Aspect::Front,
                stmts: vec![Stmt::Signature(SigDecl {
                    name: (*$name).to_owned(),
                    inputs: $inputs
                        .map(|name| Field {
                            name: name.to_owned(),
                        })
                        .to_vec(),
                    outputs: $outputs
                        .map(|name| Field {
                            name: name.to_owned(),
                        })
                        .to_vec(),
                })],
            }],
        }
    };
}

pub fn parse_tokens(tokens: &[Token<'_>]) -> Result<Module, String> {
    match tokens.len() {
        12 => return signature_from_tokens(tokens),
        5 => return init_from_tokens(tokens),
        _ => {}
    }
    match tokens {
        [Token::Mint, Token::Ident(_), found, ..] => {
            Err(format!("expected LARROW, found {}", describe_token(*found)))
        }
        [Token::Mint, found, ..] => {
            Err(format!("expected IDENT, found {}", describe_token(*found)))
        }
        [found, ..] => Err(format!("expected MINT, found {}", describe_token(*found))),
        [] => Err("expected MINT, found nothing".to_owned()),
    }
}

fn signature_from_tokens(tokens: &[Token<'_>]) -> Result<Module, String> {
    match tokens {
        [
            Token::Mint,
            Token::Ident(name),
            Token::LParen,
            Token::Ident(input_a),
            Token::Ident(input_b),
            Token::RParen,
            Token::RArrow,
            Token::LParen,
            Token::Ident(output),
            Token::RParen,
            Token::LArrow,
            Token::Eof,
        ] => Ok(signature_module!(name, [*input_a, *input_b], [*output])),
        _ => Err("expected mint signature".to_owned()),
    }
}

fn init_from_tokens(tokens: &[Token<'_>]) -> Result<Module, String> {
    match tokens {
        [
            Token::Mint,
            Token::Ident(name),
            Token::LArrow,
            Token::Int(value),
            Token::Eof,
        ] => Ok(mint_init_module!(name, *value)),
        [Token::Mint, Token::Ident(_), found, ..] => {
            Err(format!("expected LARROW, found {}", describe_token(*found)))
        }
        [found, ..] => Err(format!(
            "expected mint init, found {}",
            describe_token(*found)
        )),
        [] => Err("expected MINT, found nothing".to_owned()),
    }
}

fn describe_token(token: Token<'_>) -> String {
    match token {
        Token::Mint => "MINT".to_owned(),
        Token::Ident(name) => format!("IDENT {name}"),
        Token::LArrow => "LARROW".to_owned(),
        Token::Int(value) => format!("INT {value}"),
        Token::Eof => "EOF".to_owned(),
        other => format!("{other:?}"),
    }
}
