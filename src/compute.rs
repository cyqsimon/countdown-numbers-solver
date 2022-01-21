use std::collections::HashSet;

use strum::IntoEnumIterator;

use crate::util::{Op, Token};

pub type PostfixSequence = Vec<Token>;

/// Try to apply a token on the postfix stack.
/// This will either push a number or apply an operation.
/// Returns the updated stack unless operation is illegal.
fn try_apply_legal(mut stack: Vec<u32>, token: Token) -> Option<Vec<u32>> {
    match token {
        Token::Num(n) => {
            stack.push(n);
            Some(stack)
        }
        Token::Op(_) if stack.len() < 2 => None,
        Token::Op(op) => {
            let operand1 = stack.pop().unwrap();
            let operand0 = stack.pop().unwrap(); // at least 2 in stack
            match op {
                Op::Add => {
                    stack.push(operand0 + operand1);
                    Some(stack)
                }
                Op::Sub => {
                    if operand0 <= operand1 {
                        None
                    } else {
                        stack.push(operand0 - operand1);
                        Some(stack)
                    }
                }
                Op::Mul => {
                    stack.push(operand0 * operand1);
                    Some(stack)
                }
                Op::Div => {
                    if operand1 == 0 || operand0 % operand1 != 0 {
                        None
                    } else {
                        stack.push(operand0 / operand1);
                        Some(stack)
                    }
                }
            }
        }
    }
}

/// Try to apply a token on the postfix stack.
/// This will either push a number or apply an operation.
/// Returns the updated stack only when the operation is sensible.
fn try_apply_sensible(mut stack: Vec<u32>, token: Token) -> Option<Vec<u32>> {
    match token {
        Token::Num(n) => {
            // 0 shouldn't be in the list but just in case
            if n == 0 {
                None
            } else {
                stack.push(n);
                Some(stack)
            }
        }
        Token::Op(_) if stack.len() < 2 => None,
        Token::Op(op) => {
            let operand1 = stack.pop().unwrap();
            let operand0 = stack.pop().unwrap(); // at least 2 in stack
            match op {
                Op::Add => {
                    // use ordering to eliminate some commutative duplications
                    if operand0 < operand1 {
                        None
                    } else {
                        stack.push(operand0 + operand1);
                        Some(stack)
                    }
                }
                Op::Sub => {
                    // getting 0 is not helpful
                    if operand0 < operand1 {
                        None
                    } else {
                        stack.push(operand0 - operand1);
                        Some(stack)
                    }
                }
                Op::Mul => {
                    // multiply by 1 is not helpful
                    // use ordering to eliminate some commutative duplications
                    if operand0 == 1 || operand1 == 1 || operand0 < operand1 {
                        None
                    } else {
                        stack.push(operand0 * operand1);
                        Some(stack)
                    }
                }
                Op::Div => {
                    // divide by 1 is not helpful
                    if operand1 <= 1 || operand0 % operand1 != 0 {
                        None
                    } else {
                        stack.push(operand0 / operand1);
                        Some(stack)
                    }
                }
            }
        }
    }
}

/// Find all solutions with the given parameters.
///
/// Optionally filter out trivially-different solutions
/// with the `dumb` flag.
pub fn calc_postfix_sequences_all(
    numbers: &[u32],
    target: u32,
    dumb: bool,
) -> HashSet<PostfixSequence> {
    calc_postfix_sequences_all_recurse(numbers, target, dumb, vec![], vec![])
}

