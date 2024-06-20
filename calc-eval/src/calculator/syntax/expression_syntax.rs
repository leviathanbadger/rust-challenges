use crate::calculator::tokenizer::Token;
use super::{
    additive_expression_syntax::AdditiveExpressionSyntax,
    syntax::Syntax
};

#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd)]
#[repr(u8)]
pub enum ExpressionPrecedence {
    Primary = 0,
    Unary = 1,
    Multiplicative = 2,
    Additive = 3
}

impl Ord for ExpressionPrecedence {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

pub trait ExpressionSyntax: Syntax {
    fn get_expression_precedence(&self) -> ExpressionPrecedence;
}

pub fn try_parse_expression(tokens: &Vec<Token>, pos: &mut usize) -> Option<Box<dyn ExpressionSyntax>> {
    AdditiveExpressionSyntax::try_parse_expression(tokens, pos)
}
