use std::fs;
use std::io::Error;
use std::io::Write;

fn program_from_str(input: &str) -> Vec<u32> {
    let program_str;
    if input.starts_with('P') {
        (_, program_str) = input.trim().split_once(" ").unwrap();
    } else {
        program_str = input.trim();
    }

    program_str.split(',').flat_map(|val| val.parse()).collect()
}

#[derive(Copy, Clone)]
struct Computer {
    // the instructions say that registers can be any integer, but if the initial values are all >=0, then they will stay so.
    // so since it makes other things easier (especially the 2^reg_a operation in dv and the XOR), I'll keep them as unsigned ints.
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    instruction_pointer: u32,
}
impl Computer {
    fn from_config(config: &str) -> Computer {
        let mut reg_a = 0;
        let mut reg_b = 0;
        let mut reg_c = 0;

        for line in config.trim().lines() {
            let (_, reg_val_str) = line.trim().split_once(": ").unwrap();
            let reg_val: u64 = reg_val_str.parse().unwrap();

            if line.contains("A") {
                reg_a = reg_val;
            } else if line.contains("B") {
                reg_b = reg_val;
            } else if line.contains("C") {
                reg_c = reg_val;
            }
        }

        Computer {
            reg_a,
            reg_b,
            reg_c,
            instruction_pointer: 0,
        }
    }
    fn operand_to_combo(&self, operand: u32) -> u64 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => {
                panic!("Invalid program, found combo operand {}!", operand);
            }
        }
    }
    fn compute_dv(&mut self, operand: u32) -> u64 {
        self.reg_a >> self.operand_to_combo(operand)
    }
    fn exec_adv(&mut self, operand: u32) {
        self.reg_a = self.compute_dv(operand);
    }
    fn exec_bxl(&mut self, operand: u32) {
        self.reg_b ^= operand as u64;
    }
    fn exec_bst(&mut self, operand: u32) {
        self.reg_b = self.operand_to_combo(operand) % 8;
    }

    /// also returns whether it jumped
    fn exec_jnz(&mut self, operand: u32) -> bool {
        if self.reg_a != 0 {
            self.instruction_pointer = operand;
            true
        } else {
            false
        }
    }
    fn exec_bxc(&mut self, _operand: u32) {
        self.reg_b ^= self.reg_c;
    }
    fn exec_out(&mut self, operand: u32) -> u64 {
        self.operand_to_combo(operand) % 8
    }
    fn exec_bdv(&mut self, operand: u32) {
        self.reg_b = self.compute_dv(operand);
    }
    fn exec_cdv(&mut self, operand: u32) {
        self.reg_c = self.compute_dv(operand);
    }

    /// return whether we jumped
    fn execute_instruction(&mut self, instruction: u32, operand: u32) -> (bool, Option<u64>) {
        match instruction {
            0 => self.exec_adv(operand),
            1 => self.exec_bxl(operand),
            2 => self.exec_bst(operand),
            3 => return (self.exec_jnz(operand), None),
            4 => self.exec_bxc(operand),
            5 => return (false, Some(self.exec_out(operand))),
            6 => self.exec_bdv(operand),
            7 => self.exec_cdv(operand),
            _ => panic!("Invalid instruction: {}", instruction),
        }
        (false, None)
    }
    fn execute_program(&mut self, program: Vec<u32>) -> Vec<u64> {
        let mut output: Vec<u64> = Vec::new();

        while self.instruction_pointer < program.len() as u32 - 1 {
            let instruction = program[self.instruction_pointer as usize];
            let operand = program[self.instruction_pointer as usize + 1];

            let (jumped, output_maybe) = self.execute_instruction(instruction, operand);
            if let Some(out_val) = output_maybe {
                output.push(out_val);
            }
            if !jumped {
                self.instruction_pointer += 2;
            }
        }

        output
    }

    fn get_first_program_output(&mut self, program: Vec<u32>) -> u64 {
        while self.instruction_pointer < program.len() as u32 - 1 {
            let instruction = program[self.instruction_pointer as usize];
            let operand = program[self.instruction_pointer as usize + 1];

            let (jumped, output_maybe) = self.execute_instruction(instruction, operand);
            if let Some(out_val) = output_maybe {
                return out_val;
            }
            if !jumped {
                self.instruction_pointer += 2;
            }
        }
        0 // default value
    }

    fn _check_output_program_copy(&mut self, program: Vec<u32>) -> bool {
        let mut output: Vec<u64> = Vec::new();

        while self.instruction_pointer < program.len() as u32 - 1 {
            let instruction = program[self.instruction_pointer as usize];
            let operand = program[self.instruction_pointer as usize + 1];

            let (jumped, output_maybe) = self.execute_instruction(instruction, operand);
            if let Some(out_val) = output_maybe {
                if output.len() == program.len() || out_val != program[output.len()] as u64 {
                    return false;
                }
                output.push(out_val);
            }
            if !jumped {
                self.instruction_pointer += 2;
            }
        }

        output.len() == program.len()
    }
}

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 17!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 17...");

    let input_data = fs::read_to_string("input_data/day17_input.txt")?;

    let (computer_config, program_str) = input_data.split_once("\n\n").unwrap();
    let program = program_from_str(program_str);
    let mut computer = Computer::from_config(computer_config);

    let program_output = computer.execute_program(program);

    let output_str = program_output
        .iter()
        .map(|out| out.to_string())
        .collect::<Vec<_>>()
        .join(",");

    let mut solution_file = fs::File::create("solutions/day17_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 17:")?;
    writeln!(
        solution_file,
        "The output values of the program are {}.",
        output_str
    )?;

    Ok(())
}

