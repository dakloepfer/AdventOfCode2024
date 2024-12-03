use std::fs;
use std::io::Error;
use std::io::Write;

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 3!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 3...");

    let input_data = fs::read_to_string(
        "/Users/dominik.kloepfer/Documents/advent-of-code-2024/input_data/day03_input.txt",
    )?;
    let mut program_cleaner = ProgramCleaner::new();

    let program_solution = program_cleaner.compute_sum_of_muls(input_data)?;

    let mut solution_file = fs::File::create("solutions/day03_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 03:")?;
    writeln!(
        solution_file,
        "The output of the cleaned program is {}.",
        program_solution
    )?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 3...");

    Ok(())
}

struct ProgramCleaner {
    running_sum: i32,
    current_state: u32, // 1 for looking for mul, 2 for first number, 3 for second number
    current_mul_command: Vec<char>,
    current_first_number: Vec<char>,
    current_second_number: Vec<char>,
}

impl ProgramCleaner {
    fn reset(&mut self) -> Result<(), Error> {
        self.current_state = 1;
        self.current_mul_command = Vec::new();
        self.current_first_number = Vec::new();
        self.current_second_number = Vec::new();
        Ok(())
    }

    /// Note: this adds to the existing value of running_sum
    pub fn compute_sum_of_muls(&mut self, program: String) -> Result<i32, Error> {
        for el in program.chars() {
            if self.current_state == 1 {
                let expected_next_char: char = match self.current_mul_command.len() {
                    0 => 'm',
                    1 => 'u',
                    2 => 'l',
                    3 =>  '(',
                    _ => panic!("current_mul_command has length longer than 3 in state 1, this shouldn't be possible"),
                };
                if el == expected_next_char {
                    self.current_mul_command.push(el);
                } else {
                    let _ = self.reset();
                }
                if self.current_mul_command.len() == 4 {
                    // completed mul command
                    self.current_state = 2;
                }
            } else if self.current_state == 2 {
                if (self.current_first_number.is_empty() && el == '-') || el.is_numeric() {
                    self.current_first_number.push(el);
                } else if el == ',' {
                    self.current_state = 3;
                } else {
                    let _ = self.reset();
                }
            } else if self.current_state == 3 {
                if (self.current_second_number.is_empty() && el == '-') || el.is_numeric() {
                    self.current_second_number.push(el);
                } else if el == ')' {
                    let first_number: i32 = self
                        .current_first_number
                        .iter()
                        .collect::<String>()
                        .parse::<i32>()
                        .expect("Failed to convert first number to i32");
                    let second_number: i32 = self
                        .current_second_number
                        .iter()
                        .collect::<String>()
                        .parse::<i32>()
                        .expect("Failed to convert second number to i32");
                    self.running_sum += first_number * second_number;
                    let _ = self.reset();
                } else {
                    let _ = self.reset();
                }
            }
        }

        Ok(self.running_sum)
    }

    pub fn new() -> ProgramCleaner {
        ProgramCleaner {
            running_sum: 0,
            current_state: 1,
            current_mul_command: Vec::new(),
            current_first_number: Vec::new(),
            current_second_number: Vec::new(),
        }
    }
}
