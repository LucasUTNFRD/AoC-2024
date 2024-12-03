use anyhow::{Error, Result};
use regex::Regex;

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_03.txt");

#[cfg(feature = "part_1")]
fn solve_part_1(input: &str) -> Result<String, Error> {
    // Convert to array of arrays of integers
    // create a regex that only matches the following pattern
    // mul(x,y) where x and y are 3 digit numbers
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    //from puzzle input extract all the mul(x,y) instructions and store only the x and y values as
    //tuples in an array of tuples
    let instructions: Vec<(i32, i32)> = re
        .captures_iter(input)
        .map(|cap| {
            (
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
            )
        })
        .collect();

    let mut solution = 0;

    for (x, y) in instructions {
        solution += x * y;
    }

    Ok(solution.to_string())
}

#[cfg(feature = "part_2")]
fn solve_part_2(input: &str) -> Result<String, Error> {
    let solution = input.lines().next().unwrap().replace("input", "answer");

    Ok(solution)
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
    let sample_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    // Convert to array of arrays of integers
    // create a regex that only matches the following pattern
    // mul(x,y) where x and y are 3 digit numbers
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    //from puzzle input extract all the mul(x,y) instructions and store only the x and y values as
    //tuples in an array of tuples
    let instructions: Vec<(i32, i32)> = re
        .captures_iter(sample_input)
        .map(|cap| {
            (
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
            )
        })
        .collect();

    let mut solution = 0;

    for (x, y) in instructions {
        solution += x * y;
    }

    assert_eq!(solution, 161);
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
