use std::collections::{HashMap, HashSet};

use anyhow::{Error, Result};

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_06.txt");

//NOTES:
// '^' indicated the guard current position is facing up from the perspective of the map
// obstructions are represented by '#'
// Guards patrol with the following rules:
//      If there is something directly in front of you, turn right 90 degrees.
//      Otherwise, take a step forward.
// count how many distinct positions the guard visits
// The input is an assci map as a grid of '.' and '#' characters and the guard '^'

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone, Hash, Copy)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone, Hash, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Guard {
    position: Position,
    direction: Direction,
    grid: Vec<Vec<char>>,
}

impl Guard {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        let position = Self::find_guard_position(&grid);
        Self {
            position,
            direction: Direction::Up,
            grid,
        }
    }

    fn find_guard_position(grid: &Vec<Vec<char>>) -> Position {
        for (y, row) in grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == '^' {
                    return Position { y, x };
                }
            }
        }
        Position { x: 0, y: 0 } // Default position if guard not found
    }

    fn rotate_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }

    pub fn walk(&mut self) -> usize {
        let mut visited = HashSet::new();
        visited.insert((self.position.x, self.position.y));

        loop {
            if let None = self.next_step() {
                break;
            }
            let (next_x, next_y) = self.next_step().unwrap();

            if self.grid[next_y][next_x] == '#' {
                self.rotate_right();
                continue;
            }
            //move to next position
            self.position.x = next_x;
            self.position.y = next_y;
            visited.insert((self.position.x, self.position.y));
        }
        visited.len()
    }

    fn next_step(&self) -> Option<(usize, usize)> {
        let height = self.grid.len();
        let width = self.grid[0].len();
        let (next_x, next_y) = match self.direction {
            Direction::Up => {
                if self.position.y == 0 {
                    return None;
                }
                (self.position.x, self.position.y - 1)
            }
            Direction::Down => {
                if self.position.y == height - 1 {
                    return None;
                }
                (self.position.x, self.position.y + 1)
            }
            Direction::Left => {
                if self.position.x == 0 {
                    return None;
                }
                (self.position.x - 1, self.position.y)
            }
            Direction::Right => {
                if self.position.x == width - 1 {
                    // break;
                    return None;
                }
                (self.position.x + 1, self.position.y)
            }
        };
        Some((next_x, next_y))
    }

    /// 1. Instead of trying to detect loops during a single walk, we now test each possible position
    /// 2. For each position, we simulate placing an obstruction and check if it creates a true loop
    /// 3. A true loop is detected when we revisit a position with the same direction we had before
    /// 4. We maintain the guard's original state by resetting after each test
    pub fn walk_in_loop(&mut self) -> usize {
        let mut obstacles = HashSet::new();

        loop {
            if let None = self.next_step() {
                break;
            }
            let (next_x, next_y) = self.next_step().unwrap();

            if !obstacles.contains(&(next_x, next_y)) && self.grid[next_y][next_x] != '#' {
                let mut grid_with_obstacles = self.grid.clone();
                grid_with_obstacles[next_y][next_x] = '#';
                let mut new_guard = Self::new(grid_with_obstacles);
                if new_guard.detect_loop() {
                    obstacles.insert((next_x, next_y));
                }
            }

            if self.grid[next_y][next_x] == '#' {
                self.rotate_right();
                continue;
            }
            //move to next position
            self.position.x = next_x;
            self.position.y = next_y;
        }

        obstacles.len()
    }

    fn detect_loop(&mut self) -> bool {
        let mut visited = HashSet::new();
        let mut state = (self.position, self.direction);

        loop {
            if !visited.insert(state) {
                return true; // Found a loop
            }

            match self.next_step() {
                None => return false, // Reached edge of grid
                Some((next_x, next_y)) => {
                    if self.grid[next_y][next_x] == '#' {
                        self.rotate_right();
                    } else {
                        self.position.x = next_x;
                        self.position.y = next_y;
                    }
                    state = (self.position, self.direction);
                }
            }
        }
    }
}

#[cfg(feature = "part_1")]
fn solve_part_1(input: &str) -> Result<String, Error> {
    // from input load the grid a 2d.
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut guard = Guard::new(grid);
    let solution = guard.walk();

    Ok(solution.to_string())
}

#[cfg(feature = "part_2")]
fn solve_part_2(input: &str) -> Result<String, Error> {
    // from input load the grid a 2d.
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut guard = Guard::new(grid);
    let solution = guard.walk_in_loop();

    Ok(solution.to_string())
}

fn main() -> Result<(), Error> {
    println!("\nDay 06\n------");

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
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    const SAMPLE_ANSWER_1: &str = "41";

    assert_eq!(solve_part_1(SAMPLE_INPUT_1).unwrap(), SAMPLE_ANSWER_1);
}

#[cfg(feature = "part_2")]
#[test]
fn sample_part_2() {
    const SAMPLE_INPUT_2: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    const SAMPLE_ANSWER_2: &str = "6";

    assert_eq!(solve_part_2(SAMPLE_INPUT_2).unwrap(), SAMPLE_ANSWER_2);
}
