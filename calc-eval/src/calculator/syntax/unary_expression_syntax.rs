use std::fmt::Display;
use crate::calculator::{
    interpreter::{MethodBuilder, Op},
    tokenizer::Token
};
use super::{
    expression_syntax::{ExpressionPrecedence, ExpressionSyntax},
    primary_expression_syntax::PrimaryExpressionSyntax,
    syntax::Syntax
};

#[derive(Debug)]
pub enum UnaryExpressionKind {
    Plus,
    Minus
}

pub struct UnaryExpressionSyntax {
    kind: UnaryExpressionKind,
    nested_expr: Box<dyn ExpressionSyntax>
}

impl UnaryExpressionSyntax {
    pub fn try_parse_expression(tokens: &Vec<Token>, pos: &mut usize) -> Option<Box<dyn ExpressionSyntax>> {
        if *pos >= tokens.len() {
            return None;
        }

        let token = &tokens[*pos];
        let mut kind_opt = None;
        if token.is_operator("+") {
            kind_opt = Some(UnaryExpressionKind::Plus);
        }
        else if token.is_operator("-") {
            kind_opt = Some(UnaryExpressionKind::Minus);
        }

        if let Some(kind) = kind_opt {
            let mut npos = *pos + 1;
            let nested_expr_opt = UnaryExpressionSyntax::try_parse_expression(tokens, &mut npos);
            if let Some(nested_expr) = nested_expr_opt {
                *pos = npos;

                Some(Box::new(UnaryExpressionSyntax {
                    kind,
                    nested_expr
                }))
            }
            else {
                None
            }
        }
        else {
            PrimaryExpressionSyntax::try_parse_expression(tokens, pos)
        }
    }
}

impl ExpressionSyntax for UnaryExpressionSyntax {
    fn get_expression_precedence(&self) -> ExpressionPrecedence {
        ExpressionPrecedence::Unary
    }

    fn emit_bytecode(&self, method_builder: &mut MethodBuilder) -> anyhow::Result<()> {
        match self.kind {
            UnaryExpressionKind::Minus => {
                method_builder.ops.push(Op::Neg);
            },
            UnaryExpressionKind::Plus => { }
        }

        self.nested_expr.emit_bytecode(method_builder)?;

        Ok(())
    }
}

impl Syntax for UnaryExpressionSyntax { }

impl Display for UnaryExpressionSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            UnaryExpressionKind::Plus => {
                write!(f, "+")?;
            },
            UnaryExpressionKind::Minus => {
                write!(f, "-")?;
            }
        };

        let needs_parenthesis = self.nested_expr.get_expression_precedence() > self.get_expression_precedence();
        if needs_parenthesis {
            write!(f, "({})", self.nested_expr)?;
        }
        else {
            write!(f, "{}", self.nested_expr)?;
        }

        Ok(())
    }
}
