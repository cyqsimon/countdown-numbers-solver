use itertools::Itertools;
use strum::EnumIter;

use crate::compute::PostfixSequence;

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

/// Convert a postfix sequence to a postfix string suitable for
/// display printing.
pub fn postfix_print(seq: &PostfixSequence) -> String {
    seq.iter()
        .map(|token| match token {
            Token::Num(n) => n.to_string(),
            Token::Op(Op::Add) => "+".into(),
            Token::Op(Op::Sub) => "-".into(),
            Token::Op(Op::Mul) => "*".into(),
            Token::Op(Op::Div) => "/".into(),
        })
        .join(",")
}
