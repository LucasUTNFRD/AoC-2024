use anyhow::{Error, Result};
use std::collections::HashMap;

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_01.txt");

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let first = iter.next().unwrap().parse().unwrap();
        let second = iter.next().unwrap().parse().unwrap();
        left.push(first);
        right.push(second);
    }

    (left, right)
}

#[cfg(feature = "part_1")]
fn solve_part_1(input: &str) -> Result<String, Error> {
    let (mut left, mut right) = parse_input(input);

    left.sort();
    right.sort();

    let solution: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    Ok(solution.to_string())
}

#[cfg(feature = "part_2")]
fn solve_part_2(input: &str) -> Result<String, Error> {
    let (left, right) = parse_input(input);

    let mut right_list_frequency = HashMap::new();

    right
        .iter()
        .for_each(|&n| *right_list_frequency.entry(n).or_default() += 1);

    let mut solution: i32 = 0;

    for l in left {
        solution += right_list_frequency.get(&l).unwrap_or(&0) * l;
    }
    Ok(solution.to_string())
}

fn main() -> Result<(), Error> {
    println!("\nDay 01\n------");

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

// Test as playground to see how to parse correctly the input file day1.txt
#[cfg(test)]
#[test]
fn test_input() {
    const SAMPLE_INPUT_1: &str = "12 34\n56 78\n90 12";

    let lines: Vec<(i32, i32)> = SAMPLE_INPUT_1
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let first = iter.next().unwrap().parse().unwrap();
            let second = iter.next().unwrap().parse().unwrap();
            (first, second)
        })
        .collect();

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = lines.iter().cloned().unzip();
    // sort left and right vectors from smallest to largest
    left.sort();
    right.sort();
    //print them sorted
    println!("{:?}", left);
    println!("{:?}", right);
}
