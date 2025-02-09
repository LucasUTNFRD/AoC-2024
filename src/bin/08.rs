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

// #[cfg(feature = "part_1")]
// fn solve_part_1(input: &str) -> Result<String, Error> {
//     let antennas = parse_input(input);

//     let height = input.lines().count() as i32;
//     let width = input.lines().next().unwrap().chars().count() as i32;

//     // Use HashSet to store unique antinode locations
//     let mut antinodes: HashSet<Point> = HashSet::new();

//     // Process each frequency (character) separately
//     for (_, antenna_positions) in antennas.iter() {
//         // For each pair of antennas of the same frequency
//         for (i, &p) in antenna_positions.iter().enumerate() {
//             for (j, &q) in antenna_positions.iter().enumerate() {
//                 if i != j {
//                     let diff = q.sub(p);
//                     let antinode_1 = q.add(diff);
//                     let antinode_2 = p.sub(diff);
//                     if is_in_bounds(antinode_1, width, height) {
//                         antinodes.insert(antinode_1);
//                     }

//                     if is_in_bounds(antinode_2, width, height) {
//                         antinodes.insert(antinode_2);
//                     }
//                 }
//             }
//         }
//     }

//     Ok(antinodes.len().to_string())
// }

fn visualize_grid(
    input: &str,
    antennas: &HashMap<char, Vec<Point>>,
    antinodes: &HashSet<Point>,
    current_pair: Option<(Point, Point)>,
) {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    // Create empty grid
    let mut grid = vec![vec!['.'; width]; height];

    // Place antennas
    for (symbol, positions) in antennas {
        for &(x, y) in positions {
            grid[y as usize][x as usize] = *symbol;
        }
    }

    // Place antinodes
    for &(x, y) in antinodes {
        if grid[y as usize][x as usize] == '.' {
            grid[y as usize][x as usize] = '×'; // Using × for antinodes
        }
    }

    // Highlight current pair being processed
    if let Some((p1, p2)) = current_pair {
        grid[p1.1 as usize][p1.0 as usize] = '█';
        grid[p2.1 as usize][p2.0 as usize] = '█';
    }

    // Print grid with border
    println!("╔{}╗", "═".repeat(width + 2));
    for row in grid {
        print!("║ ");
        for cell in row {
            match cell {
                '×' => print!("\x1b[31m×\x1b[0m"),   // Red for antinodes
                '█' => print!("\x1b[33m█\x1b[0m"),   // Yellow for current pair
                '.' => print!("\x1b[90m.\x1b[0m"),   // Dark gray for empty space
                c => print!("\x1b[36m{}\x1b[0m", c), // Cyan for antennas
            }
        }
        println!(" ║");
    }
    println!("╚{}╝", "═".repeat(width + 2));
    println!();
}

#[cfg(feature = "part_1")]
fn solve_part_1(input: &str) -> Result<String, Error> {
    let antennas = parse_input(input);
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().chars().count() as i32;
    let mut antinodes: HashSet<Point> = HashSet::new();

    println!("\nVisualization of Part 1:");
    println!("----------------------");
    println!("Legend:");
    println!("× = Antinode");
    println!("█ = Current antenna pair");
    println!("Colored letters = Antennas\n");

    for (_, antenna_positions) in antennas.iter() {
        for (i, &p) in antenna_positions.iter().enumerate() {
            for (j, &q) in antenna_positions.iter().enumerate() {
                if i != j {
                    let diff = q.sub(p);
                    let antinode_1 = q.add(diff);
                    let antinode_2 = p.sub(diff);

                    // Visualize current state
                    visualize_grid(input, &antennas, &antinodes, Some((p, q)));

                    if is_in_bounds(antinode_1, width, height) {
                        antinodes.insert(antinode_1);
                    }
                    if is_in_bounds(antinode_2, width, height) {
                        antinodes.insert(antinode_2);
                    }

                    // Small delay to make visualization visible
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
            }
        }
    }

    // Show final state
    println!("Final state:");
    visualize_grid(input, &antennas, &antinodes, None);

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

                    visualize_grid(input, &antennas, &antinodes, Some((p, q)));
                    std::thread::sleep(std::time::Duration::from_millis(500));
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
