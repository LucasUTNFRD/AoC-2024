use rustc_hash::FxHashMap;

use anyhow::{Error, Result};

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_11.txt");

fn count(stone: &u64, blinks: usize, cache: &mut FxHashMap<(u64, usize), u64>) -> u64 {
    if let Some(&count) = cache.get(&(*stone, blinks)) {
        return count;
    }
    if blinks == 0 {
        return 1;
    }
    let result = match stone {
        0 => count(&1, blinks - 1, cache),
        _ if stone.to_string().len() % 2 == 0 => {
            let stone_str = stone.to_string();
            let digit_count = stone_str.len();

            let mid = digit_count / 2;
            let left = stone_str[..mid].parse::<u64>().unwrap_or(0);
            let right = stone_str[mid..].parse::<u64>().unwrap_or(0);
            count(&left, blinks - 1, cache) + count(&right, blinks - 1, cache)
        }
        _ => count(&(stone * 2024), blinks - 1, cache),
    };

    cache.insert((*stone, blinks), result);

    result
}

const TIMES_PART_ONE: usize = 25;
const TIMES_PART_TWO: usize = 75;

#[cfg(feature = "part_1")]
fn solve_part_1(input: &str) -> Result<String, Error> {
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut cache: FxHashMap<(u64, usize), u64> = FxHashMap::default();
    let count_blinks: u64 = stones
        .iter()
        .map(|stone| count(stone, TIMES_PART_ONE, &mut cache))
        .sum();

    Ok(count_blinks.to_string())
}

#[cfg(feature = "part_2")]
fn solve_part_2(input: &str) -> Result<String, Error> {
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut cache: FxHashMap<(u64, usize), u64> = FxHashMap::default();
    let count_blinks: u64 = stones
        .iter()
        .map(|stone| count(stone, TIMES_PART_TWO, &mut cache))
        .sum();

    Ok(count_blinks.to_string())
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
