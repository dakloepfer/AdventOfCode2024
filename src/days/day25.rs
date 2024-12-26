use std::fs;
use std::io::Error;
use std::io::Write;

#[derive(Clone)]
struct Lock {
    heights: Vec<u8>,
}
#[derive(Clone)]
struct Key {
    heights: Vec<u8>,
}

fn check_lock_and_key(lock: Lock, key: Key, max_height: u8) -> bool {
    let mut can_fit = true;

    for (column, lock_height) in lock.heights.iter().enumerate() {
        if key.heights[column] + lock_height > max_height {
            can_fit = false;
            break;
        }
    }
    can_fit
}

fn check_all_locks_and_all_keys(locks: Vec<Lock>, keys: Vec<Key>, max_height: u8) -> u32 {
    let mut num_possible_pairs = 0;

    for lock in locks.iter() {
        for key in keys.iter() {
            if check_lock_and_key(lock.clone(), key.clone(), max_height) {
                num_possible_pairs += 1;
            }
        }
    }

    num_possible_pairs
}

fn parse_locks_and_keys(input_data: String) -> (Vec<Lock>, Vec<Key>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let schematics: Vec<&str> = input_data.split("\n\n").collect();

    for &schematic in schematics.iter() {
        let lines: Vec<&str> = schematic.lines().collect();

        let is_key = (*lines.first().unwrap())
            .chars()
            .collect::<Vec<char>>()
            .first()
            .unwrap()
            == &'.';

        let mut heights = vec![0; lines.first().unwrap().len()];
        for &line in &lines[1..lines.len() - 1] {
            for (idx, char) in line.char_indices() {
                if char == '#' {
                    heights[idx] += 1;
                }
            }
        }
        if is_key {
            keys.push(Key { heights });
        } else {
            locks.push(Lock { heights });
        }
    }

    (locks, keys)
}

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 25!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 25...");

    let input_data = fs::read_to_string("input_data/day25_input.txt")?;

    let (locks, keys) = parse_locks_and_keys(input_data);
    let num_possible_pairs = check_all_locks_and_all_keys(locks, keys, 5);

    let mut solution_file = fs::File::create("solutions/day25_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 25:")?;
    writeln!(
        solution_file,
        "There are {} lock/key pairs where no columns overlap.",
        num_possible_pairs
    )?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 25...");

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day25_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 25:")?;
    writeln!(
        solution_file,
        "The chronicle has been successfully completed."
    )?;

    Ok(())
}
