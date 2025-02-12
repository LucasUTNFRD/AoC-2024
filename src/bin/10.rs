use rayon::prelude::*;
use std::collections::HashSet;

use anyhow::{Error, Result};

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_10.txt");

#[derive(Debug)]
struct TopographicMap {
    grid: Vec<Vec<u8>>,
    trailhead: Vec<(usize, usize)>,
}

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl TopographicMap {
    pub fn new(input: &str) -> Self {
        let mut grid = Vec::new();
        let mut trailhead = Vec::new();

        for (row, line) in input.lines().enumerate() {
            let mut grid_row = Vec::new();
            for (col, c) in line.bytes().enumerate() {
                grid_row.push(c - b'0');
                if c == b'0' {
                    trailhead.push((row, col))
                }
            }
            grid.push(grid_row);
        }

        Self { grid, trailhead }
    }
    fn hike(
        &self,
        starting_pos: (usize, usize),
        visited: &mut HashSet<(usize, usize)>,
        reachable_nines: &mut HashSet<(usize, usize)>,
    ) {
        let (row, col) = starting_pos;
        let current_height = self.grid[row][col];

        // Base case: If we reach height 9, return 1 (found a valid trail)
        if current_height == 9 {
            reachable_nines.insert((row, col));
            return;
        }

        // Mark the current cell as visited
        visited.insert((row, col));

        // Explore all four directions
        for (dr, dc) in DIRECTIONS {
            let new_row = (row as i32 + dr) as usize;
            let new_col = (col as i32 + dc) as usize;

            // Check if the new cell is within bounds
            if new_row < self.grid.len() && new_col < self.grid[0].len() {
                let next_height = self.grid[new_row][new_col];

                // Check if the next cell is unvisited and has height +1
                if next_height == current_height + 1 && !visited.contains(&(new_row, new_col)) {
                    self.hike((new_row, new_col), visited, reachable_nines);
                }
            }
        }

        // Backtrack: Unmark the current cell as visited
        visited.remove(&(row, col));
    }

    /// DFS to count the number of reachable height 9 cells from a starting position
    fn hike_with_rating(
        &self,
        starting_pos: (usize, usize),
        visited: &mut HashSet<(usize, usize)>,
    ) -> usize {
        let (row, col) = starting_pos;
        let current_height = self.grid[row][col];

        // Base case: If we reach height 9, return 1 (found a valid trail)
        if current_height == 9 {
            return 1;
        }

        // Mark the current cell as visited
        visited.insert((row, col));

        let mut count = 0;

        // Explore all four directions
        for (dr, dc) in DIRECTIONS {
            let new_row = (row as i32 + dr) as usize;
            let new_col = (col as i32 + dc) as usize;

            // Check if the new cell is within bounds
            if new_row < self.grid.len() && new_col < self.grid[0].len() {
                let next_height = self.grid[new_row][new_col];

                // Check if the next cell is unvisited and has height +1
                if next_height == current_height + 1 && !visited.contains(&(new_row, new_col)) {
                    count += self.hike_with_rating((new_row, new_col), visited);
                }
            }
        }

        // Backtrack: Unmark the current cell as visited
        visited.remove(&(row, col));

        count
    }

    /// Count the number of reachable height 9 cells for each trailhead
    pub fn count_trailhead(&self) -> usize {
        self.trailhead
            // .iter()
            .par_iter()
            .map(|&trail| {
                let mut visited = HashSet::new();
                let mut reachable_nines = HashSet::new();
                self.hike(trail, &mut visited, &mut reachable_nines);
                reachable_nines.len()
            })
            .sum()
    }
    /// Count the number of reachable height 9 cells for each trailhead with different paths
    pub fn count_trailhead_part_2(&self) -> usize {
        self.trailhead
            // .iter()
            .par_iter()
            .map(|&trail| {
                let mut visited = HashSet::new();
                self.hike_with_rating(trail, &mut visited)
                // reachable_nines.len()
            })
            .sum()
    }
}

#[cfg(feature = "part_1")]
fn solve_part_1(input: &str) -> Result<String, Error> {
    let map = TopographicMap::new(input);

    let count = map.count_trailhead();

    Ok(count.to_string())
}

#[cfg(feature = "part_2")]
fn solve_part_2(input: &str) -> Result<String, Error> {
    let map = TopographicMap::new(input);

    let count = map.count_trailhead_part_2();

    Ok(count.to_string())
}

fn main() -> Result<(), Error> {
    println!("\nDay 10\n------");

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
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
    const SAMPLE_ANSWER_1: &str = "36";

    assert_eq!(solve_part_1(SAMPLE_INPUT_1).unwrap(), SAMPLE_ANSWER_1);
}

#[cfg(feature = "part_2")]
#[test]
fn sample_part_2() {
    const SAMPLE_INPUT_2: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
    const SAMPLE_ANSWER_2: &str = "81";

    assert_eq!(solve_part_2(SAMPLE_INPUT_2).unwrap(), SAMPLE_ANSWER_2);
}
