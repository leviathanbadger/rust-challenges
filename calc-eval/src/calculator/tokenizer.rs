use std::{
    iter::Peekable,
    str::CharIndices,
    fmt::Display
};

macro_rules! count {
    ( ) => {
        0usize
    };
    ( $throwaway:tt $( $val:expr )* ) => {
        1usize + count![ $( $val )* ]
    };
}
macro_rules! char_array_const {
    ( $( $name:ident = [ $( $val:expr ),* ] ; )* ) => {
        $(
            const $name: [char; count![ $( $val )* ]] = [
                $( $val ),*
            ];
        )*
    };
}

char_array_const!
{
    SINGLE_CHAR_OPERATORS = [
        '+',
        '-',
        '*',
        '/',
        '%',
        '(',
        ')'
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Integer,
    Operator,
    Error,
    EOF
}

impl TokenKind {
    pub fn is_literal(self) -> bool {
        match self {
            Self::Integer => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Token {
    source: String,
    token_kind: TokenKind
}

impl Token {
    pub fn get_kind(&self) -> TokenKind {
        self.token_kind
    }

    pub fn is_literal(&self) -> bool {
        self.get_kind().is_literal()
    }

    pub fn is_operator(&self, op: &str) -> bool {
        self.get_kind() == TokenKind::Operator && op == self.source
    }

    pub fn repr(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.source))
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("<{:?}; {}>", self.token_kind, self.source))
    }
}

pub struct Tokenize<'a> {
    full_source: &'a str,
    char_indices: Peekable<CharIndices<'a>>,
    sent_eof: bool
}

impl<'a> Tokenize<'a> {
    pub fn new(str: &'a str) -> Self {
        Tokenize {
            full_source: str,
            char_indices: str.char_indices().peekable(),
            sent_eof: false
        }
    }

    fn try_collect_numeric(&mut self, start_idx: usize, chr: char) -> Option<Token> {
        if chr < '0' || chr > '9' {
            return None;
        }
        let mut end_idx: usize = start_idx + chr.len_utf8();
        loop {
            let next = self.char_indices.peek();
            if let Some((next_idx, next_chr)) = next {
                debug_assert!(*next_idx == end_idx);
                if *next_chr < '0' || *next_chr > '9' {
                    break;
                }
                else {
                    //Consume peeked token
                    end_idx += next_chr.len_utf8();
                    self.char_indices.next();
                }
            }
            else {
                break;
            }
        }
        Some(Token {
            source: self.full_source[start_idx..end_idx].to_owned(),
            token_kind: TokenKind::Integer
        })
    }

    fn try_collect_operator(&mut self, start_idx: usize, chr: char) -> Option<Token> {
        if SINGLE_CHAR_OPERATORS.contains(&chr) {
            let end_idx = start_idx + chr.len_utf8();
            Some(Token {
                source: self.full_source[start_idx..end_idx].to_owned(),
                token_kind: TokenKind::Operator
            })
        }
        else {
            None
        }
    }

    fn collect_error(&mut self, start_idx: usize, chr: char) -> Token {
        let mut end_idx = start_idx + chr.len_utf8();
        loop {
            let next = self.char_indices.peek();
            if let Some((next_idx, next_chr)) = next {
                debug_assert!(*next_idx == end_idx);
                if next_chr.is_whitespace() || SINGLE_CHAR_OPERATORS.contains(next_chr) {
                    break;
                }
                else {
                    //Consume peeked token
                    end_idx += next_chr.len_utf8();
                    self.char_indices.next();
                }
            }
            else {
                break;
            }
        }
        Token {
            source: self.full_source[start_idx..end_idx].to_owned(),
            token_kind: TokenKind::Error
        }
    }

    fn try_collect_token(&mut self) -> Option<Token> {
        if self.sent_eof {
            return None
        }

        loop {
            let next = self.char_indices.next();
            match next {
                None => {
                    self.sent_eof = true;
                    return Some(Token {
                        source: self.full_source[self.full_source.len()..].to_owned(),
                        token_kind: TokenKind::EOF
                    })
                },
                Some((start_idx, chr)) => {
                    if chr.is_whitespace() {
                        continue;
                    }
                    return self.try_collect_numeric(start_idx, chr)
                        .or_else(|| self.try_collect_operator(start_idx, chr))
                        .or_else(|| Some(self.collect_error(start_idx, chr)))
                }
            }
        }
    }
}

impl<'a> Iterator for Tokenize<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_collect_token()
    }
}

pub struct Tokenizer {
}

impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer { }
    }

    pub fn tokenize<'a>(&self, str: &'a str) -> Tokenize<'a> {
        Tokenize::new(str)
    }
}
