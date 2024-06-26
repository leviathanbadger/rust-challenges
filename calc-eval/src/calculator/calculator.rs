use anyhow::{anyhow, Result};
use crate::calculator::{
    interpreter::{Interpreter, MethodBuilder},
    tokenizer::{Tokenizer, Token},
    syntax::try_parse_expression
};

pub struct Calculator {
    tokenizer: Tokenizer,
    interpreter: Interpreter
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            tokenizer: Tokenizer::new(),
            interpreter: Interpreter::new()
        }
    }

    #[allow(unused)]
    pub fn eval<T: AsRef<str>>(&self, str: T) -> Result<f64> {
        let str = str.as_ref();
        let tokens = self.tokenizer.tokenize(str)
            .collect::<Vec<Token>>();

        let mut pos = 0;
        let expr = try_parse_expression(&tokens, &mut pos)
            .ok_or(anyhow!("Failed to parse expression."))?;

        if pos != tokens.len() - 1 {
            return Err(anyhow!("Unexpected token: {}.", tokens[pos]));
        }

        let mut method_builder = MethodBuilder::new();
        expr.emit_bytecode(&mut method_builder)?;

        self.interpreter.evaluate_method(&method_builder)
    }
}