fn _task2_general() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 17...");
    println!("NOTE / WARNING: This would work in theory (until overflow), but this takes way too long. The actual solution for the input needs to be obtained by analysing the program's actions and back-tracking the necessary register states.");

    let input_data = fs::read_to_string("input_data/day17_input.txt")?;

    let (computer_config, program_str) = input_data.split_once("\n\n").unwrap();
    let program = program_from_str(program_str);
    let computer = Computer::from_config(computer_config);

    let mut reg_a_start_value = 0;
    loop {
        let mut test_computer = computer;
        test_computer.reg_a = reg_a_start_value;

        if test_computer._check_output_program_copy(program.clone()) {
            break;
        }
        reg_a_start_value += 1;
    }

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day17_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 17:")?;
    writeln!(solution_file, "The smallest initial value of register A that causes the program to output a copy of itself is {}.", reg_a_start_value)?;

    Ok(())
}

fn start_value_vector_to_u64(start_value_vec: &[u32]) -> u64 {
    let mut start_value = 0;
    for val in start_value_vec.iter() {
        start_value += *val as u64;
        start_value <<= 3;
    }
    start_value >> 3
}
fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 17...");
    println!("This solution makes use of the fact that for the given input program, the program loops, right-shifting the value in register A by 3 bits each iteration and outputting a single value, until register A reaches 0.");

    let input_data = fs::read_to_string("input_data/day17_input.txt")?;

    let (computer_config, program_str) = input_data.split_once("\n\n").unwrap();
    let program = program_from_str(program_str);
    let computer = Computer::from_config(computer_config);

    let mut reg_a_start_value_vec: Vec<u32> = Vec::new();

    let mut initial_test_value = 0;
    while reg_a_start_value_vec.len() < program.len() {
        let mut success = false;
        for test_bits in initial_test_value..8 {
            reg_a_start_value_vec.push(test_bits);

            let mut test_computer = computer;
            test_computer.reg_a = start_value_vector_to_u64(&reg_a_start_value_vec);

            if test_computer.get_first_program_output(program.clone())
                == program[program.len() - reg_a_start_value_vec.len()] as u64
            {
                success = true;
                break;
            }

            reg_a_start_value_vec.pop();
        }
        if !success {
            // go back and continue looking
            initial_test_value = reg_a_start_value_vec.pop().unwrap() + 1;
        } else {
            initial_test_value = 0;
        }
    }

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day17_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 17:")?;
    writeln!(solution_file, "The smallest initial value of register A that causes the program to output a copy of itself is {}.", start_value_vector_to_u64(&reg_a_start_value_vec))?;

    Ok(())
}
