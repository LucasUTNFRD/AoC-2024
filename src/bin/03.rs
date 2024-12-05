use anyhow::{Error, Result};
use regex::Regex;

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_03.txt");
//#[cfg(feature = "part_1")]
fn solve_part_1(input: &str) -> Result<String, Error> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let instructions: Vec<(i32, i32)> = re
        .captures_iter(input)
        .map(|cap| {
            (
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
            )
        })
        .collect();

    let solution = instructions.iter().fold(0, |acc, (a, b)| acc + a * b);

    Ok(solution.to_string())
}

#[cfg(feature = "part_2")]
fn solve_part_2(input: &str) -> Result<String, Error> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    //Only the most recent do() or don't() instruction applies
    let mut enabled = true;
    let instructions: Vec<(i32, i32)> = re
        .captures_iter(input)
        .filter(|cap| {
            if cap.get(0).unwrap().as_str() == "do()" {
                enabled = true;
                return false;
            } else if cap.get(0).unwrap().as_str() == "don't()" {
                enabled = false;
            }
            enabled
        })
        .map(|cap| {
            (
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
            )
        })
        .collect();

    let solution = instructions.iter().fold(0, |acc, (a, b)| acc + a * b);

    Ok(solution.to_string())
}

fn main() -> Result<(), Error> {
    println!("\nDay 03\n------");

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
    let sample_input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let sample_input2 =
        "don'txmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    //Only the most recent do() or don't() instruction applies
    let mut enabled = true;
    let instructions: Vec<(i32, i32)> = re
        .captures_iter(sample_input)
        .filter(|cap| {
            if cap.get(0).unwrap().as_str() == "do()" {
                enabled = true;
                return false;
            } else if cap.get(0).unwrap().as_str() == "don't()" {
                enabled = false;
            }
            enabled
        })
        .map(|cap| {
            (
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
            )
        })
        .collect();

    let mut enabled1 = false;
    let instructions2: Vec<(i32, i32)> = re
        .captures_iter(sample_input2)
        .filter(|cap| {
            if cap.get(0).unwrap().as_str() == "do()" {
                enabled1 = true;
                return false;
            } else if cap.get(0).unwrap().as_str() == "don't()" {
                enabled1 = false;
            }
            enabled1
        })
        .map(|cap| {
            (
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
            )
        })
        .collect();

    let expected_input1 = vec![(2, 4), (8, 5)];
    let expected_input2 = vec![(8, 5)];

    assert_eq!(instructions, expected_input1);
    assert_eq!(instructions2, expected_input2);
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
