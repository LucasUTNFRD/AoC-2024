use std::collections::HashMap;
use std::time::Instant;

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
    // evaluate_equation_optimal(numbers, numbers.len() - 1, target)
}

//
fn evaluate_equation_optimal(numbers: &[i64], index: usize, target: i64) -> bool {
    // Base case: if we're at the first number
    if index == 0 {
        return numbers[0] == target;
    }

    let current_num = numbers[index];

    // Try addition: if R + an = T, then R = T - an
    let sub_result = target - current_num;
    if evaluate_equation_optimal(numbers, index - 1, sub_result) {
        return true;
    }

    // Try multiplication: if R × an = T, then R = T/an (if T is divisible by an)
    if current_num != 0 && target % current_num == 0 {
        let div_result = target / current_num;
        if evaluate_equation_optimal(numbers, index - 1, div_result) {
            return true;
        }
    }

    // Try concatenation: if R || an = T, then T must end with an
    let current_str = current_num.to_string();
    let target_str = target.to_string();
    if target_str.ends_with(&current_str) {
        if let Some(remaining_str) = target_str.strip_suffix(&current_str) {
            if let Ok(remaining) = remaining_str.parse::<i64>() {
                if evaluate_equation_optimal(numbers, index - 1, remaining) {
                    return true;
                }
            }
        }
    }

    false
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

    let concatenated_value = format!("{}{}", current_result, numbers[index + 1])
        .parse::<i64>()
        .unwrap();
    if evaluate_equation(numbers, index + 1, concatenated_value, target) {
        return true; //they added a third operations which is || combines the digits from its left and right inputs into a single number
    }

    false
}

#[cfg(feature = "part_2")]
fn solve_part_2(input: &str) -> Result<String, Error> {
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

fn main() -> Result<(), Error> {
    println!("\nDay 07\n------");

    #[cfg(feature = "part_1")]
    {
        let start = Instant::now();
        let answer_part_1 = solve_part_1(PUZZLE_INPUT)?;
        let duration = start.elapsed();
        println!("Part One: {} (took: {:?})", answer_part_1, duration);
    }

    #[cfg(feature = "part_2")]
    {
        let start = Instant::now();
        let answer_part_2 = solve_part_2(PUZZLE_INPUT)?;
        let duration = start.elapsed();
        println!("Part Two: {} (took: {:?})", answer_part_2, duration);
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
    const SAMPLE_ANSWER_2: &str = "11387";

    assert_eq!(solve_part_2(SAMPLE_INPUT_2).unwrap(), SAMPLE_ANSWER_2);
}
