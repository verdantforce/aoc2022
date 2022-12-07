// https://adventofcode.com/2022/day/3
// cargo run --bin day6

use std::collections::HashSet;

fn all_distinct(chs: &[char]) -> bool {
    let set: HashSet<&char> = HashSet::from_iter(chs.iter());
    set.len() == chs.len()
}

fn find_distinct_block(input: &str, n: usize) -> usize {
    assert!(input.len() >= n);
    let chs: Vec<char> = input.chars().collect();
    for start in n..chs.len() {
        if all_distinct(&chs[start-n..start]) {
            return start;
        }
    }
    return 0;
}

fn part1(input: &str) {
    let answer = find_distinct_block(input, 4);
    println!("part1: {}", answer);
}

fn part2(input: &str) {
    let answer = find_distinct_block(input, 14);
    println!("part2: {}", answer);
}

fn main() {
    let input = include_str!("input.txt");
    part1(input);
    part2(input);
}
