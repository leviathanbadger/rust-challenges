use crate::calculator::tokenizer::Token;
use super::{
    additive_expression_syntax::AdditiveExpressionSyntax,
    syntax::Syntax
};

pub trait ExpressionSyntax: Syntax { }

pub fn try_parse_expression(tokens: &Vec<Token>, pos: &mut usize) -> Option<Box<dyn ExpressionSyntax>> {
    AdditiveExpressionSyntax::try_parse_expression(tokens, pos)
}
