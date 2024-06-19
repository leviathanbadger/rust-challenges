use crate::calculator::tokenizer::Token;
use super::{
    primary_expression_syntax::PrimaryExpressionSyntax,
    syntax::Syntax
};

pub trait ExpressionSyntax: Syntax { }

pub fn try_parse_expression(tokens: &Vec<Token>, pos: &mut usize) -> Option<Box<dyn ExpressionSyntax>> {
    PrimaryExpressionSyntax::try_parse_expression(tokens, pos)
}
