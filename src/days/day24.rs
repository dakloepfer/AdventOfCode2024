use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::io::Write;

#[derive(Clone, Copy)]
enum GateType {
    Input,
    And,
    Or,
    Xor,
}

#[derive(Clone)]
struct Gate {
    gate_type: GateType,
    in_vals: Vec<bool>,
    out_val: bool,
}
impl Gate {
    fn able_to_evaluate(&self) -> bool {
        match self.gate_type {
            GateType::Input => !self.in_vals.is_empty(),
            _ => self.in_vals.len() >= 2,
        }
    }
    fn evaluate(&mut self) -> bool {
        match self.gate_type {
            GateType::Input => {
                self.out_val = *self.in_vals.first().unwrap();
            }
            GateType::And => {
                self.out_val = self.in_vals[0] & self.in_vals[1];
            }
            GateType::Or => {
                self.out_val = self.in_vals[0] | self.in_vals[1];
            }
            GateType::Xor => {
                self.out_val = self.in_vals[0] ^ self.in_vals[1];
            }
        }
        self.out_val
    }
}

struct Network {
    output_gates: Vec<String>,

    gates: HashMap<String, Gate>,
    successors: HashMap<String, Vec<String>>,
}
impl Network {
    fn from_gate_configs_and_input_vals(input_vals: String, gate_configs: String) -> Network {
        let mut input_gates = Vec::new();
        let mut output_gates: Vec<String> = Vec::new();
        let mut gates = HashMap::new();
        let mut successors: HashMap<String, Vec<String>> = HashMap::new();

        for input_str in input_vals.lines() {
            let (gate_name, input_val_string) = input_str.trim().split_once(": ").unwrap();
            let input_val = input_val_string == "1";
            gates.insert(
                gate_name.to_string(),
                Gate {
                    gate_type: GateType::Input,
                    in_vals: vec![input_val],
                    out_val: input_val,
                },
            );
            input_gates.push(gate_name.to_string());
        }

        for gate_config in gate_configs.lines() {
            let data: Vec<&str> = gate_config.split(" ").collect();

            let gate_input1 = data[0].to_string();
            let gate_input2 = data[2].to_string();
            let gate_type = match data[1] {
                "OR" => GateType::Or,
                "AND" => GateType::And,
                "XOR" => GateType::Xor,
                _ => unreachable!(),
            };
            let gate_name: String = data.last().unwrap().to_string();

            if gate_name.starts_with("z") {
                output_gates.push(gate_name.clone());
                successors.insert(gate_name.clone(), Vec::new());
            }
            successors
                .entry(gate_input1)
                .or_default()
                .push(gate_name.clone());
            successors
                .entry(gate_input2)
                .or_default()
                .push(gate_name.clone());

            gates.insert(
                gate_name,
                Gate {
                    gate_type,
                    in_vals: Vec::new(),
                    out_val: false,
                },
            );
        }
        output_gates.sort();

        Network {
            output_gates,
            gates,
            successors,
        }
    }

    fn evaluate(&mut self) {
        let mut gates_to_evaluate: Vec<String> = Vec::new();

        for (gate_name, gate) in &self.gates {
            if gate.able_to_evaluate() {
                gates_to_evaluate.push(gate_name.to_string());
            }
        }

        while let Some(current_gate_name) = gates_to_evaluate.pop() {
            let mut current_gate = self.gates.get(&current_gate_name).unwrap().clone();
            let current_output = current_gate.evaluate();

            for successor_gate_name in self.successors.get(&current_gate_name).unwrap().iter() {
                let mut successor_gate = self.gates.get(successor_gate_name).unwrap().clone();
                successor_gate.in_vals.push(current_output);
                if successor_gate.able_to_evaluate() {
                    gates_to_evaluate.push(successor_gate_name.clone());
                }
                self.gates
                    .insert(successor_gate_name.clone(), successor_gate);
            }
            self.gates.insert(current_gate_name, current_gate);
        }
    }

    fn get_output_number(&self) -> u64 {
        let mut output_number = 0;
        for (exponent, output_name) in self.output_gates.iter().enumerate() {
            if self.gates.get(output_name).unwrap().out_val {
                output_number += 2u64.pow(exponent as u32);
            }
        }
        output_number
    }
}

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 24!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 24...");

    let input_data = fs::read_to_string("input_data/day24_input.txt")?;

    let (input_vals, gate_configs) = input_data.split_once("\n\n").unwrap();
    let mut network =
        Network::from_gate_configs_and_input_vals(input_vals.to_string(), gate_configs.to_string());

    network.evaluate();
    let output_number = network.get_output_number();

    let mut solution_file = fs::File::create("solutions/day24_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 24:")?;
    writeln!(
        solution_file,
        "The network produces the output number {}.",
        output_number
    )?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 24...");

    let solution = 0; // TODO

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day24_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 24:")?;
    writeln!(solution_file, "TODO {}.", solution)?;

    Ok(())
}
