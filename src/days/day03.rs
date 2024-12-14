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

    let input_data = fs::read_to_string("input_data/day03_input.txt")?;
    let mut program_cleaner = ProgramCleaner::new();

    let program_solution = program_cleaner.compute_sum_of_muls(input_data, false);

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

    let input_data = fs::read_to_string("input_data/day03_input.txt")?;
    let mut program_cleaner = ProgramCleaner::new();

    let program_solution = program_cleaner.compute_sum_of_muls(input_data, true);

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day03_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 03:")?;
    writeln!(
        solution_file,
        "The output of the cleaned program taking into account the conditionals is {}.",
        program_solution
    )?;
    Ok(())
}

#[derive(PartialEq)]
enum LookingFor {
    Mul,
    FirstNum,
    SecondNum,
    Do,
    Dont,
}

struct ProgramCleaner {
    running_sum: i32,
    current_state: LookingFor, // which type / part of command to look for
    current_mul_command: Vec<char>,
    current_first_number: Vec<char>,
    current_second_number: Vec<char>,
    current_conditional: Vec<char>,
    mul_enabled: bool,
}

impl ProgramCleaner {
    fn reset(&mut self) {
        self.current_state = LookingFor::Mul;
        self.current_mul_command = Vec::new();
        self.current_first_number = Vec::new();
        self.current_second_number = Vec::new();
        self.current_conditional = Vec::new();
    }

    /// Note: this adds to the existing value of running_sum
    pub fn compute_sum_of_muls(&mut self, program: String, enable_conditionals: bool) -> i32 {
        for el in program.chars() {
            if el == 'd' && enable_conditionals {
                self.reset();
                if self.mul_enabled {
                    self.current_state = LookingFor::Dont; // check if we get a disabler
                } else {
                    self.current_state = LookingFor::Do;
                }
            }
            if self.current_state == LookingFor::Mul || (el == 'm' && self.mul_enabled) {
                self.current_state = LookingFor::Mul; // in case we were in state 4 or 5

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
                    self.reset();
                }
                if self.current_mul_command.len() == 4 {
                    // completed mul command
                    self.current_state = LookingFor::FirstNum;
                }
            } else if self.current_state == LookingFor::FirstNum {
                if (self.current_first_number.is_empty() && el == '-') || el.is_numeric() {
                    self.current_first_number.push(el);
                } else if el == ',' {
                    self.current_state = LookingFor::SecondNum;
                } else {
                    self.reset();
                }
            } else if self.current_state == LookingFor::SecondNum {
                if (self.current_second_number.is_empty() && el == '-') || el.is_numeric() {
                    self.current_second_number.push(el);
                } else if el == ')' && self.mul_enabled {
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
                    self.reset();
                } else {
                    self.reset();
                }
            } else if self.current_state == LookingFor::Do {
                let expected_next_char: char = match self.current_conditional.len() {
                    0 => 'd',
                    1 => 'o',
                    2 => '(',
                    3 =>  ')',
                    _ => panic!("current_conditional has length longer than 3 in state 4, this shouldn't be possible"),
                };

                if el == expected_next_char {
                    self.current_conditional.push(el);
                } else {
                    self.reset();
                }
                if self.current_conditional.len() == 4 {
                    // completed do() command
                    self.mul_enabled = true;
                    self.reset();
                }
            } else if self.current_state == LookingFor::Dont {
                let expected_next_char: char = match self.current_conditional.len() {
                    0 => 'd',
                    1 => 'o',
                    2 => 'n',
                    3 =>  '\'',
                    4 =>  't',
                    5 =>  '(',
                    6 =>  ')',
                    _ => panic!("current_conditional has length longer than 6 in state 5, this shouldn't be possible"),
                };

                if el == expected_next_char {
                    self.current_conditional.push(el);
                } else {
                    self.reset();
                }
                if self.current_conditional.len() == 7 {
                    // completed don't() command
                    self.mul_enabled = false;
                    self.reset();
                }
            }
        }

        self.running_sum
    }

    pub fn new() -> ProgramCleaner {
        ProgramCleaner {
            running_sum: 0,
            current_state: LookingFor::Mul,
            current_mul_command: Vec::new(),
            current_first_number: Vec::new(),
            current_second_number: Vec::new(),
            current_conditional: Vec::new(),
            mul_enabled: true,
        }
    }
}
