use anyhow::*;
use super::MethodBuilder;

pub struct Interpreter { }

impl Interpreter {
    pub fn new() -> Self {
        Self { }
    }

    pub fn evaluate_method(&self, method: &MethodBuilder) -> Result<f64> {
        let mut stack = vec![];

        for &op in method.ops.iter() {
            match op {
                super::Op::LdcF8(num) => {
                    stack.push(num);
                },
                super::Op::Neg => {
                    let val = -stack.pop().ok_or(anyhow!("Stack underflow"))?;
                    stack.push(val);
                },
                super::Op::Mul => {
                    let val1 = stack.pop().ok_or(anyhow!("Stack underflow"))?;
                    let val2 = stack.pop().ok_or(anyhow!("Stack underflow"))?;
                    stack.push(val2 * val1);
                },
                super::Op::Div => {
                    let val1 = stack.pop().ok_or(anyhow!("Stack underflow"))?;
                    let val2 = stack.pop().ok_or(anyhow!("Stack underflow"))?;
                    stack.push(val2 / val1);
                },
                super::Op::Rem => {
                    let val1 = stack.pop().ok_or(anyhow!("Stack underflow"))?;
                    let val2 = stack.pop().ok_or(anyhow!("Stack underflow"))?;
                    stack.push(val2 % val1);
                },
                super::Op::Add => {
                    let val1 = stack.pop().ok_or(anyhow!("Stack underflow"))?;
                    let val2 = stack.pop().ok_or(anyhow!("Stack underflow"))?;
                    stack.push(val2 + val1);
                },
                super::Op::Sub => {
                    let val1 = stack.pop().ok_or(anyhow!("Stack underflow"))?;
                    let val2 = stack.pop().ok_or(anyhow!("Stack underflow"))?;
                    stack.push(val2 - val1);
                }
            }
        }

        let retval = stack.pop().ok_or(anyhow!("Stack underflow"))?;
        if stack.len() != 0 {
            Err(anyhow!("Somehow the stack had multiple values before returning"))
        }
        else {
            Ok(retval)
        }
    }
}
