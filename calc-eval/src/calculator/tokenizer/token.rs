use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Integer,
    Float,
    Operator,
    Identifier,
    Error,
    EOF
}

impl TokenKind {
    pub fn is_literal(self) -> bool {
        match self {
            Self::Integer |
            Self::Float => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(unused)]
pub struct Token {
    pub source: String,
    pub token_kind: TokenKind
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

impl TryFrom<&Token> for f64 {
    type Error = anyhow::Error;

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        match value.get_kind() {
            TokenKind::Integer |
            TokenKind::Float => Ok(value.source.parse::<f64>()?),
            _ => Err(anyhow::anyhow!("This token can't be interpreted as a f64"))
        }
    }
}

impl TryFrom<&Token> for i64 {
    type Error = anyhow::Error;

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        match value.get_kind() {
            TokenKind::Integer => Ok(value.source.parse::<i64>()?),
            _ => Err(anyhow::anyhow!("This token can't be interpreted as a i64"))
        }
    }
}
