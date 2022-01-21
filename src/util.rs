use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

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
