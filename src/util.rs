//! This module contains miscellaneous utility structs and functions.

use std::{cmp::Ordering, fmt};

use itertools::Itertools;
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
impl PartialOrd for Op {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
/// Operational precedence rules.
impl Ord for Op {
    fn cmp(&self, other: &Self) -> Ordering {
        use Op::*;
        use Ordering::*;
        match (self, other) {
            (Add | Sub, Add | Sub) | (Mul | Div, Mul | Div) => Equal,
            (Add | Sub, Mul | Div) => Less,
            (Mul | Div, Add | Sub) => Greater,
        }
    }
}

/// A sequence of tokens representing a postfix expression.
pub type PostfixSequence = Vec<Token>;

/// Convert a postfix sequence to a postfix string suitable for display printing.
pub fn to_postfix_string(seq: &PostfixSequence) -> String {
    seq.iter().map(Token::to_string).join(",")
}

#[derive(Debug)]
pub struct InvalidPostfixSequenceError {
    seq_repr: String,
}
impl fmt::Display for InvalidPostfixSequenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.seq_repr)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpBTree {
    Num(u32),
    Exp {
        lhs: Box<ExpBTree>,
        rhs: Box<ExpBTree>,
        op: Op,
    },
}
impl From<u32> for ExpBTree {
    fn from(n: u32) -> Self {
        ExpBTree::Num(n)
    }
}
impl TryFrom<PostfixSequence> for ExpBTree {
    type Error = InvalidPostfixSequenceError;

    fn try_from(seq: PostfixSequence) -> Result<Self, Self::Error> {
        fn try_from_impl(seq: PostfixSequence) -> Option<ExpBTree> {
            let mut stack = vec![];
            for token in seq.iter() {
                match token {
                    &Token::Num(n) => stack.push(n.into()),
                    &Token::Op(op) => {
                        let rhs = Box::new(stack.pop()?);
                        let lhs = Box::new(stack.pop()?);
                        let exp = ExpBTree::Exp { lhs, rhs, op };
                        stack.push(exp);
                    }
                }
            }

            // the sequence is only valid if there is exactly one value
            // on the stack at this point
            if stack.len() == 1 {
                stack.pop()
            } else {
                None
            }
        }

        let seq_repr = to_postfix_string(&seq);
        try_from_impl(seq).ok_or_else(|| InvalidPostfixSequenceError { seq_repr })
    }
}
impl ExpBTree {
    pub fn commutative_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&ExpBTree::Num(s), &ExpBTree::Num(o)) => s == o,
            (
                ExpBTree::Exp {
                    lhs: s_lhs,
                    rhs: s_rhs,
                    op: s_op,
                },
                ExpBTree::Exp {
                    lhs: o_lhs,
                    rhs: o_rhs,
                    op: o_op,
                },
            ) => {
                if s_op != o_op {
                    return false;
                }
                match s_op {
                    Op::Add | Op::Mul => {
                        s_lhs.commutative_eq(o_lhs) && s_rhs.commutative_eq(o_rhs)
                            || s_lhs.commutative_eq(o_rhs) && s_rhs.commutative_eq(o_lhs)
                    }
                    Op::Sub | Op::Div => s_lhs.commutative_eq(o_lhs) && s_rhs.commutative_eq(o_rhs),
                }
            }
            _ => false,
        }
    }
    pub fn to_postfix_string(&self) -> String {
        match self {
            ExpBTree::Num(n) => n.to_string(),
            ExpBTree::Exp { lhs, rhs, op } => format!(
                "{},{},{}",
                lhs.to_postfix_string(),
                rhs.to_postfix_string(),
                op
            ),
        }
    }
    pub fn to_infix_string(&self) -> String {
        self.to_infix_string_impl().0
    }
    fn to_infix_string_impl(&self) -> (String, Option<Op>) {
        match self {
            &ExpBTree::Num(n) => (n.to_string(), None),
            ExpBTree::Exp { lhs, rhs, op } => {
                let (lhs_repr_raw, lhs_op) = lhs.to_infix_string_impl();
                let lhs_repr = match lhs_op {
                    None => lhs_repr_raw,
                    Some(lhs_op) => match lhs_op.cmp(op) {
                        Ordering::Less => format!("({})", lhs_repr_raw),
                        Ordering::Equal | Ordering::Greater => lhs_repr_raw,
                    },
                };
                let (rhs_repr_raw, rhs_op) = rhs.to_infix_string_impl();
                let rhs_repr = match rhs_op {
                    None => rhs_repr_raw,
                    Some(rhs_op) => match rhs_op.cmp(op) {
                        Ordering::Less | Ordering::Equal => format!("({})", rhs_repr_raw),
                        Ordering::Greater => rhs_repr_raw,
                    },
                };
                let repr = format!("{}{}{}", lhs_repr, op, rhs_repr);
                (repr, Some(*op))
            }
        }
    }
}
