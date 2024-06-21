use anyhow::*;
use crate::calculator::{
    interpreter::MethodBuilder,
    tokenizer::Token
};
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

    fn emit_bytecode(&self, method_builder: &mut MethodBuilder) -> Result<()>;
}

pub fn try_parse_expression(tokens: &Vec<Token>, pos: &mut usize) -> Option<Box<dyn ExpressionSyntax>> {
    AdditiveExpressionSyntax::try_parse_expression(tokens, pos)
}

#[cfg(test)]
mod tests {
    use crate::calculator::tokenizer::Tokenizer;
    use super::*;

    #[test]
    fn try_parse_expression_should_correctly_parse_expressions() {
        let test_cases: &[(&str, &str)] = &[
            //Primary
            ("25", "25"),
            ("2.5", "2.5"),
            ("(((1)))", "1"),

            //Unary
            ("-42", "-42"),
            ("-(42)", "-42"),
            ("+-+-+42", "+-+-+42"),

            //Multiplicative
            ("1*2/3%4", "1 * 2 / 3 % 4"),
            ("1*(2/(3%4))", "1 * (2 / (3 % 4))"),
            ("-1*-2", "-1 * -2"),
            ("-(1*2)", "-(1 * 2)"),

            //Additive
            ("1+2-3", "1 + 2 - 3"),
            ("1+(2-3)", "1 + (2 - 3)"),
            ("-1+-2", "-1 + -2"),
            ("-(1+2)", "-(1 + 2)"),
            ("1*2+3*4", "1 * 2 + 3 * 4"),
            ("1*(2+3)*4", "1 * (2 + 3) * 4"),
            ("1+2/3-4", "1 + 2 / 3 - 4"),
            ("(1+2)/(3-4)", "(1 + 2) / (3 - 4)"),
        ];

        let tokenizer = Tokenizer::new();

        for &(input, expected_output) in test_cases {
            let tokens = tokenizer.tokenize(input).collect();

            let mut pos = 0;
            let expr = try_parse_expression(&tokens, &mut pos);

            assert!(expr.is_some());
            assert_eq!(pos, tokens.len() - 1);
            assert_eq!(expr.unwrap().to_string().as_str(), expected_output);
        }
    }
}
