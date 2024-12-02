use std::fs;
use std::io::Error;
use std::io::Write;

pub fn run() -> Result<(), Error> {
    let (list_a, list_b) = task1()?;
    let _ = task2(list_a, list_b);

    println!("Completed solutions for Day 1!");

    Ok(())
}

fn task1() -> Result<(Vec<i32>, Vec<i32>), Error> {
    println!("Computing solution for task 1 of Day 1...");

    let input_data = fs::read_to_string(
        "/Users/dominik.kloepfer/Documents/advent-of-code-2024/input_data/day01_input.txt",
    )?;

    let mut list_a: Vec<i32> = Vec::new();
    let mut list_b: Vec<i32> = Vec::new();

    for line in input_data.lines() {
        let list_elements: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        if list_elements.len() >= 2 {
            list_a.push(list_elements[0]);
            list_b.push(list_elements[1]);
        }
    }
    list_a.sort();
    list_b.sort();

    let mut distance: u32 = 0;
    for (item_a, item_b) in list_a.iter().zip(list_b.iter()) {
        distance += (item_a - item_b).unsigned_abs();
    }

    let mut solution_file = fs::File::create("solutions/day01_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 01:")?;
    writeln!(
        solution_file,
        "The total distance between the two lists is {}.",
        distance
    )?;

    Ok((list_a, list_b))
}

/// list_a and list_b are assumed to be sorted already.
fn task2(list_a: Vec<i32>, list_b: Vec<i32>) -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 1...");

    let mut num_appearances: Vec<u32> = vec![0; list_a.len()];

    let mut list_a_index: usize = 0;
    let mut list_b_index: usize = 0;
    while list_b_index < list_b.len() {
        let list_a_element = list_a[list_a_index];
        let list_b_element = list_b[list_b_index];

        match list_b_element {
            belem if belem < list_a_element => {
                list_b_index += 1;
            }
            belem if belem == list_a_element => {
                num_appearances[list_a_index] += 1;
                list_b_index += 1;
            }
            belem if belem > list_a_element => {
                if list_a_index < list_a.len() - 1 {
                    list_a_index += 1;
                } else {
                    break;
                }
                // copy num_appearances
                if list_a[list_a_index] == list_a_element {
                    num_appearances[list_a_index] = num_appearances[list_a_index - 1];
                }
            }
            _ => {
                eprintln!("What is going on? This is unexpected.")
            }
        }
    }

    let mut similarity_score: i32 = 0;
    for (list_a_element, num_apps) in list_a.iter().zip(num_appearances.iter()) {
        similarity_score += list_a_element * (*num_apps as i32);
    }

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day01_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 01:")?;
    writeln!(
        solution_file,
        "The total similarity score between the two lists is {}.",
        similarity_score
    )?;

    Ok(())
}
