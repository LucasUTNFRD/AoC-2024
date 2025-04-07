use std::{
    char,
    ops::{Add, Sub},
};

use anyhow::{Error, Result};

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_15.txt");

#[derive(Default, Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Position {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

struct Warehouse {
    grid: Vec<Vec<char>>,
    height: usize,
    width: usize,
    lanternfish: Position,
}

impl Warehouse {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let mut position = Position::default();
        for (y, row) in grid.iter().enumerate() {
            for (x, _col) in row.iter().enumerate() {
                if row[x] == '@' {
                    position = Position::new(x as i32, y as i32)
                }
            }
        }
        let height = grid.len();
        let width = grid[0].len();

        Self {
            grid,
            height,
            width,
            lanternfish: position,
        }
    }

    pub fn sum_of_all_boxes(&self) -> u32 {
        let mut gps_sum = 0;
        const GPS_MULTIPLIER: u32 = 100;

        for (y, row) in self.grid.iter().enumerate() {
            for (x, _col) in row.iter().enumerate() {
                if row[x] == 'O' {
                    gps_sum += GPS_MULTIPLIER * y as u32 + x as u32;
                }
            }
        }

        gps_sum
    }

    pub fn move_to(&mut self, direction: Position) {
        let dir = direction;
        let pos = self.lanternfish;

        // Calculate the new position after moving one step
        let original_new_pos = pos + dir;

        // Check if the new position is a wall
        if self.grid[original_new_pos.y as usize][original_new_pos.x as usize] == '#' {
            return;
        }

        // Collect all consecutive boxes in the direction of movement
        let mut current_pos = original_new_pos;
        let mut obstacles = vec![];
        while self.grid[current_pos.y as usize][current_pos.x as usize] == 'O' {
            obstacles.push(current_pos);
            current_pos = current_pos + dir;
        }

        // Determine the final position after the last box
        let final_pos = current_pos;

        // Check if the final position is blocked by a wall or out of bounds
        if self.grid[final_pos.y as usize][final_pos.x as usize] == '#' {
            return;
        }

        // Move all obstacles (boxes) one step in the direction
        for &obstacle in obstacles.iter().rev() {
            let new_obs_pos = obstacle + dir;
            // Update the grid positions for the moved box
            self.grid[new_obs_pos.y as usize][new_obs_pos.x as usize] = 'O';
            self.grid[obstacle.y as usize][obstacle.x as usize] = '.';
        }

        // Update the grid
        self.grid[pos.y as usize][pos.x as usize] = '.';
        self.grid[original_new_pos.y as usize][original_new_pos.x as usize] = '@';
        // Update the lanternfish position
        self.lanternfish = original_new_pos;
    }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, &str) {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let grid = parse_grid(grid);
    (grid, moves)
}

fn parse_grid(grid: &str) -> Vec<Vec<char>> {
    grid.lines().map(|line| line.chars().collect()).collect()
}

// #[cfg(feature = "part_1")]
fn solve_part_1(input: &str) -> Result<String, Error> {
    let (grid, moves) = parse_input(input);
    let mut warehouse = Warehouse::new(grid);

    for movement in moves.chars().filter(|&c| !char::is_whitespace(c)) {
        let direction = match movement {
            '>' => Position::new(1, 0),
            '<' => Position::new(-1, 0),
            '^' => Position::new(0, -1),
            'v' => Position::new(0, 1),
            _ => unreachable!(),
        };

        warehouse.move_to(direction);
    }

    Ok(warehouse.sum_of_all_boxes().to_string())
}

#[cfg(feature = "part_2")]
fn solve_part_2(input: &str) -> Result<String, Error> {
    let solution = input.lines().next().unwrap().replace("input", "answer");

    Ok(solution)
}

fn main() -> Result<(), Error> {
    println!("\nDay 15\n------");

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
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";
    const SAMPLE_ANSWER_1: &str = "10092";

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
