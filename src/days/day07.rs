use std::fs;
use std::io::Write;
use std::io::{Error, ErrorKind};

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 7!");

    Ok(())
}

fn check_equation(
    output_value: u64,
    numbers: Vec<u64>,
    operators: Vec<&str>,
) -> Result<bool, Error> {
    let mut valid = false;
    let mut stack: Vec<(usize, u64)> = vec![(1, *numbers.first().expect("numbers is empty!"))];

    while let Some((current_idx, current_result)) = stack.pop() {
        if (current_idx == numbers.len()) & (current_result == output_value) {
            valid = true;
            break;
        }
        if let Some(&current_number) = numbers.get(current_idx) {
            for operator in operators.iter() {
                let result: u64;
                if operator == &"add" {
                    result = current_result + current_number;
                } else if operator == &"mul" {
                    result = current_result * current_number;
                } else if operator == &"cat" {
                    result =
                        current_result * 10u64.pow(current_number.ilog10() + 1) + current_number
                } else {
                    eprintln!("Unknown operator {}!", operator);
                    return Err(Error::new(ErrorKind::InvalidInput, "Unknown Operator"));
                }
                if result <= output_value {
                    stack.push((current_idx + 1, result));
                }
            }
        }
    }

    Ok(valid)
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 7...");

    let input_data = fs::read_to_string("input_data/day07_input.txt")?;

    let mut sum_of_valid_test_values: u64 = 0;
    for line in input_data.lines() {
        let (test_value_str, numbers_str) = line.split_once(':').expect("Line has no ':'!");

        let test_value = test_value_str
            .parse::<u64>()
            .expect("could not parse test value.");

        let numbers: Vec<u64> = numbers_str
            .split_whitespace()
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();

        if check_equation(test_value, numbers, vec!["add", "mul"])? {
            sum_of_valid_test_values += test_value;
        }
    }

    let mut solution_file = fs::File::create("solutions/day07_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 07:")?;
    writeln!(
        solution_file,
        "The sum of the test values of equations that could be true with addition and multiplication is {}.",
        sum_of_valid_test_values
    )?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 7...");

    let input_data = fs::read_to_string("input_data/day07_input.txt")?;

    let mut sum_of_valid_test_values: u64 = 0;
    for line in input_data.lines() {
        let (test_value_str, numbers_str) = line.split_once(':').expect("Line has no ':'!");

        let test_value = test_value_str
            .parse::<u64>()
            .expect("could not parse test value.");

        let numbers: Vec<u64> = numbers_str
            .split_whitespace()
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();

        if check_equation(test_value, numbers, vec!["add", "mul", "cat"])? {
            sum_of_valid_test_values += test_value;
        }
    }

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day07_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 07:")?;
    writeln!(solution_file, "The sum of the test values of equations that could be true with addition, multiplication, and concatenation is {}.", sum_of_valid_test_values)?;

    Ok(())
}
