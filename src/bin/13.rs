use anyhow::{Error, Result};
use regex::Regex;
use std::str::FromStr;

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_13.txt");

// Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400
//
// point (0,0)
// 94x + 22y = 8400
// 34x + 67y = 5400

//To avoid floating points precision i will solve this as diophantine equation

const BUTTOM_A: i64 = 3;
const BUTTOM_B: i64 = 1;

const SCALE_PART_2: i64 = 10000000000000;

#[derive(Debug)]
struct Equation {
    eq1: (i64, i64, i64), //x_coef,y_coef,result
    eq2: (i64, i64, i64),
}

impl Equation {
    fn solve(&mut self) -> Option<(i64, i64)> {
        //if executino for part 2 then sum the scale to the result
        if cfg!(feature = "part_2") {
            self.eq1.2 += SCALE_PART_2;
            self.eq2.2 += SCALE_PART_2;
        }

        let (a1, b1, c1) = self.eq1;
        let (a2, b2, c2) = self.eq2;

        // Solve the system:
        // a1 * x + b1 * y = c1
        // a2 * x + b2 * y = c2

        // First, find the determinant of the system
        let determinant = a1 * b2 - a2 * b1;

        if determinant == 0 {
            // The system is either inconsistent or has infinitely many solutions
            return None;
        }

        // Find the solution using Cramer's rule
        let x_numerator = c1 * b2 - c2 * b1;
        let y_numerator = a1 * c2 - a2 * c1;

        if x_numerator % determinant != 0 || y_numerator % determinant != 0 {
            // No integer solutions exist
            return None;
        }

        let x = x_numerator / determinant;
        let y = y_numerator / determinant;

        Some((x, y))
    }
}

#[cfg(feature = "part_1")]
fn solve_part_1(input: &str) -> Result<String, Error> {
    let mut equations = Vec::new();

    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\s+Button B: X\+(\d+), Y\+(\d+)\s+Prize: X=(\d+), Y=(\d+)",
    )?;

    for caps in re.captures_iter(input) {
        // Extract the captured groups and parse them into integers
        let x1 = i64::from_str(&caps[1])?;
        let y1 = i64::from_str(&caps[2])?;
        let x2 = i64::from_str(&caps[3])?;
        let y2 = i64::from_str(&caps[4])?;
        let prize_x = i64::from_str(&caps[5])?;
        let prize_y = i64::from_str(&caps[6])?;

        equations.push(Equation {
            eq1: (x1, x2, prize_x),
            eq2: (y1, y2, prize_y),
        });
    }

    let total_tokens: i64 = equations
        .iter_mut()
        .filter_map(|eq| eq.solve())
        .map(|(x, y)| x * BUTTOM_A + y * BUTTOM_B)
        .sum();

    Ok(total_tokens.to_string())
}

#[cfg(feature = "part_2")]
fn solve_part_2(input: &str) -> Result<String, Error> {
    let mut equations = Vec::new();

    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\s+Button B: X\+(\d+), Y\+(\d+)\s+Prize: X=(\d+), Y=(\d+)",
    )?;

    for caps in re.captures_iter(input) {
        // Extract the captured groups and parse them into integers
        let x1 = i64::from_str(&caps[1])?;
        let y1 = i64::from_str(&caps[2])?;
        let x2 = i64::from_str(&caps[3])?;
        let y2 = i64::from_str(&caps[4])?;
        let prize_x = i64::from_str(&caps[5])?;
        let prize_y = i64::from_str(&caps[6])?;

        equations.push(Equation {
            eq1: (x1, x2, prize_x),
            eq2: (y1, y2, prize_y),
        });
    }

    let total_tokens: i64 = equations
        .iter_mut()
        .filter_map(|eq| eq.solve())
        .map(|(x, y)| x * BUTTOM_A + y * BUTTOM_B)
        .sum();

    Ok(total_tokens.to_string())
}

fn main() -> Result<(), Error> {
    println!("\nDay 13\n------");

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
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
    const SAMPLE_ANSWER_1: &str = "480";

    assert_eq!(solve_part_1(SAMPLE_INPUT_1).unwrap(), SAMPLE_ANSWER_1);
}

#[cfg(feature = "part_2")]
#[test]
fn sample_part_2() {
    const SAMPLE_INPUT_2: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
    const SAMPLE_ANSWER_2: &str = "sample part 2 answer";

    assert_eq!(solve_part_2(SAMPLE_INPUT_2).unwrap(), SAMPLE_ANSWER_2);
}
