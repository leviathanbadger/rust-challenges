use anyhow::{anyhow, Result};
use crate::calculator::{
    tokenizer::{Tokenizer, Token},
    syntax::try_parse_expression
};

pub struct Calculator {
    tokenizer: Tokenizer
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            tokenizer: Tokenizer::new()
        }
    }

    #[allow(unused)]
    pub fn eval<T: AsRef<str>>(&self, str: T) -> Result<f64> {
        let str = str.as_ref();
        let tokens = self.tokenizer.tokenize(str)
            .collect::<Vec<Token>>();

        for token in tokens.iter() {
            println!("{}", token);
        }

        let mut pos = 0;
        let expr_opt = try_parse_expression(&tokens, &mut pos);
        if let Some(expr) = expr_opt {
            println!("Parsed expression: {}", expr);
        }
        else {
            return Err(anyhow!("Failed to parse expression."));
        }

        if pos != tokens.len() - 1 {
            return Err(anyhow!("Unexpected token: {}.", tokens[pos]));
        }

        Ok(0.0)
    }
}
