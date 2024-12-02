use std::fs;
use std::io::Error;
use std::io::Write;

pub fn run() {
    let _ = task1();
    task2();

    println!("Completed solutions for Day 1!");
}

fn task1() -> Result<(), Error> {
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

    let mut solution_file = fs::File::create("solutions/day01_task1_solution.txt")?;
    writeln!(
        solution_file,
        "The total distance between the two lists is {}",
        distance
    )?;

    Ok(())
}

fn task2() {
    println!("Computing solution for task 2 of Day 1...")
}
