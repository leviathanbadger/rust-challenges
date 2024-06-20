

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    LdcF8(f64),
    Neg,
    Mul,
    Div,
    Rem,
    Add,
    Sub
}
