use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::io::Write;

fn parse_stones(input_data: String) -> HashMap<u64, u64> {
    let stone_list: Vec<u64> = input_data
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
    let mut stones: HashMap<u64, u64> = HashMap::new();

    for &stone in stone_list.iter() {
        *stones.entry(stone).or_insert(0) += 1;
    }

    stones
}

fn update_stones(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::new();

    for (stone, num_occurrences) in stones {
        if stone == 0 {
            *new_stones.entry(1).or_insert(0) += num_occurrences;
        } else {
            let num_digits = stone.ilog(10) + 1;
            if num_digits % 2 == 0 {
                let basis = 10_u64.pow(num_digits / 2);
                *new_stones.entry(stone / basis).or_insert(0) += num_occurrences;
                *new_stones.entry(stone % basis).or_insert(0) += num_occurrences;
            } else {
                *new_stones.entry(stone * 2024).or_insert(0) += num_occurrences;
            }
        }
    }

    new_stones
}

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 11!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 11...");

    let input_data = fs::read_to_string("input_data/day11_input.txt")?;

    let mut stones = parse_stones(input_data);

    for _ in 0..25 {
        stones = update_stones(stones);
    }
    let mut num_stones: u64 = 0;
    for (_, num_occurrences) in stones {
        num_stones += num_occurrences;
    }

    let mut solution_file = fs::File::create("solutions/day11_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 11:")?;
    writeln!(
        solution_file,
        "After blinking 25 times, I have {} stones.",
        num_stones
    )?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 11...");

    let input_data = fs::read_to_string("input_data/day11_input.txt")?;

    let mut stones = parse_stones(input_data);

    for _ in 0..75 {
        stones = update_stones(stones);
    }
    let mut num_stones: u64 = 0;
    for (_, num_occurrences) in stones {
        num_stones += num_occurrences;
    }

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day11_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 11:")?;
    writeln!(
        solution_file,
        "After blinking 75 times, I have {} stones.",
        num_stones
    )?;

    Ok(())
}
