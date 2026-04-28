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

macro_rules! facet_blocks_module {
    ($name:expr, $input_a:expr, $input_b:expr, $output:expr, $front_name:expr, $front_value:expr, $left_name:expr, $left_value:expr) => {
        Module {
            facets: vec![
                Facet {
                    aspect: Aspect::Front,
                    stmts: vec![
                        Stmt::Signature(SigDecl {
                            name: (*$name).to_owned(),
                            inputs: vec![field!($input_a), field!($input_b)],
                            outputs: vec![field!($output)],
                        }),
                        Stmt::Assign(
                            Pattern::Name((*$front_name).to_owned()),
                            Expr::Int($front_value),
                        ),
                    ],
                },
                Facet {
                    aspect: Aspect::Left,
                    stmts: vec![Stmt::Assign(
                        Pattern::Name((*$left_name).to_owned()),
                        Expr::Int($left_value),
                    )],
                },
            ],
        }
    };
}

macro_rules! field {
    ($name:expr) => {
        Field {
            name: (*$name).to_owned(),
        }
    };
}

macro_rules! facet_blocks_match {
    ($tokens:expr) => {
        match $tokens {
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
                Token::AspectTag(d1_lex::AspectKind::Front),
                Token::Mint,
                Token::Ident(front_name),
                Token::LArrow,
                Token::Int(front_value),
                Token::AspectTag(d1_lex::AspectKind::Left),
                Token::Mint,
                Token::Ident(left_name),
                Token::LArrow,
                Token::Int(left_value),
                Token::Eof,
            ] => Ok(facet_blocks_module!(
                name,
                input_a,
                input_b,
                output,
                front_name,
                *front_value,
                left_name,
                *left_value
            )),
            _ => Err("expected facet blocks".to_owned()),
        }
    };
}

pub fn parse_tokens(tokens: &[Token<'_>]) -> Result<Module, String> {
    match tokens.len() {
        22 => return facet_blocks_from_tokens(tokens),
        12 => return signature_from_tokens(tokens),
        5 => return init_from_tokens(tokens),
        _ => {}
    }
    match tokens {
        [Token::Mint, Token::Ident(_), found, ..] => {
            Err(format!("expected LARROW, found {found:?}"))
        }
        [Token::Mint, found, ..] => Err(format!("expected IDENT, found {found:?}")),
        [found, ..] => Err(format!("expected MINT, found {found:?}")),
        [] => Err("expected MINT, found nothing".to_owned()),
    }
}

fn facet_blocks_from_tokens(tokens: &[Token<'_>]) -> Result<Module, String> {
    facet_blocks_match!(tokens)
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
            Err(format!("expected LARROW, found {found:?}"))
        }
        [found, ..] => Err(format!("expected mint init, found {found:?}")),
        [] => Err("expected MINT, found nothing".to_owned()),
    }
}
