use std::fs;
use std::io::Error;
use std::io::Write;

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 20!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 20...");

    let input_data = fs::read_to_string("input_data/day20_input.txt")?;

    let solution = 0;

    let mut solution_file = fs::File::create("solutions/day20_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 20:")?;
    writeln!(solution_file, "TODO {}.", solution)?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 20...");

    let solution = 0; // TODO

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day20_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 20:")?;
    writeln!(solution_file, "TODO {}.", solution)?;

    Ok(())
}
