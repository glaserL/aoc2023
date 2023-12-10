mod aoc;
mod day01;
use std::time::Instant;

use aoc::get_puzzle;
use day01::{solve1, solve2};

fn timed<F>(function: F, input: String)
where
    F: Fn(String) -> u32,
{
    let start_time = Instant::now();

    let solution = function(input);

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    println!(
        "Solution: {:?} - Time elapsed: {:?}",
        solution, elapsed_time
    );
}

fn main() {
    let puzzle = get_puzzle(1);
    timed(solve1, puzzle.clone());
    timed(solve2, puzzle.clone());
}
