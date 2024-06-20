use std::fmt::Display;
use crate::calculator::tokenizer::Token;
use super::{
    expression_syntax::{ExpressionPrecedence, ExpressionSyntax},
    multiplicative_expression_syntax::MultiplicativeExpressionSyntax,
    syntax::Syntax
};

#[derive(Debug)]
pub enum AdditiveExpressionKind {
    Add,
    Subtract
}

pub struct AdditiveExpressionSyntax {
    kind: AdditiveExpressionKind,
    left_expr: Box<dyn ExpressionSyntax>,
    right_expr: Box<dyn ExpressionSyntax>
}

impl AdditiveExpressionSyntax {
    pub fn try_parse_expression(tokens: &Vec<Token>, pos: &mut usize) -> Option<Box<dyn ExpressionSyntax>> {
        let mut expr_opt = MultiplicativeExpressionSyntax::try_parse_expression(tokens, pos);
        if expr_opt.is_none() {
            return None
        }

        let mut npos = *pos;
        while npos < tokens.len() - 1 {
            let token = &tokens[npos];
            let mut kind = None;
            if token.is_operator("+") {
                kind = Some(AdditiveExpressionKind::Add);
            }
            else if token.is_operator("-") {
                kind = Some(AdditiveExpressionKind::Subtract);
            }

            if kind.is_none() {
                break;
            }

            npos += 1;

            let rhs_expr_opt = MultiplicativeExpressionSyntax::try_parse_expression(tokens, &mut npos);
            if rhs_expr_opt.is_none() {
                break;
            }

            *pos = npos;
            expr_opt = Some(Box::new(AdditiveExpressionSyntax {
                kind: kind.unwrap(),
                left_expr: expr_opt.unwrap(),
                right_expr: rhs_expr_opt.unwrap()
            }));
        }

        expr_opt
    }
}

impl ExpressionSyntax for AdditiveExpressionSyntax {
    fn get_expression_precedence(&self) -> ExpressionPrecedence {
        ExpressionPrecedence::Additive
    }
}

impl Syntax for AdditiveExpressionSyntax { }

impl Display for AdditiveExpressionSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        {
            let needs_parenthesis = self.left_expr.get_expression_precedence() > self.get_expression_precedence();
            if needs_parenthesis {
                write!(f, "({})", self.left_expr)?;
            }
            else {
                write!(f, "{}", self.left_expr)?;
            }
        }

        match self.kind {
            AdditiveExpressionKind::Add => {
                write!(f, " + ")?;
            },
            AdditiveExpressionKind::Subtract => {
                write!(f, " - ")?;
            }
        };

        {
            let needs_parenthesis = self.right_expr.get_expression_precedence() >= self.get_expression_precedence();
            if needs_parenthesis {
                write!(f, "({})", self.right_expr)?;
            }
            else {
                write!(f, "{}", self.right_expr)?;
            }
        }

        Ok(())
    }
}
