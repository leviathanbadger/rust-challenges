use super::op::Op;

pub struct MethodBuilder {
    pub ops: Vec<Op>
}

impl MethodBuilder {
    pub fn new() -> Self {
        Self {
            ops: vec![]
        }
    }
}
