// https://adventofcode.com/2022/day/1
// (part1) cargo run --bin day1
// (part2) cargo run --bin day1 3

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let top: usize = args
        .get(1)
        .map_or(1, |x| x.parse::<usize>().expect("top X"));

    let path = "src/bin/day1/input.txt";
    let contents = fs::read_to_string(path).expect(&format!("Failed to read input from {path}"));

    let input_data: Vec<Vec<u64>> = contents
        .split("\n\n")
        .map(|x| x.split("\n").map(|s| s.parse::<u64>().unwrap()).collect())
        .collect();

    // println!("input_data:\n{:?}", input_data);

    let mut totals: Vec<u64> = input_data.iter().map(|xs| xs.iter().sum()).collect();

    // sort in reverse
    totals.sort_by(|x, y| y.cmp(&x));

    let top_total = totals.iter().take(top).sum::<u64>();

    println!("top {top} carries a total of {top_total} calories!");

}
