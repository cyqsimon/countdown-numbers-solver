//! This module contains code for post-processing calculated result.
//!
//! E.g. generating operation tree, formatting for print,
//! removing duplicates by commutative & associative properties, etc.

use std::cmp::Ordering;

use itertools::Itertools;

use crate::{
    compute::PostfixSequence,
    util::{ExpPrecedence, Token},
};

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
