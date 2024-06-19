use std::fmt::Display;
use crate::calculator::tokenizer::Token;
use super::{
    expression_syntax::ExpressionSyntax,
    syntax::Syntax,
    unary_expression_syntax::UnaryExpressionSyntax
};

#[derive(Debug)]
pub enum MultiplicativeExpressionKind {
    Multiply,
    Divide,
    Modulus
}

pub struct MultiplicativeExpressionSyntax {
    kind: MultiplicativeExpressionKind,
    left_expr: Box<dyn ExpressionSyntax>,
    right_expr: Box<dyn ExpressionSyntax>
}

impl MultiplicativeExpressionSyntax {
    pub fn try_parse_expression(tokens: &Vec<Token>, pos: &mut usize) -> Option<Box<dyn ExpressionSyntax>> {
        let mut expr_opt = UnaryExpressionSyntax::try_parse_expression(tokens, pos);
        if expr_opt.is_none() {
            return None
        }

        let mut npos = *pos;
        while npos < tokens.len() - 1 {
            let token = &tokens[npos];
            let mut kind = None;
            if token.is_operator("*") {
                kind = Some(MultiplicativeExpressionKind::Multiply);
            }
            else if token.is_operator("/") {
                kind = Some(MultiplicativeExpressionKind::Divide);
            }
            else if token.is_operator("%") {
                kind = Some(MultiplicativeExpressionKind::Modulus);
            }

            if kind.is_none() {
                break;
            }

            npos += 1;

            let rhs_expr_opt = UnaryExpressionSyntax::try_parse_expression(tokens, &mut npos);
            if rhs_expr_opt.is_none() {
                break;
            }

            *pos = npos;
            expr_opt = Some(Box::new(MultiplicativeExpressionSyntax {
                kind: kind.unwrap(),
                left_expr: expr_opt.unwrap(),
                right_expr: rhs_expr_opt.unwrap()
            }));
        }

        expr_opt
    }
}

impl ExpressionSyntax for MultiplicativeExpressionSyntax { }

impl Syntax for MultiplicativeExpressionSyntax { }

impl Display for MultiplicativeExpressionSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            MultiplicativeExpressionKind::Multiply => {
                write!(f, "{} * {}", self.left_expr, self.right_expr)?;
            },
            MultiplicativeExpressionKind::Divide => {
                write!(f, "{} / {}", self.left_expr, self.right_expr)?;
            },
            MultiplicativeExpressionKind::Modulus => {
                write!(f, "{} % {}", self.left_expr, self.right_expr)?;
            }
        };

        Ok(())
    }
}
