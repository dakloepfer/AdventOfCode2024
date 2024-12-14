use std::fs;
use std::io::Error;
use std::io::Write;

fn update_stones(mut stones: Vec<u64>) -> Vec<u64> {
    let mut new_stones = Vec::new();

    for stone in stones.drain(..) {
        if stone == 0 {
            new_stones.push(1);
        } else {
            let num_digits = stone.ilog(10) + 1;
            if num_digits % 2 == 0 {
                let basis = 10_u64.pow(num_digits / 2);
                new_stones.push(stone / basis);
                new_stones.push(stone % basis);
            } else {
                new_stones.push(stone * 2024);
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

    let mut stones: Vec<u64> = input_data
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    for _ in 0..25 {
        stones = update_stones(stones);
    }

    let mut solution_file = fs::File::create("solutions/day11_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 11:")?;
    writeln!(
        solution_file,
        "After blinking 25 times, I have {} stones.",
        stones.len()
    )?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 11...");

    let solution = 0; // TODO

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day11_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 11:")?;
    writeln!(solution_file, "TODO {}.", solution)?;

    Ok(())
}
