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
        '(',
        ')'
    ];
}

pub struct Tokenizer {
}

#[derive(Debug)]
pub enum TokenType {
    Integer,
    Operator,
    Error,
    EOF
}
#[derive(Debug)]
#[allow(unused)]
pub struct Token<'a> {
    source: &'a str,
    token_type: TokenType
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("<{:?}; {:?}>", self.token_type, self.source))
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

    fn try_collect_numeric(&mut self, start_idx: usize, chr: char) -> Option<Token<'a>> {
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
            source: &self.full_source[start_idx..end_idx],
            token_type: TokenType::Integer
        })
    }

    fn try_collect_operator(&mut self, start_idx: usize, chr: char) -> Option<Token<'a>> {
        if SINGLE_CHAR_OPERATORS.contains(&chr) {
            let end_idx = start_idx + chr.len_utf8();
            Some(Token {
                source: &self.full_source[start_idx..end_idx],
                token_type: TokenType::Operator
            })
        }
        else {
            None
        }
    }

    fn collect_error(&mut self, start_idx: usize, chr: char) -> Token<'a> {
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
            source: &self.full_source[start_idx..end_idx],
            token_type: TokenType::Error
        }
    }

    fn try_collect_token(&mut self) -> Option<Token<'a>> {
        if self.sent_eof {
            return None
        }

        loop {
            let next = self.char_indices.next();
            match next {
                None => {
                    self.sent_eof = true;
                    return Some(Token {
                        source: &self.full_source[self.full_source.len()..],
                        token_type: TokenType::EOF
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
    type Item = Token<'a>;

    #[allow(unused)]
    fn next(&mut self) -> Option<Self::Item> {
        self.try_collect_token()
    }
}

impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer { }
    }

    pub fn tokenize<'a>(&self, str: &'a str) -> Tokenize<'a> {
        Tokenize::new(str)
    }
}
