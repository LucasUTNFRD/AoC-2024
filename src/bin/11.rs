use std::collections::VecDeque;

use anyhow::{Error, Result};

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_11.txt");

fn apply_rules(stone: &u64) -> (u64, Option<u64>) {
    let stone_str = stone.to_string();
    let digit_count = stone_str.len();

    match stone {
        // Rule 1: If the stone is 0, replace with 1
        0 => (1, None),

        // Rule 2: If number has even digits, split into two stones
        _ if digit_count % 2 == 0 => {
            let mid = digit_count / 2;
            let left = stone_str[..mid].parse::<u64>().unwrap_or(0);
            let right = stone_str[mid..].parse::<u64>().unwrap_or(0);
            (left, Some(right))
        }

        // Rule 3: Otherwise multiply by 2024
        _ => (*stone * 2024, None),
    }
}

fn blink(stones: &VecDeque<u64>, time: usize) -> usize {
    let mut current_stones = stones.clone();

    for _ in 0..time {
        // let mut new_stones = VecDeque::new();
        while let Some(stone) = current_stones.pop_front() {
            let stone_with_rules = apply_rules(&stone);
            new_stones.push_back(stone_with_rules.0);
            if let Some(right) = stone_with_rules.1 {
                new_stones.push_back(right);
            }
        }
        current_stones = new_stones;
    }
    current_stones.len()
}

const TIMES: usize = 75;

// #[cfg(feature = "part_1")]
fn solve_part_1(input: &str) -> Result<String, Error> {
    let stones: VecDeque<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let count_blinks = blink(&stones, TIMES);

    Ok(count_blinks.to_string())
}

#[cfg(feature = "part_2")]
fn solve_part_2(input: &str) -> Result<String, Error> {
    let solution = input.lines().next().unwrap().replace("input", "answer");

    Ok(solution)
}

fn main() -> Result<(), Error> {
    println!("\nDay 11\n------");

    #[cfg(feature = "part_1")]
    {
        let answer_part_1 = solve_part_1(PUZZLE_INPUT)?;
        println!("Part One: {answer_part_1}");
    }

    #[cfg(feature = "part_2")]
    {
        let answer_part_2 = solve_part_2(PUZZLE_INPUT)?;
        println!("Part Two: {answer_part_2}");
    }

    println!();

    Ok(())
}

#[cfg(feature = "part_1")]
#[test]
fn sample_part_1() {
    const SAMPLE_INPUT_1: &str = "\
125 17
";
    const SAMPLE_ANSWER_1: &str = "55312";

    assert_eq!(solve_part_1(SAMPLE_INPUT_1).unwrap(), SAMPLE_ANSWER_1);
}

#[cfg(feature = "part_2")]
#[test]
fn sample_part_2() {
    const SAMPLE_INPUT_2: &str = "\
sample part 2 input
goes here
like this
";
    const SAMPLE_ANSWER_2: &str = "sample part 2 answer";

    assert_eq!(solve_part_2(SAMPLE_INPUT_2).unwrap(), SAMPLE_ANSWER_2);
}
