use anyhow::{Error, Result};
use std::collections::HashMap;

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_05.txt");

#[cfg(feature = "part_1")]
fn solve_part_1(input: &str) -> Result<String, Error> {
    let mut ordering_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut pages: Vec<Vec<i32>> = Vec::new();

    let mut break_line_flag = false;
    for line in input.lines() {
        if line.is_empty() {
            break_line_flag = true;
            continue;
        }
        if !break_line_flag {
            let parts: Vec<&str> = line.split('|').collect();
            let (from, to): (i32, i32) = (parts[0].parse().unwrap(), parts[1].parse().unwrap());

            ordering_rules.entry(from).or_default().push(to);
        } else {
            pages.push(line.split(',').map(|x| x.parse().unwrap()).collect());
        }
    }
    // add up the middle page number from those correctly-ordered updates
    let sum: i32 = pages
        .iter()
        .filter(|page| correct_order(page, &ordering_rules))
        .map(|page| get_middle_page_number(page))
        .sum();

    Ok(sum.to_string())
}

fn correct_order(page: &[i32], rules: &HashMap<i32, Vec<i32>>) -> bool {
    let mut index_map = HashMap::new();
    for (idx, &p) in page.iter().enumerate() {
        index_map.insert(p, idx);
    }

    for (&from, tos) in rules {
        if let Some(&from_idx) = index_map.get(&from) {
            for &to in tos {
                if let Some(&to_idx) = index_map.get(&to) {
                    if from_idx >= to_idx {
                        return false; // Ordering rule violated
                    }
                }
            }
        }
    }

    true
}
fn get_middle_page_number(page: &[i32]) -> i32 {
    page[page.len() / 2]
}

#[cfg(feature = "part_2")]
fn solve_part_2(input: &str) -> Result<String, Error> {
    let mut ordering_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut pages: Vec<Vec<i32>> = Vec::new();

    let mut break_line_flag = false;
    for line in input.lines() {
        if line.is_empty() {
            break_line_flag = true;
            continue;
        }
        if !break_line_flag {
            let parts: Vec<&str> = line.split('|').collect();
            let (from, to): (i32, i32) = (parts[0].parse().unwrap(), parts[1].parse().unwrap());

            ordering_rules.entry(from).or_default().push(to);
        } else {
            pages.push(line.split(',').map(|x| x.parse().unwrap()).collect());
        }
    }

    let sum: i32 = pages
        .iter()
        .filter(|page| !correct_order(page, &ordering_rules))
        .map(|page| order_page(page, &ordering_rules))
        .map(|page| get_middle_page_number(&page))
        .sum();

    Ok(sum.to_string())
}

fn order_page(page: &[i32], rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut page_ordered = page.to_vec();
    page_ordered.sort_by(|a, b| {
        if rules.get(a).map_or(false, |v| v.contains(b)) {
            std::cmp::Ordering::Less
        } else if rules.get(b).map_or(false, |v| v.contains(a)) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    page_ordered
}

fn main() -> Result<(), Error> {
    println!("\nDay 05\n------");

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
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    const SAMPLE_ANSWER_1: &str = "143";

    assert_eq!(solve_part_1(SAMPLE_INPUT_1).unwrap(), SAMPLE_ANSWER_1);
}

#[cfg(feature = "part_2")]
#[test]
fn sample_part_2() {
    const SAMPLE_INPUT_1: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    const SAMPLE_ANSWER_2: &str = "123";

    assert_eq!(solve_part_2(SAMPLE_INPUT_1).unwrap(), SAMPLE_ANSWER_2);
}
