use crate::calculator::tokenizer::Token;
use super::{
    unary_expression_syntax::UnaryExpressionSyntax,
    syntax::Syntax
};

pub trait ExpressionSyntax: Syntax { }

pub fn try_parse_expression(tokens: &Vec<Token>, pos: &mut usize) -> Option<Box<dyn ExpressionSyntax>> {
    UnaryExpressionSyntax::try_parse_expression(tokens, pos)
}
