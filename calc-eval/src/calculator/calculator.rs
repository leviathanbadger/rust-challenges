use anyhow::Result;
use super::tokenizer::Tokenizer;

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
        let tokens = self.tokenizer.tokenize(str);

        for token in tokens {
            println!("{}", token);
        }

        todo!("Not implemented!");
    }
}
