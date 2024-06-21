use super::token::{Token, TokenKind};

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

pub struct Tokenize<'a> {
    full_source: &'a str,
    char_indices: Vec<(usize, char)>,
    pos: usize,
    sent_eof: bool
}

impl<'a> Tokenize<'a> {
    pub fn new(str: &'a str) -> Self {
        let char_indices = str.char_indices()
            .collect::<Vec<(usize, char)>>();
        Tokenize {
            full_source: str,
            char_indices,
            pos: 0,
            sent_eof: false
        }
    }

    fn try_collect_numeric(&mut self) -> Option<Token> {
        let (start_idx, chr) = self.char_indices[self.pos];
        if chr < '0' || chr > '9' {
            return None;
        }

        let mut end_idx = start_idx + chr.len_utf8();
        let mut mpos = self.pos + 1;

        let mut collected_period = false;
        let mut has_number_after_period = false;
        while mpos < self.char_indices.len() {
            let (next_idx, next_chr) = self.char_indices[mpos];
            debug_assert!(next_idx == end_idx);

            match next_chr {
                '.' => {
                    if collected_period {
                        return None;
                    }
                    collected_period = true;
                    end_idx += next_chr.len_utf8();
                    mpos += 1;
                },
                '0'..='9' => {
                    if collected_period {
                        has_number_after_period = true;
                    }
                    end_idx += next_chr.len_utf8();
                    mpos += 1;
                },
                _ => break
            }
        }

        if collected_period && !has_number_after_period {
            return None;
        }

        self.pos = mpos;
        Some(Token {
            source: self.full_source[start_idx..end_idx].to_owned(),
            token_kind: if collected_period { TokenKind::Float } else { TokenKind::Integer }
        })
    }

    fn try_collect_operator(&mut self) -> Option<Token> {
        let (start_idx, this_chr) = self.char_indices[self.pos];
        if SINGLE_CHAR_OPERATORS.contains(&this_chr) {
            let end_idx = start_idx + this_chr.len_utf8();
            self.pos += 1;
            Some(Token {
                source: self.full_source[start_idx..end_idx].to_owned(),
                token_kind: TokenKind::Operator
            })
        }
        else {
            None
        }
    }

    fn collect_error(&mut self) -> Token {
        let (start_idx, this_chr) = self.char_indices[self.pos];
        let mut end_idx = start_idx + this_chr.len_utf8();
        self.pos += 1;

        while self.pos < self.char_indices.len() {
            let (next_idx, next_chr) = self.char_indices[self.pos];
            debug_assert!(next_idx == end_idx);
            if next_chr.is_whitespace() || SINGLE_CHAR_OPERATORS.contains(&next_chr) {
                break;
            }
            else {
                end_idx += next_chr.len_utf8();
                self.pos += 1;
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

        while self.pos < self.char_indices.len() {
            let (_, next) = self.char_indices[self.pos];
            if next.is_whitespace() {
                self.pos += 1;
            }
            else {
                break;
            }
        }

        if self.pos == self.char_indices.len() {
            self.sent_eof = true;
            Some(Token {
                source: "".to_owned(),
                token_kind: TokenKind::EOF
            })
        }
        else {
            self.try_collect_numeric()
                .or_else(|| self.try_collect_operator())
                .or_else(|| Some(self.collect_error()))
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
