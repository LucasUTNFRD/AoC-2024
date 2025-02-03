use std::collections::HashMap;

use anyhow::{Error, Result};

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_07.txt");

struct Equation {
    target: i64,
    numbers: Vec<i64>,
}

#[cfg(feature = "part_1")]
fn solve_part_1(input: &str) -> Result<String, Error> {
    let parsed_input: Vec<Equation> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(':');
            let target = parts.next().unwrap().trim().parse().unwrap();
            let numbers = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            Equation { target, numbers }
        })
        .collect();

    let total_sum: i64 = parsed_input
        .iter()
        .filter(|eq| is_valid(&eq.numbers, eq.target))
        .map(|eq| eq.target)
        .sum();
    Ok(total_sum.to_string())
}

fn is_valid(numbers: &[i64], target: i64) -> bool {
    evaluate_equation(numbers, 0, numbers[0], target)
}

fn evaluate_equation(numbers: &[i64], index: usize, current_result: i64, target: i64) -> bool {
    if index == numbers.len() - 1 {
        return current_result == target;
    }

    if evaluate_equation(
        numbers,
        index + 1,
        current_result + numbers[index + 1],
        target,
    ) {
        return true;
    }

    if evaluate_equation(
        numbers,
        index + 1,
        current_result * numbers[index + 1],
        target,
    ) {
        return true;
    }

    false
}

#[cfg(feature = "part_2")]
fn solve_part_2(input: &str) -> Result<String, Error> {
    let solution = input.lines().next().unwrap().replace("input", "answer");

    Ok(solution)
}

fn main() -> Result<(), Error> {
    println!("\nDay 07\n------");

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
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
    const SAMPLE_ANSWER_1: &str = "3749";

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
