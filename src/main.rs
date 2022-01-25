mod compute;
mod util;

use clap::Parser;
use itertools::Itertools;

use crate::{
    compute::{calc_postfix_sequences_all, calc_postfix_sequences_first},
    util::{infix_print, postfix_print},
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct CliArgs {
    /// Don't stop until all possible solutions are found
    #[clap(short = 'a', long = "find-all")]
    find_all: bool,

    /// Include trivially-different solutions (e.g. *1, /1, a+b vs. b+a, etc.)
    #[clap(short = 'd', long = "dumb")]
    dumb: bool,

    /// Output the solution using postfix notation
    #[clap(short = 'p', long = "postfix")]
    postfix: bool,

    /// The list of numbers to work with, delimited by commas
    #[clap(required = true, value_delimiter = ',', multiple_occurrences = false)]
    numbers: Vec<u32>,

    /// The target number
    target: u32,
}

fn main() {
    let CliArgs {
        find_all,
        dumb,
        postfix,
        numbers,
        target,
    } = CliArgs::parse();
    println!("Numbers: {:?}, Target: {}", numbers, target);
    println!(
        "  Find all: {}, Dumb: {}, Postfix: {}",
        find_all, dumb, postfix
    );

    if find_all {
        let solutions = calc_postfix_sequences_all(&numbers, target, dumb);
        match solutions.len() {
            0 => println!("No solution found"),
            n => {
                println!("{} solutions found", n);
                solutions
                    .into_iter()
                    .map(|solution| {
                        if postfix {
                            postfix_print(&solution)
                        } else {
                            infix_print(&solution).unwrap() // solution is always valid
                        }
                    })
                    .sorted()
                    .for_each(|repr| println!(" - {}", repr));
            }
        };
    } else {
        let solution = calc_postfix_sequences_first(&numbers, target, dumb);
        match solution {
            Some(solution) => println!(
                "Solution found: {}",
                if postfix {
                    postfix_print(&solution)
                } else {
                    infix_print(&solution).unwrap() // solution is always valid
                }
            ),
            None => println!("No solution found"),
        };
    }
}
