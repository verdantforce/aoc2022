// https://adventofcode.com/2022/day/3
// (part1) cargo run --bin day4
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Range {
    start: u32,
    end: u32
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        !(other.start > self.end || other.end < self.start)
    }
}

#[derive(Debug, Clone)]
struct RangeParseError;

impl FromStr for Range {
    type Err = RangeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<u32> = s.split('-').map(|n| n.parse::<u32>().unwrap()).collect();
        Ok(Range {
            start: *v.get(0).ok_or(RangeParseError)?,
            end: *v.get(1).ok_or(RangeParseError)?,
        })
    }
}

fn part1(input: &str) {
    let answer: u32 = input.lines().map(
        |line| {
            let v: Vec<Range> = line.split(',').map(|s| Range::from_str(s).unwrap()).collect();
            if v[0].contains(&v[1]) || v[1].contains(&v[0]) {
                1
            } else {
                0
            }
        }
    ).sum();
    println!("part1: {answer}");
}

fn part2(input: &str) {
    let answer: u32 = input.lines().map(
        |line| {
            let v: Vec<Range> = line.split(',').map(|s| Range::from_str(s).unwrap()).collect();
            if v[0].overlaps(&v[1]) {
                1
            } else {
                0
            }
        }
    ).sum();
    println!("part2: {answer}");
}

fn main() {
    let input = include_str!("input.txt");
    part1(input);
    part2(input);
}
