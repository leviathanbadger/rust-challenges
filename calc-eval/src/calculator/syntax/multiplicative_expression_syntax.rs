use std::fmt::Display;
use crate::calculator::{
    interpreter::{MethodBuilder, Op},
    tokenizer::Token
};
use super::{
    expression_syntax::{ExpressionPrecedence, ExpressionSyntax},
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

impl ExpressionSyntax for MultiplicativeExpressionSyntax {
    fn get_expression_precedence(&self) -> ExpressionPrecedence {
        ExpressionPrecedence::Multiplicative
    }

    fn emit_bytecode(&self, method_builder: &mut MethodBuilder) -> anyhow::Result<()> {
        self.left_expr.emit_bytecode(method_builder)?;
        self.right_expr.emit_bytecode(method_builder)?;

        match self.kind {
            MultiplicativeExpressionKind::Multiply => {
                method_builder.ops.push(Op::Mul);
            },
            MultiplicativeExpressionKind::Divide => {
                method_builder.ops.push(Op::Div);
            },
            MultiplicativeExpressionKind::Modulus => {
                method_builder.ops.push(Op::Rem);
            }
        }

        Ok(())
    }
}

impl Syntax for MultiplicativeExpressionSyntax { }

impl Display for MultiplicativeExpressionSyntax {
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
            MultiplicativeExpressionKind::Multiply => {
                write!(f, " * ")?;
            },
            MultiplicativeExpressionKind::Divide => {
                write!(f, " / ")?;
            },
            MultiplicativeExpressionKind::Modulus => {
                write!(f, " % ")?;
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
            ("1*2", &[Op::LdcF8(1.0), Op::LdcF8(2.0), Op::Mul]),
            ("3/4", &[Op::LdcF8(3.0), Op::LdcF8(4.0), Op::Div]),
            ("5%6", &[Op::LdcF8(5.0), Op::LdcF8(6.0), Op::Rem]),
            ("1/2/3", &[Op::LdcF8(1.0), Op::LdcF8(2.0), Op::Div, Op::LdcF8(3.0), Op::Div]),
            ("1/(2/3)", &[Op::LdcF8(1.0), Op::LdcF8(2.0), Op::LdcF8(3.0), Op::Div, Op::Div]),
            ("-5*-3", &[Op::LdcF8(5.0), Op::Neg, Op::LdcF8(3.0), Op::Neg, Op::Mul]),
            ("-(4*6)", &[Op::LdcF8(4.0), Op::LdcF8(6.0), Op::Mul, Op::Neg]),
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
