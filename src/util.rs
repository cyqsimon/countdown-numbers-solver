use strum::EnumIter;

/// A single operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

/// An atomic unit in a postfix-order expression.
///
/// Either a positive integer or an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Token {
    Num(u32),
    Op(Op),
}

impl From<u32> for Token {
    fn from(n: u32) -> Self {
        Token::Num(n)
    }
}
impl From<Op> for Token {
    fn from(op: Op) -> Self {
        Token::Op(op)
    }
}
