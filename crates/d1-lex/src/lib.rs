//! DiscoveryOne lexer. Token set defined in
//! `docs/design.md` section 3; arrives in saga
//! `discoveryone-lex` (M1).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectKind {
    Front,
    Left,
    Right,
    Top,
    Bottom,
    Rear,
    Internal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'src> {
    Mint,
    AspectTag(AspectKind),
    ZTag(i32),
    Hash(&'src str),
    Ident(&'src str),
    RArrow,
    LParen,
    RParen,
    Eof,
    Unknown(u8),
}

const ASPECT_TAGS: [(&str, AspectKind); 7] = [
    ("front", AspectKind::Front),
    ("left", AspectKind::Left),
    ("right", AspectKind::Right),
    ("top", AspectKind::Top),
    ("bottom", AspectKind::Bottom),
    ("rear", AspectKind::Rear),
    ("internal", AspectKind::Internal),
];

pub fn lex(source: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();
    let mut offset = 0;
    let bytes = source.as_bytes();

    while offset < bytes.len() {
        if bytes[offset].is_ascii_whitespace() {
            offset += 1;
            continue;
        }
        let (token, next) = lex_one(source, bytes, offset);
        tokens.push(token);
        offset = next;
    }

    tokens.push(Token::Eof);
    tokens
}

fn lex_one<'src>(source: &'src str, bytes: &[u8], offset: usize) -> (Token<'src>, usize) {
    let byte = bytes[offset];
    match byte {
        b'*' => (Token::Mint, offset + 1),
        b'@' => lex_at(source, bytes, offset),
        b'#' => {
            let end = source[offset..]
                .find('\n')
                .map_or(source.len(), |i| offset + i);
            (Token::Hash(&source[offset..end]), end)
        }
        b'(' => (Token::LParen, offset + 1),
        b')' => (Token::RParen, offset + 1),
        b'-' if bytes.get(offset + 1) == Some(&b'>') => (Token::RArrow, offset + 2),
        b if b.is_ascii_alphabetic() || b == b'_' => {
            let tail = &source[offset..];
            let end = tail
                .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                .map_or(source.len(), |i| offset + i);
            (Token::Ident(&source[offset..end]), end)
        }
        b => (Token::Unknown(b), offset + 1),
    }
}

fn lex_at<'src>(source: &'src str, bytes: &[u8], offset: usize) -> (Token<'src>, usize) {
    let mut end = offset + 2;
    while bytes.get(end).is_some_and(|b| b.is_ascii_alphabetic()) {
        end += 1;
    }
    if &source[offset + 1..end] == "z" {
        let mut start = end;
        while bytes.get(start).is_some_and(|b| b.is_ascii_whitespace()) {
            start += 1;
        }
        end = start;
        while bytes.get(end).is_some_and(|b| b.is_ascii_digit()) {
            end += 1;
        }
        return (Token::ZTag(source[start..end].parse().unwrap_or(0)), end);
    }
    ASPECT_TAGS
        .iter()
        .find(|(name, _)| *name == &source[offset + 1..end])
        .map_or((Token::Unknown(b'@'), offset + 1), |(_, aspect)| {
            (Token::AspectTag(*aspect), end)
        })
}

pub fn dump_tokens(tokens: &[Token<'_>]) -> String {
    let mut dump = String::new();
    for token in tokens {
        match token {
            Token::Mint => dump.push_str("MINT"),
            Token::AspectTag(aspect) => {
                let name = ASPECT_TAGS.iter().find(|(_, kind)| kind == aspect);
                let name = name.map(|(name, _)| *name).unwrap_or("unknown");
                dump.push_str(&format!("ASPECT {name}"));
            }
            Token::ZTag(value) => dump.push_str(&format!("ZTAG   {value}")),
            Token::Hash(text) => dump.push_str(&format!("HASH   {text}")),
            Token::Ident(text) => dump.push_str(&format!("IDENT  {text}")),
            Token::RArrow => dump.push_str("RARROW"),
            Token::LParen => dump.push_str("LPAREN"),
            Token::RParen => dump.push_str("RPAREN"),
            Token::Eof => dump.push_str("EOF"),
            Token::Unknown(byte) => dump.push_str(&format!("UNK    {byte}")),
        }
        dump.push('\n');
    }
    dump
}
