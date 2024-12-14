use std::fs;
use std::io::Error;
use std::io::Write;

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 2!");

    Ok(())
}

fn check_level_pair(level_a: i32, level_b: i32, increasing: bool) -> bool {
    let level_diff = level_b - level_a;

    if increasing {
        if level_diff <= 0 {
            return false;
        }
    } else {
        // decreasing
        if level_diff >= 0 {
            return false;
        }
    }
    if level_diff.abs() > 3 {
        return false;
    }
    true
}

fn check_safe(levels: Vec<i32>) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let mut safe = true;
    let increasing = levels[1] - levels[0] > 0;
    let mut prev_level = levels[0];
    for level in levels.iter().skip(1) {
        if !check_level_pair(prev_level, *level, increasing) {
            safe = false;
            break;
        }
        prev_level = *level;
    }
    safe
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 2...");

    let input_data = fs::read_to_string("input_data/day02_input.txt")?;

    let mut num_safe: u32 = 0;
    for report in input_data.lines() {
        let levels: Vec<i32> = report
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let safe = check_safe(levels);
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
    let input_data = fs::read_to_string("input_data/day02_input.txt")?;

    let mut num_safe: u32 = 0;
    for report in input_data.lines() {
        let levels: Vec<i32> = report
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let safe_without_first = check_safe(levels.iter().skip(1).cloned().collect());
        if safe_without_first {
            num_safe += 1;
            continue;
        }

        let safe_without_second = check_safe(
            levels[..1]
                .iter()
                .chain(levels[2..].iter())
                .cloned()
                .collect(),
        );
        if safe_without_second {
            num_safe += 1;
            continue;
        }

        // don't remove either first or second element
        let mut safe = true;
        let increasing = levels[1] - levels[0] > 0;
        let mut prev_idx = 0;
        let mut curr_idx = 1;
        let mut next_idx = 2;
        let mut skipped_level_already = false;
        while next_idx < levels.len() {
            let prev_level = levels[curr_idx];
            let current_level = levels[prev_idx];
            let next_level = levels[next_idx];

            if check_level_pair(current_level, next_level, increasing) {
                prev_idx = curr_idx;
                curr_idx = next_idx;
                next_idx += 1;
                continue;
            }

            if skipped_level_already {
                safe = false;
                break;
            }

            // try removing current level
            if check_level_pair(prev_level, next_level, increasing) {
                skipped_level_already = true;
                curr_idx = next_idx;
                next_idx += 1;
                continue;
            }

            // remove next level
            skipped_level_already = true;
            next_idx += 1;
        }
        if safe {
            num_safe += 1;
        }
    }

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day02_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 02:")?;
    writeln!(
        solution_file,
        "The total number of safe reports taking into account the problem dampener is {}.",
        num_safe
    )?;

    Ok(())
}
