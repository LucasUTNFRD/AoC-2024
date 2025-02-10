use anyhow::{Error, Result};

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_09.txt");

//NOTE:
//The disk map uses a dense format to represent the layout of files and free space
//on the disk.
//The digits alternate between indicating the length of a file and the length of free space.

fn solve_part_1(diskmap: &str) -> Result<String, Error> {
    let mut extended_layout: Vec<i32> = Vec::new();
    let mut id_number = 0;
    for (i, c) in diskmap.chars().enumerate() {
        if c == '\n' {
            continue;
        }
        let file = c.to_digit(10).unwrap() as i32;
        if i % 2 == 0 {
            extended_layout.extend(std::iter::repeat(id_number).take(file as usize));
            id_number += 1;
        } else {
            extended_layout.extend(std::iter::repeat(-1).take(file as usize));
        }
    }

    let mut left = 0;
    let mut right = extended_layout.len() - 1;

    while left < right {
        while left < right && extended_layout[left] != -1 {
            left += 1;
        }
        while left < right && extended_layout[right] == -1 {
            right -= 1;
        }
        // swap arr[i] to arr[j]
        if left < right {
            extended_layout.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    // println!("Compacted layout: {:?}", extended_layout);

    let mut output: u64 = 0;
    for (i, &id) in extended_layout.iter().enumerate() {
        if id < 0 {
            continue;
        }
        let product = i as u64 * id as u64;
        // dbg!(i, id, product);
        output += product;
    }

    Ok(output.to_string())
}

#[cfg(feature = "part_2")]
fn solve_part_2(diskmap: &str) -> Result<String, Error> {
    let mut extended_layout: Vec<i32> = Vec::new();
    let mut id_number = 0;
    for (i, c) in diskmap.chars().enumerate() {
        if c == '\n' {
            continue;
        }
        let file = c.to_digit(10).unwrap() as i32;
        if i % 2 == 0 {
            for _ in 0..file {
                extended_layout.push(id_number);
            }
            id_number += 1;
        } else {
            for _ in 0..file {
                extended_layout.push(-1);
            }
        }
    }

    let mut right = extended_layout.len();
    while right > 0 {
        right -= 1; // Move to a file

        if extended_layout[right] != -1 {
            let file_id = extended_layout[right];

            // Compute file size correctly
            let mut file_size = 1;
            while right > 0 && extended_layout[right - 1] == file_id {
                right -= 1;
                file_size += 1;
            }

            let mut left = 0;
            // Find first suitable free space
            while left < right {
                if extended_layout[left] == -1 {
                    let mut free_space = 0;
                    let mut tmp_right = left;

                    while tmp_right < extended_layout.len() && extended_layout[tmp_right] == -1 {
                        free_space += 1;
                        tmp_right += 1;
                    }

                    // Move file if space is available
                    if free_space >= file_size {
                        // Clear old position and move in one pass
                        for i in 0..file_size {
                            extended_layout[left + i] = file_id;
                            extended_layout[right + i] = -1;
                        }

                        // println!("Layout after moving: {:?}", extended_layout);
                        break; // Stop searching after moving
                    }
                }
                left += 1;
            }
        }
    }

    let mut output: u64 = 0;
    for (i, &id) in extended_layout.iter().enumerate() {
        if id < 0 {
            continue;
        }
        let product = i as u64 * id as u64;
        // dbg!(i, id, product);
        output += product;
    }

    Ok(output.to_string())
}

fn main() -> Result<(), Error> {
    println!("\nDay 09\n------");

    #[cfg(feature = "part_1")]
    {
        let start = std::time::Instant::now();
        let answer_part_1 = solve_part_1(PUZZLE_INPUT)?;
        let duration = start.elapsed();
        println!("Part One: {answer_part_1}");
        println!("Time: {:?}", duration);
    }

    #[cfg(feature = "part_2")]
    {
        let start = std::time::Instant::now();
        let answer_part_2 = solve_part_2(PUZZLE_INPUT)?;
        let duration = start.elapsed();
        println!("Part Two: {answer_part_2}");
        println!("Time: {:?}", duration);
    }

    println!();

    Ok(())
}
// fn main() -> Result<(), Error> {
//     println!("\nDay 09\n------");

//     #[cfg(feature = "part_1")]
//     {
//         let answer_part_1 = solve_part_1(PUZZLE_INPUT)?;
//         println!("Part One: {answer_part_1}");
//     }

//     #[cfg(feature = "part_2")]
//     {
//         let answer_part_2 = solve_part_2(PUZZLE_INPUT)?;
//         println!("Part Two: {answer_part_2}");
//     }

//     println!();

//     Ok(())
// }

#[cfg(feature = "part_1")]
#[test]
fn sample_part_1() {
    const SAMPLE_INPUT_1: &str = "\
2333133121414131402
";
    const SAMPLE_ANSWER_1: &str = "1928";

    assert_eq!(solve_part_1(SAMPLE_INPUT_1).unwrap(), SAMPLE_ANSWER_1);
}

#[cfg(feature = "part_2")]
#[test]
fn sample_part_2() {
    const SAMPLE_INPUT_2: &str = "\
2333133121414131402
";
    const SAMPLE_ANSWER_2: &str = "2858";

    assert_eq!(solve_part_2(SAMPLE_INPUT_2).unwrap(), SAMPLE_ANSWER_2);
}
