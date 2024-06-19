use crate::calculator::tokenizer::Token;
use super::{
    multiplicative_expression_syntax::MultiplicativeExpressionSyntax,
    syntax::Syntax
};

pub trait ExpressionSyntax: Syntax { }

pub fn try_parse_expression(tokens: &Vec<Token>, pos: &mut usize) -> Option<Box<dyn ExpressionSyntax>> {
    MultiplicativeExpressionSyntax::try_parse_expression(tokens, pos)
}
