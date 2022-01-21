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

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
        };
        write!(f, "{}", repr)
    }
}

/// An atomic unit in a postfix-order expression.
///
/// Either a positive integer or an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Token {
    Num(u32),
    Op(Op),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Token::Num(n) => n.to_string(),
            Token::Op(op) => op.to_string(),
        };
        write!(f, "{}", repr)
    }
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

/// Convert a postfix sequence to a postfix string suitable for display printing.
pub fn postfix_print(seq: &PostfixSequence) -> String {
    seq.iter().map(Token::to_string).join(",")
}

/// Convert a postfix sequence to a infix string suitable for display printing.
///
/// Returns None if the sequence does not produce a valid expression.
pub fn infix_print(seq: &PostfixSequence) -> Option<String> {
    let mut stack = vec![];
    for token in seq.iter() {
        match token {
            Token::Num(n) => stack.push(n.to_string()),
            Token::Op(op) => {
                let n1 = stack.pop()?;
                let n0 = stack.pop()?;
                let repr = format!("({}{}{})", n0, op, n1);
                stack.push(repr);
            }
        };
    }
    if stack.len() != 1 {
        return None;
    }

    stack.pop()
}
