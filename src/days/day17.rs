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

struct Computer {
    // the instructions say that registers can be any integer, but if the initial values are all >=0, then they will stay so.
    // so since it makes other things easier (especially the 2^reg_a operation in dv and the XOR), I'll keep them as unsigned ints.
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    instruction_pointer: u32,
}
impl Computer {
    fn from_config(config: &str) -> Computer {
        let mut reg_a = 0;
        let mut reg_b = 0;
        let mut reg_c = 0;

        for line in config.trim().lines() {
            let (_, reg_val_str) = line.trim().split_once(": ").unwrap();
            let reg_val: u32 = reg_val_str.parse().unwrap();

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
    fn operand_to_combo(&self, operand: u32) -> u32 {
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
    fn compute_dv(&mut self, operand: u32) -> u32 {
        let numerator = self.reg_a;
        let denominator = 2u32.pow(self.operand_to_combo(operand));

        numerator / denominator
    }
    fn exec_adv(&mut self, operand: u32) {
        self.reg_a = self.compute_dv(operand);
    }
    fn exec_bxl(&mut self, operand: u32) {
        self.reg_b ^= operand;
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
    fn exec_out(&mut self, operand: u32) -> u32 {
        self.operand_to_combo(operand) % 8
    }
    fn exec_bdv(&mut self, operand: u32) {
        self.reg_b = self.compute_dv(operand);
    }
    fn exec_cdv(&mut self, operand: u32) {
        self.reg_c = self.compute_dv(operand);
    }

    fn execute_program(&mut self, program: Vec<u32>) -> Vec<u32> {
        let mut output: Vec<u32> = Vec::new();

        while self.instruction_pointer < program.len() as u32 - 1 {
            let instruction = program[self.instruction_pointer as usize];
            let operand = program[self.instruction_pointer as usize + 1];

            match instruction {
                0 => self.exec_adv(operand),
                1 => self.exec_bxl(operand),
                2 => self.exec_bst(operand),
                3 => {
                    let jumped = self.exec_jnz(operand);
                    if jumped {
                        continue;
                    }
                }
                4 => self.exec_bxc(operand),
                5 => output.push(self.exec_out(operand)),
                6 => self.exec_bdv(operand),
                7 => self.exec_cdv(operand),
                _ => panic!("Invalid instruction: {}", instruction),
            }
            self.instruction_pointer += 2;
        }

        output
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

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 17...");

    let solution = 0; // TODO

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day17_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 17:")?;
    writeln!(solution_file, "TODO {}.", solution)?;

    Ok(())
}
