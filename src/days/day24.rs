use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Error;
use std::io::Write;

fn sorted(mut vec: Vec<String>) -> Vec<String> {
    vec.sort();
    vec
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum GateType {
    Input,
    And,
    Or,
    Xor,
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct GateConfig {
    gate_type: GateType,
    in_gates: Vec<String>,
}

#[derive(Clone, Hash)]
struct Gate {
    config: GateConfig,
    in_vals: Vec<bool>,
    out_val: bool,
}
impl Gate {
    fn able_to_evaluate(&self) -> bool {
        self.in_vals.len() == self.config.in_gates.len()
    }
    fn evaluate(&mut self) -> bool {
        match self.config.gate_type {
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
    input_gates: Vec<String>,
    gates: HashMap<String, Gate>,
    gateconfig_to_name: HashMap<GateConfig, String>,
    successors: HashMap<String, HashSet<String>>,
}
impl Network {
    fn from_gate_configs_and_input_vals(input_vals: String, gate_configs: String) -> Network {
        let mut input_gates = Vec::new();
        let mut output_gates: Vec<String> = Vec::new();
        let mut gates: HashMap<String, Gate> = HashMap::new();
        let mut successors: HashMap<String, HashSet<String>> = HashMap::new();
        let mut gateconfig_to_name = HashMap::new();

        for input_str in input_vals.lines() {
            let (gate_name, input_val_string) = input_str.trim().split_once(": ").unwrap();
            let input_val = input_val_string == "1";
            let gate_config = GateConfig {
                gate_type: GateType::Input,
                in_gates: vec!["Input".to_string()],
            };
            gateconfig_to_name.insert(gate_config.clone(), gate_name.to_string());
            gates.insert(
                gate_name.to_string(),
                Gate {
                    config: gate_config,
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
                successors.insert(gate_name.clone(), HashSet::new());
            }
            successors
                .entry(gate_input1.clone())
                .or_default()
                .insert(gate_name.clone());
            successors
                .entry(gate_input2.clone())
                .or_default()
                .insert(gate_name.clone());

            let gate_config = GateConfig {
                gate_type,
                in_gates: sorted(vec![gate_input1.clone(), gate_input2.clone()]),
            };
            gateconfig_to_name.insert(gate_config.clone(), gate_name.clone());
            gates.insert(
                gate_name,
                Gate {
                    config: gate_config,
                    in_vals: Vec::new(),
                    out_val: false,
                },
            );
        }
        input_gates.sort();
        output_gates.sort();

        Network {
            input_gates,
            output_gates,
            gates,
            successors,
            gateconfig_to_name,
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

    fn swap_outputs(&mut self, gate_a_name: String, gate_b_name: String) {
        let gate_a = self.gates.get(&gate_a_name).unwrap().clone();
        let gate_b = self.gates.get(&gate_b_name).unwrap().clone();

        self.gates.insert(gate_a_name.clone(), gate_b.clone());
        self.gates.insert(gate_b_name.clone(), gate_a.clone());

        self.gateconfig_to_name
            .insert(gate_a.config.clone(), gate_b_name.clone());
        self.gateconfig_to_name
            .insert(gate_b.config.clone(), gate_a_name.clone());

        for predecessor_gate in gate_a.config.in_gates.iter() {
            self.successors
                .entry(predecessor_gate.clone())
                .or_default()
                .remove(&gate_a_name.clone());
            self.successors
                .entry(predecessor_gate.clone())
                .or_default()
                .insert(gate_b_name.clone());
        }
        for predecessor_gate in gate_b.config.in_gates.iter() {
            self.successors
                .entry(predecessor_gate.clone())
                .or_default()
                .remove(&gate_b_name.clone());
            self.successors
                .entry(predecessor_gate.clone())
                .or_default()
                .insert(gate_a_name.clone());
        }
    }

    fn debug_addition(&mut self) -> Vec<String> {
        let mut swapped_gates = Vec::new();

        let num_digits = self.input_gates.len() / 2;
        let mut carry = "".to_string();
        for digit in 0..num_digits {
            let xinput_name = format!("x{:02}", digit);
            let yinput_name = format!("y{:02}", digit);
            let output_name = format!("z{:02}", digit);

            // these two have to exist if all that happened is some swapped output wires
            let xxory_gate = self
                .gateconfig_to_name
                .get(&GateConfig {
                    in_gates: vec![xinput_name.clone(), yinput_name.clone()],
                    gate_type: GateType::Xor,
                })
                .unwrap()
                .clone();
            let xandy_gate = self
                .gateconfig_to_name
                .get(&GateConfig {
                    in_gates: vec![xinput_name.clone(), yinput_name.clone()],
                    gate_type: GateType::And,
                })
                .unwrap()
                .clone();

            if carry.is_empty() {
                // first digit
                if xxory_gate != output_name {
                    swapped_gates.push(xxory_gate.clone());
                    swapped_gates.push(output_name.clone());
                    self.swap_outputs(xxory_gate.clone(), output_name.clone());
                }
                carry = xandy_gate.clone();
            } else {
                let output_predecessors = sorted(vec![carry.clone(), xxory_gate.clone()]);

                match self.gateconfig_to_name.get(&GateConfig {
                    in_gates: output_predecessors.clone(),
                    gate_type: GateType::Xor,
                }) {
                    None => {
                        // one of the carry or xxory gate outputs was swapped
                        let actual_output_predecessors: HashSet<String> = self
                            .gates
                            .get(&output_name.clone())
                            .unwrap()
                            .clone()
                            .config
                            .in_gates
                            .into_iter()
                            .collect();
                        let desired_output_predecessors: HashSet<String> =
                            output_predecessors.into_iter().collect();
                        let swapped_outputs: Vec<String> = actual_output_predecessors
                            .symmetric_difference(&desired_output_predecessors)
                            .cloned()
                            .collect();
                        swapped_gates.push(swapped_outputs[0].clone());
                        swapped_gates.push(swapped_outputs[1].clone());
                        self.swap_outputs(swapped_outputs[0].clone(), swapped_outputs[1].clone());
                    }
                    Some(maybe_output_gate) => {
                        if *maybe_output_gate != output_name {
                            // output gate itself is swapped
                            swapped_gates.push(maybe_output_gate.clone());
                            swapped_gates.push(output_name.clone());
                            self.swap_outputs(maybe_output_gate.clone(), output_name.clone());
                        }
                    }
                }

                // update carry
                // need to update after potentially swapping
                let new_xxory_gate = self
                    .gateconfig_to_name
                    .get(&GateConfig {
                        in_gates: vec![xinput_name.clone(), yinput_name.clone()],
                        gate_type: GateType::Xor,
                    })
                    .unwrap()
                    .clone();
                let new_xandy_gate = self
                    .gateconfig_to_name
                    .get(&GateConfig {
                        in_gates: vec![xinput_name.clone(), yinput_name.clone()],
                        gate_type: GateType::And,
                    })
                    .unwrap()
                    .clone();

                carry = self
                    .gateconfig_to_name
                    .get(&GateConfig {
                        in_gates: sorted(vec![carry.clone(), new_xxory_gate.clone()]),
                        gate_type: GateType::And,
                    })
                    .unwrap()
                    .clone();
                carry = self
                    .gateconfig_to_name
                    .get(&GateConfig {
                        in_gates: sorted(vec![carry.clone(), new_xandy_gate.clone()]),
                        gate_type: GateType::Or,
                    })
                    .unwrap()
                    .clone();
            }
        }

        swapped_gates.sort();
        swapped_gates
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

    let input_data = fs::read_to_string("input_data/day24_input.txt")?;

    let (input_vals, gate_configs) = input_data.split_once("\n\n").unwrap();
    let mut network =
        Network::from_gate_configs_and_input_vals(input_vals.to_string(), gate_configs.to_string());

    let swapped_gates = network.debug_addition();

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day24_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 24:")?;
    writeln!(
        solution_file,
        "The output wires that were swapped are {}.",
        swapped_gates.join(",")
    )?;

    Ok(())
}
