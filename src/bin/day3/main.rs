// https://adventofcode.com/2022/day/3
// (part1) cargo run --bin day3
use std::collections::HashSet;

fn score(c: char) -> u32 {
    if c >= 'a' {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

fn intersection(s1: &HashSet<char>, s2: &HashSet<char>) -> HashSet<char> {
    // there has got to be a better way to do this
    let mut result = HashSet::new();
    for &c in s1.intersection(&s2) {
        result.insert(c);
    }
    result
}

fn priority(line: &str) -> u32 {
    let mid = line.len() / 2;
    let first: HashSet<char> = HashSet::from_iter(line[..mid].chars());
    let second: HashSet<char> = HashSet::from_iter(line[mid..].chars());
    let &c = first.intersection(&second).next().unwrap();
    score(c)
}

fn part1(input: &str) {
    let answer: u32 = input.lines().map(|line| priority(line)).sum();
    println!("part1: {answer}");
}

fn part2(input: &str) {
    let lines: Vec<&str> = input.lines().collect();
    let mut answer = 0;
    // TODO: figure out how to use feature to do chunk
    for x in (0..(lines.len())).step_by(3) {
        let common_items = lines[x..(x + 2)]
            .iter()
            .map(|&s| HashSet::from_iter(s.chars()))
            .reduce(|s1, s2| intersection(&s1, &s2))
            .unwrap();
        let common = common_items.iter().next().unwrap();
        answer += score(*common);
    }
    println!("part2: {answer}");
}

fn main() {
    let input = include_str!("input.txt");
    part1(input);
    part2(input);
}
