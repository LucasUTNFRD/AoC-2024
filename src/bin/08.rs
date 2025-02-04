use anyhow::{Error, Result};
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Mul, Sub};

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_08.txt");

type Point = (i32, i32);

trait PointOps {
    fn add(&self, other: Point) -> Point;
    fn sub(&self, other: Point) -> Point;
    fn mul(&self, scalar: i32) -> Point;
}

impl PointOps for Point {
    fn add(&self, other: Point) -> Point {
        (self.0 + other.0, self.1 + other.1)
    }

    fn sub(&self, other: Point) -> Point {
        (self.0 - other.0, self.1 - other.1)
    }

    fn mul(&self, scalar: i32) -> Point {
        (self.0 * scalar, self.1 * scalar)
    }
}
fn parse_input(input: &str) -> HashMap<char, Vec<Point>> {
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c != '.' {
                antennas
                    .entry(c)
                    .or_insert(Vec::new())
                    .push((x as i32, y as i32));
            }
        }
    }
    antennas
}

fn is_in_bounds(point: Point, width: i32, height: i32) -> bool {
    point.0 >= 0 && point.0 < width && point.1 >= 0 && point.1 < height
}

#[cfg(feature = "part_1")]
fn solve_part_1(input: &str) -> Result<String, Error> {
    let antennas = parse_input(input);

    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().chars().count() as i32;

    // Use HashSet to store unique antinode locations
    let mut antinodes: HashSet<Point> = HashSet::new();

    // Process each frequency (character) separately
    for (_, antenna_positions) in antennas.iter() {
        // For each pair of antennas of the same frequency
        for (i, &p) in antenna_positions.iter().enumerate() {
            for (j, &q) in antenna_positions.iter().enumerate() {
                if i != j {
                    let diff = q.sub(p);
                    let antinode_1 = q.add(diff);
                    let antinode_2 = p.sub(diff);
                    if is_in_bounds(antinode_1, width, height) {
                        antinodes.insert(antinode_1);
                    }

                    if is_in_bounds(antinode_2, width, height) {
                        antinodes.insert(antinode_2);
                    }
                }
            }
        }
    }

    Ok(antinodes.len().to_string())
}

#[cfg(feature = "part_2")]
fn solve_part_2(input: &str) -> Result<String, Error> {
    let antennas = parse_input(input);

    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().chars().count() as i32;

    // Use HashSet to store unique antinode locations
    let mut antinodes: HashSet<Point> = HashSet::new();

    for (_, antenna_positions) in antennas.iter() {
        for (i, &p) in antenna_positions.iter().enumerate() {
            for (j, &q) in antenna_positions.iter().enumerate() {
                if i != j {
                    let diff = q.sub(p);

                    let mut antinode = q;
                    while is_in_bounds(antinode, width, height) {
                        antinodes.insert(antinode);
                        antinode = antinode.add(diff);
                    }

                    let mut antinode = p;
                    while is_in_bounds(antinode, width, height) {
                        antinodes.insert(antinode);
                        antinode = antinode.sub(diff);
                    }
                }
            }
        }
    }

    Ok(antinodes.len().to_string())
}

fn main() -> Result<(), Error> {
    println!("\nDay 08\n------");

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
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
    const SAMPLE_ANSWER_1: &str = "14";

    assert_eq!(solve_part_1(SAMPLE_INPUT_1).unwrap(), SAMPLE_ANSWER_1);
}

#[cfg(feature = "part_2")]
#[test]
fn sample_part_2() {
    const SAMPLE_INPUT_2: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
    const SAMPLE_ANSWER_2: &str = "34";

    assert_eq!(solve_part_2(SAMPLE_INPUT_2).unwrap(), SAMPLE_ANSWER_2);
}
