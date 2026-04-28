use crate::{ASPECT_TAGS, Token};

pub fn dump_tokens(tokens: &[Token<'_>]) -> String {
    let mut dump = String::new();
    for token in tokens {
        dump.push_str(&token_line(token));
        dump.push('\n');
    }
    dump
}

fn token_line(token: &Token<'_>) -> String {
    match token {
        Token::Mint => "MINT".to_owned(),
        Token::AspectTag(aspect) => {
            let name = ASPECT_TAGS.iter().find(|(_, kind)| kind == aspect);
            let name = name.map(|(name, _)| *name).unwrap_or("unknown");
            format!("ASPECT {name}")
        }
        Token::ZTag(value) => format!("ZTAG   {value}"),
        Token::Hash(text) => format!("HASH   {text}"),
        Token::Pct(value) => format!("PCT    {value}"),
        Token::Int(value) => format!("INT    {value}"),
        Token::Ident(text) => format!("IDENT  {text}"),
        Token::RArrow => "RARROW".to_owned(),
        Token::LArrow => "LARROW".to_owned(),
        Token::LParen => "LPAREN".to_owned(),
        Token::RParen => "RPAREN".to_owned(),
        Token::Eof => "EOF".to_owned(),
        Token::Unknown(byte) => format!("UNK    {byte}"),
    }
}
