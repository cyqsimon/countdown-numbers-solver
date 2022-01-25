//! This module contains miscellaneous utility structs and functions.

use std::{cmp::Ordering, fmt};

use strum::EnumIter;

/// A single operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
impl fmt::Display for Op {
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
impl fmt::Display for Token {
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

/// Operational precedence of an expression.
///
/// Helps determine whether an expression needs parentheses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpPrecedence {
    Add,
    Sub,
    Mul,
    Div,
    Number,
}
impl PartialOrd for ExpPrecedence {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ExpPrecedence {
    fn cmp(&self, other: &Self) -> Ordering {
        use ExpPrecedence::*;
        use Ordering::*;
        match (self, other) {
            (Add | Sub, Add | Sub) | (Mul | Div, Mul | Div) | (Number, Number) => Equal,
            (Add | Sub, Mul | Div) | (_, Number) => Less,
            (Mul | Div, Add | Sub) | (Number, _) => Greater,
        }
    }
}
impl From<Op> for ExpPrecedence {
    fn from(op: Op) -> Self {
        match op {
            Op::Add => ExpPrecedence::Add,
            Op::Sub => ExpPrecedence::Sub,
            Op::Mul => ExpPrecedence::Mul,
            Op::Div => ExpPrecedence::Div,
        }
    }
}
