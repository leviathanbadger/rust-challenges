use std::fmt::Display;
use crate::calculator::{
    interpreter::{MethodBuilder, Op},
    tokenizer::Token
};
use super::{
    expression_syntax::{ExpressionPrecedence, ExpressionSyntax},
    syntax::Syntax
};

#[derive(Debug)]
pub enum PrimaryExpressionKind {
    Literal
}

pub struct PrimaryExpressionSyntax {
    kind: PrimaryExpressionKind,
    literal_token: Option<Token>
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
                literal_token: Some(token.clone())
            }))
        }

        if token.is_operator("(") {
            let mut npos = *pos + 1;
            let nested_expr_opt = super::expression_syntax::try_parse_expression(tokens, &mut npos);
            if let Some(nested_expr) = nested_expr_opt {
                if npos < tokens.len() && tokens[npos].is_operator(")") {
                    *pos = npos + 1;
                    return Some(nested_expr)
                }
            }
        }

        None
    }
}

impl ExpressionSyntax for PrimaryExpressionSyntax {
    fn get_expression_precedence(&self) -> ExpressionPrecedence {
        ExpressionPrecedence::Primary
    }

    fn emit_bytecode(&self, method_builder: &mut MethodBuilder) -> anyhow::Result<()> {
        match self.kind {
            PrimaryExpressionKind::Literal => {
                let val = f64::try_from(self.literal_token.as_ref().unwrap())?;
                method_builder.ops.push(Op::LdcF8(val));
            }
        }

        Ok(())
    }
}

impl Syntax for PrimaryExpressionSyntax { }

impl Display for PrimaryExpressionSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            PrimaryExpressionKind::Literal => {
                self.literal_token.as_ref().unwrap().repr(f)?;
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::calculator::{
        syntax::try_parse_expression,
        tokenizer::Tokenizer
    };
    use anyhow::*;
    use super::*;

    #[test]
    fn emit_bytecode_should_emit_correct_bytecode() -> Result<()> {
        let test_cases: &[(&str, &[Op])] = &[
            ("25", &[Op::LdcF8(25.0)]),
            ("123.456", &[Op::LdcF8(123.456)]),
            ("(42)", &[Op::LdcF8(42.0)]),
        ];

        let tokenizer = Tokenizer::new();

        for &(input, expected_ops) in test_cases {
            let tokens = tokenizer.tokenize(input).collect();

            let mut pos = 0;
            let expr = try_parse_expression(&tokens, &mut pos);

            assert!(expr.is_some());
            assert_eq!(pos, tokens.len() - 1);

            let mut method_builder = MethodBuilder::new();
            expr.unwrap().emit_bytecode(&mut method_builder)?;

            assert_eq!(&method_builder.ops[..], expected_ops);
        }

        Ok(())
    }
}
