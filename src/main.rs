mod compute;
mod post_processing;
mod util;

use clap::Parser;
use itertools::Itertools;

use crate::{
    compute::{calc_postfix_sequences_all, calc_postfix_sequences_first},
    util::ExpBTree,
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
                    .map(|seq| ExpBTree::try_from(seq).unwrap()) // calculated sequence is always valid
                    .dedup_by(|t0, t1| t0.commutative_eq(t1))
                    .map(|tree| {
                        if postfix {
                            tree.to_postfix_string()
                        } else {
                            tree.to_infix_string()
                        }
                    })
                    .sorted() // stable order
                    .for_each(|repr| println!(" - {}", repr));
            }
        };
    } else {
        let solution = calc_postfix_sequences_first(&numbers, target, dumb);
        match solution {
            Some(seq) => {
                let tree = ExpBTree::try_from(seq).unwrap(); // calculated sequence is always valid
                let repr = if postfix {
                    tree.to_postfix_string()
                } else {
                    tree.to_infix_string()
                };
                println!("Solution found: {}", repr);
            }
            None => println!("No solution found"),
        };
    }
}
