use anyhow::{Error, Result};
use std::cmp::Ordering::*;
const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_14.txt");

struct robots {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

fn parse(input: &str) -> Vec<robots> {
    // let bot_match = regex::Regex::new(r"p=<(\d+),(\d+)> v=<(\d+),(\d+)>").unwrap();
    let bot_match = regex::Regex::new(r"^p\=(\d+),(\d+) v\=(-\d+|\d+),(-\d+|\d+)$").unwrap();
    let mut bots = Vec::new();
    for line in input.lines() {
        if !line.is_empty() && bot_match.is_match(&line) {
            let caps = bot_match.captures(&line).unwrap();
            bots.push(robots {
                px: caps[1].parse().unwrap(),
                py: caps[2].parse().unwrap(),
                vx: caps[3].parse().unwrap(),
                vy: caps[4].parse().unwrap(),
            });
        }
    }
    bots
}

const WIDE: i32 = 101;
const HEIGHT: i32 = 103;
const TIMES: i32 = 100;

#[cfg(feature = "part_1")]
fn solve_part_1(input: &str) -> Result<String, Error> {
    let bots = parse(input);

    let mut quadrants = [0; 4];

    let center_x = WIDE / 2;
    let center_y = HEIGHT / 2;

    bots.iter().for_each(|bot| {
        let new_px = bot.px + TIMES * bot.vx;
        let new_py = bot.py + TIMES * bot.vy;

        // Wrap around the grid dimensions
        let x = (new_px % WIDE + WIDE) % WIDE;
        let y = (new_py % HEIGHT + HEIGHT) % HEIGHT;

        match (x.cmp(&center_x), y.cmp(&center_y)) {
            (Less, Less) => quadrants[0] += 1,
            (Less, Greater) => quadrants[1] += 1,
            (Greater, Less) => quadrants[2] += 1,
            (Greater, Greater) => quadrants[3] += 1,
            _ => (),
        }
    });

    // the solution is the product of all the quadrants
    let solution: i32 = quadrants.iter().product();

    Ok(solution.to_string())
}

// #[cfg(feature = "part_2")]
fn solve_part_2(input: &str) -> Result<String, Error> {
    let mut bots = parse(input);
    let mut seconds = 0;

    // create a vector that works as map of the grid storing the amount of bot in each position throug the wide .
    // use a loop that breaks when found a line of more than 10 lined robots
    loop {
        // Update positions of all robots
        let mut grid = vec![vec![0; WIDE as usize]; HEIGHT as usize];

        bots.iter_mut().for_each(|bot| {
            bot.px = (bot.px + bot.vx) % WIDE;
            bot.py = (bot.py + bot.vy) % HEIGHT;
            // Ensure positions are positive
            bot.px = (bot.px + WIDE) % WIDE;
            bot.py = (bot.py + HEIGHT) % HEIGHT;

            grid[bot.py as usize][bot.px as usize] += 1;
        });

        seconds += 1;

        for y in 0..HEIGHT as usize {
            let mut current_length = 0;
            let mut max_length = 0;

            for x in 0..WIDE as usize {
                if grid[y][x] > 0 {
                    current_length += 1;
                } else {
                    max_length = max_length.max(current_length);
                    current_length = 0;
                }
            }

            // Check the final length (in case the line ends at the edge)
            max_length = max_length.max(current_length);

            // If we found a line long enough, return the result
            if max_length >= 30 {
                //before returnign print the grid
                for y in 0..HEIGHT as usize {
                    for x in 0..WIDE as usize {
                        print!("{}", if grid[y][x] > 0 { '#' } else { '.' });
                    }
                    println!();
                }
                return Ok(seconds.to_string());
            }
        }
    }
}

fn main() -> Result<(), Error> {
    println!("\nDay 14\n------");

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
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";
    const SAMPLE_ANSWER_1: &str = "12";

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
