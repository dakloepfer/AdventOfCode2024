use std::fs;
use std::io::Error;
use std::io::Write;

fn mix(number_a: u32, number_b: u32) -> u32 {
    number_a ^ number_b
}
fn prune(number_a: u32) -> u32 {
    number_a & 0xFFFFFF // 16777216 = 2^24
}
fn compute_next_number(number: u32) -> u32 {
    let mut next_number = number;

    // times 64
    next_number = mix(next_number, next_number << 6);
    next_number = prune(next_number);

    // divide 32
    next_number = mix(next_number, next_number >> 5);
    next_number = prune(next_number);

    // times 2048
    next_number = mix(next_number, next_number << 11);
    prune(next_number)
}

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 22!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 22...");

    let input_data = fs::read_to_string("input_data/day22_input.txt")?;

    let mut sum_final_secret_numbers = 0;
    for line in input_data.lines() {
        let mut secret_number = line.trim().parse::<u32>().ok().unwrap();

        for _ in 0..2000 {
            secret_number = compute_next_number(secret_number);
        }
        sum_final_secret_numbers += secret_number as u64;
    }

    let mut solution_file = fs::File::create("solutions/day22_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 22:")?;
    writeln!(
        solution_file,
        "The sum of all the secret numbers after 2000 steps is {}.",
        sum_final_secret_numbers
    )?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 22...");

    let solution = 0; // TODO

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day22_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 22:")?;
    writeln!(solution_file, "TODO {}.", solution)?;

    Ok(())
}
