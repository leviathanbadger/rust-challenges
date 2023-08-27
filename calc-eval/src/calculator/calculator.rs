use anyhow::Result;

pub struct Calculator {
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { }
    }

    #[allow(unused)]
    pub fn eval<T: AsRef<str>>(&self, str: T) -> Result<f64> {
        let str = str.as_ref();
        todo!("Not implemented!");
    }
}
