use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::io::{Error, ErrorKind};

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 4!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 5...");

    let input_data = fs::read_to_string("input_data/day05_input.txt")?;
    let input_parts: Vec<&str> = input_data.split("\n\n").collect();
    let rules = input_parts[0];
    let updates = input_parts[1];

    let mut before_than: HashMap<u32, HashSet<u32>> = HashMap::new(); // for each page, which pages have to come later
    for rule in rules.lines() {
        let (before_str, after_str) = rule.split_once("|").expect("Cannot split rule!");

        let before = before_str.trim().parse::<u32>().map_err(|e| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Failed to parse first part: {}", e),
            )
        })?;
        let after = after_str.trim().parse::<u32>().map_err(|e| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Failed to parse first part: {}", e),
            )
        })?;

        if let Entry::Vacant(e) = before_than.entry(before) {
            e.insert(HashSet::from([after]));
        } else {
            before_than.get_mut(&before).unwrap().insert(after);
        }
    }

    let mut sum_of_middle_pages: u32 = 0;
    for update in updates.lines() {
        let pages = update
            .split(',')
            .map(|part| part.trim().parse::<u32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                Error::new(
                    ErrorKind::InvalidData,
                    format!("Failed to parse first part: {}", e),
                )
            })?;

        let mut valid = true;
        for (id, page) in pages.iter().enumerate() {
            for prev_page in &pages[..id] {
                print!("{} ", prev_page);
                if before_than[page].contains(prev_page) {
                    valid = false;
                    break;
                }
            }
            if !valid {
                break;
            }
        }

        if valid {
            sum_of_middle_pages += pages[pages.len() / 2];
        }
    }

    let mut solution_file = fs::File::create("solutions/day05_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 05:")?;
    writeln!(
        solution_file,
        "The sum of the middle page numbers of all valid updates is {}.",
        sum_of_middle_pages
    )?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 5...");

    Ok(())
}
