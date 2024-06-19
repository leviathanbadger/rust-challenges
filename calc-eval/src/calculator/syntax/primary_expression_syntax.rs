use std::fmt::Display;
use crate::calculator::tokenizer::Token;
use super::{
    expression_syntax::ExpressionSyntax,
    syntax::Syntax
};

#[derive(Debug)]
pub enum PrimaryExpressionKind {
    Literal,
    ParenthesizedExpression
}

pub struct PrimaryExpressionSyntax {
    kind: PrimaryExpressionKind,
    literal_token: Option<Token>,
    nested_expr: Option<Box<dyn ExpressionSyntax>>
}

impl PrimaryExpressionSyntax {
    pub fn try_parse_expression(tokens: &Vec<Token>, pos: &mut usize) -> Option<Box<dyn ExpressionSyntax>> {
        if *pos >= tokens.len() {
            return None;
        }

        let token = &tokens[*pos];
        if token.is_literal() {
            *pos += 1;

            return Some(Box::new(PrimaryExpressionSyntax {
                kind: PrimaryExpressionKind::Literal,
                literal_token: Some(token.clone()),
                nested_expr: None
            }))
        }

        if token.is_operator("(") {
            let mut npos = *pos + 1;
            let nested_expr_opt = super::expression_syntax::try_parse_expression(tokens, &mut npos);
            if let Some(nested_expr) = nested_expr_opt {
                if npos < tokens.len() && tokens[npos].is_operator(")") {
                    *pos = npos + 1;
                    return Some(Box::new(PrimaryExpressionSyntax {
                        kind: PrimaryExpressionKind::ParenthesizedExpression,
                        literal_token: None,
                        nested_expr: Some(nested_expr)
                    }))
                }
            }
        }

        None
    }
}

impl ExpressionSyntax for PrimaryExpressionSyntax { }

impl Syntax for PrimaryExpressionSyntax { }

impl Display for PrimaryExpressionSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            PrimaryExpressionKind::Literal => {
                self.literal_token.as_ref().unwrap().repr(f)?;
            },
            PrimaryExpressionKind::ParenthesizedExpression => {
                write!(f, "({})", self.nested_expr.as_ref().unwrap())?;
            }
        };

        Ok(())
    }
}
