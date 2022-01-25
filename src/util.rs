//! This module contains miscellaneous utility structs and functions.

use std::{cmp::Ordering, fmt};

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
enum ExpPrecedence {
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

/// Convert a postfix sequence to a postfix string suitable for display printing.
pub fn postfix_print(seq: &PostfixSequence) -> String {
    seq.iter().map(Token::to_string).join(",")
}

/// Convert a postfix sequence to an infix string suitable for display printing.
///
/// Returns None if the sequence does not produce a valid expression.
pub fn infix_print(seq: &PostfixSequence) -> Option<String> {
    let mut stack = vec![];
    for token in seq.iter() {
        match token {
            Token::Num(n) => stack.push((n.to_string(), ExpPrecedence::Number)),
            Token::Op(op) => {
                let (exp_r, prd_r) = stack.pop()?;
                let (exp_l, prd_l) = stack.pop()?;
                let prd_op = (*op).into();

                // if expression precedence < current operation precedence
                // then format with parentheses
                let repr_l = match prd_l.cmp(&prd_op) {
                    Ordering::Less => format!("({})", exp_l),
                    Ordering::Equal | Ordering::Greater => exp_l,
                };
                let repr_r = match prd_r.cmp(&prd_op) {
                    Ordering::Less => format!("({})", exp_r),
                    Ordering::Equal => {
                        // sub & div do not have associative property and need parentheses on RHS
                        // e.g. 64/(4/2) != 64/4/2
                        match prd_r {
                            ExpPrecedence::Sub | ExpPrecedence::Div => format!("({})", exp_r),
                            _ => exp_r,
                        }
                    }
                    Ordering::Greater => exp_r,
                };
                let repr_exp = format!("{}{}{}", repr_l, op, repr_r);

                // new expression precedence is the current operation precedence
                stack.push((repr_exp, prd_op));
            }
        };
    }

    // the sequence is only valid if there is exactly one value
    // on the stack at this point
    if stack.len() == 1 {
        stack.pop().map(|(exp, _)| exp)
    } else {
        None
    }
}
