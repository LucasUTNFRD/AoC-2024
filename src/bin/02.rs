use anyhow::{Error, Result};

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_02.txt");

fn is_safe_report(levels: &[i32]) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;

    for i in 1..levels.len() {
        let diff = levels[i] - levels[i - 1];

        if diff < 1 || diff > 3 {
            is_increasing = false;
        }

        let rev_diff = levels[i - 1] - levels[i];
        if rev_diff < 1 || rev_diff > 3 {
            is_decreasing = false;
        }

        if !is_increasing && !is_decreasing {
            return false;
        }
    }

    is_increasing || is_decreasing
}

fn is_safe_with_dampener(levels: &[i32]) -> bool {
    if is_safe_report(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut temp_levels = levels.to_vec();
        temp_levels.remove(i);
        if is_safe_report(&temp_levels) {
            return true;
        }
    }

    false
}

#[cfg(feature = "part_1")]
fn solve_part_1(input: &str) -> Result<String, Error> {
    let parsed_input: Vec<Vec<i32>> = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let solution = parsed_input
        .iter()
        .filter(|report| is_safe_report(report))
        .count();

    Ok(solution.to_string())
}

#[cfg(feature = "part_2")]
fn solve_part_2(input: &str) -> Result<String, Error> {
    let parsed_input: Vec<Vec<i32>> = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let solution = parsed_input
        .iter()
        .filter(|report| is_safe_with_dampener(report))
        .count();

    Ok(solution.to_string())
}

fn main() -> Result<(), Error> {
    println!("\nDay 02\n------");

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

#[cfg(test)]
#[test]
fn sample_part_1() {
    // Convert to array of arrays of integers
    let result: Vec<Vec<i32>> = PUZZLE_INPUT
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    println!("{:?}", result);
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
