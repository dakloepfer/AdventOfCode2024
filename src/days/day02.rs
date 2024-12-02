use std::fs;
use std::io::Error;
use std::io::Write;

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 2!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 2...");

    let input_data = fs::read_to_string(
        "/Users/dominik.kloepfer/Documents/advent-of-code-2024/input_data/day02_input.txt",
    )?;

    let mut num_safe: u32 = 0;
    for report in input_data.lines() {
        let levels: Vec<i32> = report
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if levels.len() < 2 {
            num_safe += 1;
            continue;
        }

        let mut safe = true;
        let increasing = levels[1] - levels[0] > 0;
        let mut prev_level = levels[0];
        for level in levels.iter().skip(1) {
            let level_diff = level - prev_level;

            if increasing {
                if level_diff <= 0 {
                    safe = false;
                    break;
                }
            } else {
                // decreasing
                if level_diff >= 0 {
                    safe = false;
                    break;
                }
            }
            if level_diff.abs() > 3 {
                safe = false;
                break;
            }
            prev_level = *level;
        }
        if safe {
            num_safe += 1;
        }
    }

    let mut solution_file = fs::File::create("solutions/day02_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 02:")?;
    writeln!(
        solution_file,
        "The total number of safe reports is {}.",
        num_safe
    )?;

    Ok(())
}
fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 2...");
    Ok(())
}
