//! DiscoveryOne lexer. Token set defined in
//! `docs/design.md` section 3; arrives in saga
//! `discoveryone-lex` (M1).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'src> {
    Mint,
    Ident(&'src str),
    RArrow,
    LParen,
    RParen,
    Eof,
    Unknown(u8),
}

type Tokens<'src> = Vec<Token<'src>>;

pub fn lex(source: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();
    let mut offset = 0;
    let bytes = source.as_bytes();

    while offset < bytes.len() {
        offset = lex_one(source, bytes, offset, &mut tokens);
    }

    tokens.push(Token::Eof);
    tokens
}

fn lex_one<'src>(
    source: &'src str,
    bytes: &[u8],
    offset: usize,
    tokens: &mut Tokens<'src>,
) -> usize {
    let byte = bytes[offset];
    if byte.is_ascii_whitespace() {
        return offset + 1;
    }

    let (token, next) = match byte {
        b'*' => (Token::Mint, offset + 1),
        b'(' => (Token::LParen, offset + 1),
        b')' => (Token::RParen, offset + 1),
        b'-' if bytes.get(offset + 1) == Some(&b'>') => (Token::RArrow, offset + 2),
        b if b.is_ascii_alphabetic() || b == b'_' => {
            let (start, end) = take_ident(bytes, offset);
            (Token::Ident(&source[start..end]), end)
        }
        b => (Token::Unknown(b), offset + 1),
    };
    tokens.push(token);
    next
}

fn take_ident(bytes: &[u8], start: usize) -> (usize, usize) {
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
