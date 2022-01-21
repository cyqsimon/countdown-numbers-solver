mod compute;

use clap::Parser;

use crate::compute::{calc_postfix_sequences_all, calc_postfix_sequences_first};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct CliArgs {
    /// Don't stop until all possible solutions are found
    #[clap(short = 'a', long = "find-all")]
    find_all: bool,

    /// Include trivially-different solutions (e.g. *1, /1, a+b vs. b+a, etc.)
    #[clap(short = 'd', long = "dumb")]
    dumb: bool,

    /// The list of numbers to work with, delimited by commas
    #[clap(required = true, value_delimiter = ',', require_delimiter = true)]
    numbers: Vec<u32>,

    /// The target number
    target: u32,
}

fn main() {
    let CliArgs {
        find_all,
        dumb,
        numbers,
        target,
    } = CliArgs::parse();
    println!("  Numbers: {:?}, Target: {}", numbers, target);
    println!("  Find all: {}, Dumb: {}", find_all, dumb);

    if find_all {
        let solutions = calc_postfix_sequences_all(&numbers, target, dumb);
        match solutions.len() {
            0 => println!("No solution found"),
            n => {
                println!("{} solutions found", n);
                solutions
                    .into_iter()
                    .for_each(|solution| println!("  - {:?}", solution));
            }
        };
    } else {
        let solution = calc_postfix_sequences_first(&numbers, target, dumb);
        match solution {
            Some(solution) => println!("Solution found: {:?}", solution),
            None => println!("No solution found"),
        };
    }
}
