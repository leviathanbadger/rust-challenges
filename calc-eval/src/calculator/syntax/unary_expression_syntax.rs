use std::fmt::Display;
use crate::calculator::tokenizer::Token;
use super::{
    expression_syntax::{ExpressionPrecedence, ExpressionSyntax},
    primary_expression_syntax::PrimaryExpressionSyntax,
    syntax::Syntax
};

#[derive(Debug)]
pub enum UnaryExpressionKind {
    Plus,
    Minus
}

pub struct UnaryExpressionSyntax {
    kind: UnaryExpressionKind,
    nested_expr: Box<dyn ExpressionSyntax>
}

impl UnaryExpressionSyntax {
    pub fn try_parse_expression(tokens: &Vec<Token>, pos: &mut usize) -> Option<Box<dyn ExpressionSyntax>> {
        if *pos >= tokens.len() {
            return None;
        }

        let mut npos = *pos;
        let token = &tokens[npos];
        let mut kind = None;
        if token.is_operator("+") {
            kind = Some(UnaryExpressionKind::Plus);
            npos += 1;
        }
        else if token.is_operator("-") {
            kind = Some(UnaryExpressionKind::Minus);
            npos += 1;
        }

        let nested_expr_opt = PrimaryExpressionSyntax::try_parse_expression(tokens, &mut npos);
        if let Some(nested_expr) = nested_expr_opt {
            *pos = npos;

            if kind.is_some() {
                return Some(Box::new(UnaryExpressionSyntax {
                    kind: kind.unwrap(),
                    nested_expr: nested_expr
                }))
            }
            else {
                return Some(nested_expr)
            }
        }

        None
    }
}

impl ExpressionSyntax for UnaryExpressionSyntax {
    fn get_expression_precedence(&self) -> ExpressionPrecedence {
        ExpressionPrecedence::Unary
    }
}

impl Syntax for UnaryExpressionSyntax { }

impl Display for UnaryExpressionSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            UnaryExpressionKind::Plus => {
                write!(f, "+")?;
            },
            UnaryExpressionKind::Minus => {
                write!(f, "-")?;
            }
        };

        let needs_parenthesis = self.nested_expr.get_expression_precedence() > self.get_expression_precedence();
        if needs_parenthesis {
            write!(f, "({})", self.nested_expr)?;
        }
        else {
            write!(f, "{}", self.nested_expr)?;
        }

        Ok(())
    }
}
