// https://adventofcode.com/2022/day/3
// cargo run --bin day5

use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt, str::FromStr};

struct Stacks {
    stacks: Vec<Vec<char>>,
}

#[derive(Debug, Clone)]
struct Move {
    src: usize,
    dst: usize,
    n: usize,
}

trait Mover {
    fn exec(s: &mut Stacks, m: &Move);
}

enum CrateMover9000 {}
enum CrateMover9001 {}

impl Mover for CrateMover9000 {
    fn exec(s: &mut Stacks, m: &Move) {
        // println!("Executing move {:?}", m);
        // println!("Before:\n{s}");
        for _ in 0..m.n {
            let c = s.stacks[m.src - 1].pop().unwrap();
            s.stacks[m.dst - 1].push(c);
        }
        // println!("After:\n{s}");
    }
}

impl Mover for CrateMover9001 {
    fn exec(s: &mut Stacks, m: &Move) {
        let i = s.stacks[m.src - 1].len();
        // println!("Executing move {:?}", m);
        // println!("Before:\n{s}");
        let t: Vec<_> = s.stacks[m.src - 1].drain((i - m.n)..).collect();
        for c in t.iter() {
            s.stacks[m.dst - 1].push(*c);
        }
        // println!("After:\n{s}");
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        let src = caps.get(2).unwrap().as_str().parse().unwrap();
        let dst = caps.get(3).unwrap().as_str().parse().unwrap();
        let n = caps.get(1).unwrap().as_str().parse().unwrap();
        Ok(Move { src, dst, n })
    }
}

impl fmt::Display for Stacks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Stacks")?;
        writeln!(f, "------")?;
        for (i, v) in self.stacks.iter().enumerate() {
            let column: String = v.iter().collect();
            writeln!(f, "{}: {}", i + 1, column)?;
        }
        Ok(())
    }
}

fn read_stacks(input: &str) -> Stacks {
    let mut lines = input.lines().rev();
    let num_stacks = (lines.next().unwrap().len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(num_stacks);
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }
    for line in lines {
        let chs: Vec<char> = line.chars().collect();
        for i in 0..num_stacks {
            let position = 1 + i * 4;
            if chs[position] != ' ' {
                stacks[i].push(chs[position]);
            }
        }
    }
    Stacks { stacks }
}

fn read_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| Move::from_str(line).unwrap())
        .collect()
}

fn read_input(input: &str) -> (Stacks, Vec<Move>) {
    let mut splits = input.split("\n\n");
    (
        read_stacks(splits.next().unwrap()),
        read_moves(splits.next().unwrap()),
    )
}

fn part1(input: &str) {
    let (mut s, moves) = read_input(input);
    moves.iter().for_each(|m| CrateMover9000::exec(&mut s, m));
    let answer: String = s.stacks.iter().map(|v| v.last().unwrap()).collect();
    println!("part1: {answer}");
}

fn part2(input: &str) {
    let (mut s, moves) = read_input(input);
    moves.iter().for_each(|m| CrateMover9001::exec(&mut s, m));
    let answer: String = s.stacks.iter().map(|v| v.last().unwrap()).collect();
    println!("part2: {answer}");
}

fn main() {
    let input = include_str!("input.txt");
    part1(input);
    part2(input);
}
