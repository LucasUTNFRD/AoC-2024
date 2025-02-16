use rustc_hash::FxHashSet as HashSet;

use anyhow::{Error, Result};

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_12.txt");

#[derive(Debug)]
struct Garden {
    plot: Vec<Vec<char>>,
    regions: Vec<Region>,
}

#[derive(Debug, Default)]
struct Region {
    area: u32,
    perimeter: u32,
    sides: u32,
    crop: Vec<(usize, usize)>,
}

impl Region {
    fn sides(&self) -> u32 {
        //count corners for each crop
    }
}

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

const CORNERS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

impl Garden {
    pub fn new(input: &str) -> Self {
        let garden_plot: Vec<Vec<char>> =
            input.lines().map(|line| line.chars().collect()).collect();
        Self {
            plot: garden_plot,
            regions: vec![],
        }
    }

    pub fn fencing_price(&self) -> u32 {
        let regions = self.get_regions();
        regions
            .iter()
            .map(|region| region.area * region.perimeter)
            .sum()
    }

    pub fn fencing_price_part_two(&self) -> u32 {
        let regions = self.get_regions();
        regions
            .iter()
            .map(|region| region.area * region.sides)
            .sum()
    }

    fn get_regions(&self) -> Vec<Region> {
        let mut regions = Vec::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::default();

        for i in 0..self.plot.len() {
            for j in 0..self.plot[0].len() {
                if !visited.contains(&(i, j)) {
                    let mut region = Region::default();
                    let plant_type = self.plot[i][j];

                    self.dfs((i, j), &mut visited, plant_type, &mut region);

                    regions.push(region);
                }
            }
        }
        regions
    }

    fn dfs(
        &self,
        pos: (usize, usize),
        visited: &mut HashSet<(usize, usize)>,
        plant_type: char,
        region: &mut Region,
    ) {
        let (row, col) = pos;

        visited.insert(pos);

        region.area += 1;
        region.crop.push(pos);

        let mut cell_perimeter = 0;

        for (dr, dc) in DIRECTIONS {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            // Check if the new position is out of bounds
            if new_row < 0
                || new_col < 0
                || new_row >= self.plot.len() as i32
                || new_col >= self.plot[0].len() as i32
            {
                cell_perimeter += 1;

                continue;
            }

            let new_row = new_row as usize;
            let new_col = new_col as usize;

            // If the adjacent cell is a different plant type, add to perimeter
            if self.plot[new_row][new_col] != plant_type {
                cell_perimeter += 1;
            }
            // If it's the same plant type and not visited, continue DFS
            else if !visited.contains(&(new_row, new_col)) {
                self.dfs((new_row, new_col), visited, plant_type, region);
            }
        }
        region.perimeter += cell_perimeter
    }
}

#[cfg(feature = "part_1")]
fn solve_part_1(input: &str) -> Result<String, Error> {
    let garden = Garden::new(input);

    let total_price = garden.fencing_price();

    Ok(total_price.to_string())
}

#[cfg(feature = "part_2")]
fn solve_part_2(input: &str) -> Result<String, Error> {
    let garden = Garden::new(input);

    let total_price = garden.fencing_price();

    Ok(total_price.to_string())
}

fn main() -> Result<(), Error> {
    println!("\nDay 12\n------");

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
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
    const SAMPLE_ANSWER_1: &str = "1930";

    assert_eq!(solve_part_1(SAMPLE_INPUT_1).unwrap(), SAMPLE_ANSWER_1);
}

#[cfg(feature = "part_2")]
#[test]
fn sample_part_2() {
    const SAMPLE_INPUT_2: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    const SAMPLE_ANSWER_2: &str = "1930";

    assert_eq!(solve_part_2(SAMPLE_INPUT_2).unwrap(), SAMPLE_ANSWER_2);
}
