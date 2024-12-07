use anyhow::{Error, Result};

// set the different directons as a vec
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_04.txt");
const KEYWORD: &str = "XMAS";

#[cfg(feature = "part_1")]
fn solve_part_1(input: &str) -> Result<String, Error> {
    use std::char;

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;
    for row in 0..rows {
        for col in 0..cols {
            for (dr, dc) in DIRECTIONS.iter() {
                if is_valid_direction(&grid, row, col, *dr, *dc) {
                    count += 1
                }
            }
        }
    }

    Ok(count.to_string())
}

fn is_valid_direction(
    grid: &[Vec<char>],
    start_row: usize,
    start_col: usize,
    dr: i32,
    dc: i32,
) -> bool {
    let keyword_chars: Vec<char> = KEYWORD.chars().collect();
    let keyword_len = keyword_chars.len() as i32;

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    //Bound checking
    let end_row = start_row as i32 + ((keyword_len - 1) * dr);
    let end_col = start_col as i32 + ((keyword_len - 1) * dc);

    if end_row < 0 || end_row >= rows || end_col < 0 || end_col >= cols {
        return false;
    }

    for (i, &char) in keyword_chars.iter().enumerate() {
        let r = start_row as i32 + i as i32 * dr;
        let c = start_col as i32 + i as i32 * dc;

        if grid[r as usize][c as usize] != char {
            return false;
        }
    }

    true
}

#[cfg(feature = "part_2")]
fn solve_part_2(input: &str) -> Result<String, Error> {
    let solution = input.lines().next().unwrap().replace("input", "answer");

    Ok(solution)
}

fn main() -> Result<(), Error> {
    println!("\nDay 04\n------");

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
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    const SAMPLE_ANSWER_1: &str = "18";

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