/// Recursive implementation for `calc_postfix_sequences_all`.
fn calc_postfix_sequences_all_recurse(
    numbers: &[u32],
    target: u32,
    dumb: bool,
    stack: Vec<u32>,
    history: PostfixSequence,
) -> HashSet<PostfixSequence> {
    // if target reached, then add current history to output
    let mut self_outputs = HashSet::new();
    if stack.len() == 1 && stack[0] == target {
        self_outputs.insert(history.clone());
    }

    // for each available number, try to apply then recurse
    // collect all solutions found via recursion
    let number_step_outputs = numbers
        .iter()
        .enumerate()
        .filter_map(|(idx, &num)| {
            if dumb {
                try_apply_legal(stack.clone(), num.into())
            } else {
                try_apply_sensible(stack.clone(), num.into())
            }
            .map(|sub_stack| (idx, num, sub_stack))
        })
        .flat_map(|(idx, num, sub_stack)| {
            let mut sub_numbers = numbers.to_vec();
            sub_numbers.swap_remove(idx);

            let mut sub_history = history.clone();
            sub_history.push(num.into());

            calc_postfix_sequences_all_recurse(&sub_numbers, target, dumb, sub_stack, sub_history)
        })
        .collect();

    // for each operation, try to apply and recurse
    // collect all solutions found via recursion
    let operation_step_outputs = Op::iter()
        .filter_map(|op| {
            if dumb {
                try_apply_legal(stack.clone(), op.into())
            } else {
                try_apply_sensible(stack.clone(), op.into())
            }
            .map(|sub_stack| (op, sub_stack))
        })
        .flat_map(|(op, sub_stack)| {
            let mut sub_history = history.clone();
            sub_history.push(op.into());

            calc_postfix_sequences_all_recurse(numbers, target, dumb, sub_stack, sub_history)
        })
        .collect();

    // combine all three HashSets
    [self_outputs, number_step_outputs, operation_step_outputs]
        .into_iter()
        .reduce(|acc, set| acc.union(&set).cloned().collect())
        .unwrap() // None only when iterator is empty
}

/// Find a solution with the given parameters, short circuiting
/// as soon as the first solution is found.
///
/// Optionally filter out trivially-different solutions
/// with the `dumb` flag.
pub fn calc_postfix_sequences_first(
    numbers: &[u32],
    target: u32,
    dumb: bool,
) -> Option<PostfixSequence> {
    calc_postfix_sequences_first_recurse(numbers, target, dumb, vec![], vec![])
}

/// Recursive implementation for `calc_postfix_sequences_first`.
fn calc_postfix_sequences_first_recurse(
    numbers: &[u32],
    target: u32,
    dumb: bool,
    stack: Vec<u32>,
    history: PostfixSequence,
) -> Option<PostfixSequence> {
    // if target reached, return current history
    if stack.len() == 1 && stack[0] == target {
        return Some(history);
    }

    // for each available number, try to apply then recurse
    // return as soon as the first solution is found
    let number_step_solution = numbers
        .iter()
        .enumerate()
        .filter_map(|(idx, &num)| {
            if dumb {
                try_apply_legal(stack.clone(), num.into())
            } else {
                try_apply_sensible(stack.clone(), num.into())
            }
            .map(|sub_stack| (idx, num, sub_stack))
        })
        .find_map(|(idx, num, sub_stack)| {
            let mut sub_numbers = numbers.to_vec();
            sub_numbers.swap_remove(idx);

            let mut sub_history = history.clone();
            sub_history.push(num.into());

            calc_postfix_sequences_first_recurse(&sub_numbers, target, dumb, sub_stack, sub_history)
        });
    // short circuit if a solution is found, fall through otherwise
    if number_step_solution.is_some() {
        return number_step_solution;
    }

    // for each operation, try to apply and recurse
    // return as soon as the first solution is found
    let operation_step_solution = Op::iter()
        .filter_map(|op| {
            if dumb {
                try_apply_legal(stack.clone(), op.into())
            } else {
                try_apply_sensible(stack.clone(), op.into())
            }
            .map(|sub_stack| (op, sub_stack))
        })
        .find_map(|(op, sub_stack)| {
            let mut sub_history = history.clone();
            sub_history.push(op.into());

            calc_postfix_sequences_first_recurse(numbers, target, dumb, sub_stack, sub_history)
        });
    // return regardless of solution found or not
    // since there is nothing to fall through into
    operation_step_solution
}

// TODO: Associative filter
