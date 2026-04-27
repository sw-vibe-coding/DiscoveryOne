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
        b'@' => {
            let (start, end) = take_word(bytes, offset + 1);
            let aspect = ASPECT_TAGS
                .iter()
                .find(|(name, _)| *name == &source[start..end])
                .map(|(_, aspect)| *aspect);
            aspect.map_or((Token::Unknown(byte), offset + 1), |aspect| {
                (Token::AspectTag(aspect), end)
            })
        }
        b'(' => (Token::LParen, offset + 1),
        b')' => (Token::RParen, offset + 1),
        b'-' if bytes.get(offset + 1) == Some(&b'>') => (Token::RArrow, offset + 2),
        b if b.is_ascii_alphabetic() || b == b'_' => {
            let (start, end) = take_word(bytes, offset);
            (Token::Ident(&source[start..end]), end)
        }
        b => (Token::Unknown(b), offset + 1),
    }
}

fn take_word(bytes: &[u8], start: usize) -> (usize, usize) {
    let mut end = start + 1;
    while bytes
        .get(end)
        .is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'_')
    {
        end += 1;
    }
    (start, end)
}

pub fn dump_tokens(tokens: &[Token<'_>]) -> String {
    let mut dump = String::new();
    for token in tokens {
        match token {
            Token::Mint => dump.push_str("MINT"),
            Token::AspectTag(aspect) => {
                let name = ASPECT_TAGS
                    .iter()
                    .find(|(_, kind)| kind == aspect)
                    .map(|(name, _)| *name)
                    .unwrap_or("unknown");
                dump.push_str(&format!("ASPECT {name}"));
            }
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
