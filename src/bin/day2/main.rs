// https://adventofcode.com/2022/day/2
// cargo run --bin day2

// Notes
// 'A' -> Rock, 'B' -> Paper, 'C' -> Scissors
// 'X' -> Rock, 'Y' -> Paper, 'Z' -> Scissors
//  Rock -> 1, Paper -> 2, Scissors -> 3

struct Score {
    me: i32,
    opp: i32,
    result: i32,
    score: i32,
}

fn score_map() -> Vec<Score> {
    let mut scores = Vec::new();
    for me in 0..3 {
        for opp in 0..3 {
            let choose_score = me + 1;
            let win_score = if me == (opp + 1) % 3 {
                6
            } else if me == opp {
                3
            } else {
                0
            };
            let result = (win_score - 3) / 3;
            let total_score = choose_score + win_score;
            let score = Score {
                me,
                opp,
                result,
                score: total_score,
            };
            scores.push(score);
        }
    }

    scores
}

fn score1(me: i32, opp: i32, score_map: &Vec<Score>) -> i32 {
    score_map
        .iter()
        .find(|&s| s.me == me && s.opp == opp)
        .map(|s| s.score)
        .unwrap()
}

fn score2(result: i32, opp: i32, score_map: &Vec<Score>) -> i32 {
    score_map
        .iter()
        .find(|&s| s.opp == opp && s.result == result)
        .map(|s| s.score)
        .unwrap()
}

fn part1() {
    let contents = include_str!("input.txt");
    let score_map = score_map();
    let answer: i32 = contents
        .lines()
        .map(|line| {
            let chs: Vec<char> = line.chars().collect();
            let opp = chs[0] as i32 - 'A' as i32;
            let me = chs[2] as i32 - 'X' as i32;
            score1(me, opp, &score_map)
        })
        .sum();
    println!("part1: {answer}");
}

fn part2() {
    let contents = include_str!("input.txt");
    let score_map = score_map();
    let answer: i32 = contents
        .lines()
        .map(|line| {
            let chs: Vec<char> = line.chars().collect();
            let opp = chs[0] as i32 - 'A' as i32;
            let result = chs[2] as i32 - 'Y' as i32;
            score2(result, opp, &score_map)
        })
        .sum();
    println!("part2: {answer}");
}

fn main() {
    part1();
    part2();
}
