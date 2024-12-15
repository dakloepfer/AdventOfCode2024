use std::cmp::{max, min};
use std::fs;
use std::io::Error;
use std::io::Write;

struct Button {
    x: i32,
    y: i32,
    cost: i32,
}
struct Target {
    x: i32,
    y: i32,
}

/// If no solution is possible, return a cost of 0.
fn compute_lowest_price(button_a: Button, button_b: Button, target: Target) -> i32 {
    let determinant = button_a.x * button_b.y - button_a.y * button_b.x;

    let numerator_a = button_b.y * target.x - button_b.x * target.y;
    let numerator_b = -button_a.y * target.x + button_a.x * target.y;

    if determinant != 0 {
        if (numerator_a % determinant != 0)
            || (numerator_b % determinant != 0)
            || (numerator_a.signum() * determinant.signum() < 0)
            || (numerator_b.signum() * determinant.signum() < 0)
        {
            // no solution in the natural numbers
            0
        } else {
            let n_a = numerator_a / determinant;
            let n_b = numerator_b / determinant;
            button_a.cost * n_a + button_b.cost * n_b
        }
    } else {
        // determinant == 0 --> no solution or infinite solutions
        if (numerator_a != 0) || (numerator_b != 0) {
            // no solution
            0
        } else if (button_b.x == 0) && (button_b.y == 0) {
            if button_a.x != 0 {
                // solution is n_b = 0, n_a = target.x / button_a.x
                if (target.x % button_a.x != 0) || (target.x * button_a.x < 0) {
                    // no solution in the natural numbers
                    return 0;
                } else {
                    return button_a.cost * target.x / button_a.x;
                }
            } else if button_a.y != 0 {
                // solution is n_b = 0, n_a = target.y / button_a.y
                if (target.y % button_a.y != 0) || (target.y * button_a.y < 0) {
                    // no solution in the natural numbers
                    return 0;
                } else {
                    return button_a.cost * target.y / button_a.y;
                }
            } else {
                // everything is 0, solution is n_a = 0, n_b = 0
                return 0;
            }
        } else if (button_a.x == 0) && (button_a.y == 0) {
            if button_b.x != 0 {
                // solution is n_a = 0, n_b = target.x / button_b.x
                if (target.x % button_b.x != 0) || (target.x * button_b.x < 0) {
                    // no solution in the natural numbers
                    return 0;
                } else {
                    return button_b.cost * target.x / button_b.x;
                }
            } else if button_b.y != 0 {
                // solution is n_a = 0, n_b = target.y / button_b.y
                if (target.y % button_b.y != 0) || (target.y * button_b.y < 0) {
                    // no solution in the natural numbers
                    return 0;
                } else {
                    return button_b.cost * target.y / button_b.y;
                }
            } else {
                // everything is 0, solution is n_a = 0, n_b = 0
                return 0;
            }
        } else if ((button_b.x > 0) && (button_a.x > 0)) || ((button_b.x < 0) && (button_a.x < 0)) {
            let max_n_a = min(target.x / button_a.x, 100);
            if max_n_a < 0 {
                // no solution in natural numbers
                return 0;
            }
            let mut min_cost = i32::MAX;
            for n_a in 0..=max_n_a {
                if (target.x - button_a.x * n_a) % button_b.x == 0 {
                    let cost = button_a.cost * n_a
                        + button_b.cost * (target.x - button_a.x * n_a) / button_b.x;
                    if cost < min_cost {
                        min_cost = cost;
                    }
                }
            }
            return min_cost;
        } else if ((button_b.x > 0) && (button_a.x < 0)) || ((button_b.x < 0) && (button_a.x > 0)) {
            let min_n_a = max(target.x / button_a.x, 0);

            let mut min_cost = i32::MAX;
            for n_a in min_n_a..=100 {
                if (target.x - button_a.x * n_a) % button_b.x == 0 {
                    let cost = button_a.cost * n_a
                        + button_b.cost * (target.x - button_a.x * n_a) / button_b.x;
                    if cost < min_cost {
                        min_cost = cost;
                    }
                }
            }
            return min_cost;
        } else if ((button_b.y > 0) && (button_a.y > 0)) || ((button_b.y < 0) && (button_a.y < 0)) {
            let max_n_a = min(target.y / button_a.y, 100);
            if max_n_a < 0 {
                // no solution in natural numbers
                return 0;
            }
            let mut min_cost = i32::MAX;
            for n_a in 0..=max_n_a {
                if (target.y - button_a.y * n_a) % button_b.y == 0 {
                    let cost = button_a.cost * n_a
                        + button_b.cost * (target.y - button_a.y * n_a) / button_b.y;
                    if cost < min_cost {
                        min_cost = cost;
                    }
                }
            }
            return min_cost;
        } else if ((button_b.y > 0) && (button_a.y < 0)) || ((button_b.y < 0) && (button_a.y > 0)) {
            let min_n_a = max(target.y / button_a.y, 0);

            let mut min_cost = i32::MAX;
            for n_a in min_n_a..=100 {
                if (target.y - button_a.y * n_a) % button_b.y == 0 {
                    let cost = button_a.cost * n_a
                        + button_b.cost * (target.y - button_a.y * n_a) / button_b.y;
                    if cost < min_cost {
                        min_cost = cost;
                    }
                }
            }
            return min_cost;
        } else {
            eprintln!("No previous possibility happened, this should be impossible!");
            return 0;
        }
    }
}

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 13!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 13...");

    let input_data = fs::read_to_string("input_data/day13_input.txt")?;

    let mut claw_machine_configs: Vec<Vec<&str>> = Vec::new();
    let mut current_claw_machine_config: Vec<&str> = Vec::new();

    for line in input_data.lines() {
        if line.trim().is_empty() && (!current_claw_machine_config.is_empty()) {
            claw_machine_configs.push(current_claw_machine_config.clone());
            current_claw_machine_config.clear();
        } else {
            current_claw_machine_config.push(line);
        }
    }
    if !current_claw_machine_config.is_empty() {
        claw_machine_configs.push(current_claw_machine_config);
    }

    let mut tokens_needed = 0;
    for claw_machine_config in claw_machine_configs {
        let mut button_a = Button {
            x: 0,
            y: 0,
            cost: 0,
        };
        let mut button_b = Button {
            x: 0,
            y: 0,
            cost: 0,
        };
        let mut target = Target { x: 0, y: 0 };
        for line in claw_machine_config {
            if line.starts_with("Button A:") {
                let x_start = line.find("X").unwrap() + 1;
                let x_end = line.find(",").unwrap();
                let x: i32 = line[x_start..x_end].trim().parse().ok().unwrap();

                let y_start = line.find("Y").unwrap() + 1;
                let y: i32 = line[y_start..].trim().parse().ok().unwrap();

                button_a = Button { x, y, cost: 3 };
            } else if line.starts_with("Button B:") {
                let x_start = line.find("X").unwrap() + 1;
                let x_end = line.find(",").unwrap();
                let x: i32 = line[x_start..x_end].trim().parse().ok().unwrap();

                let y_start = line.find("Y").unwrap() + 1;
                let y: i32 = line[y_start..].trim().parse().ok().unwrap();

                button_b = Button { x, y, cost: 1 };
            } else if line.starts_with("Prize:") {
                let x_start = line.find("X").unwrap() + 2;
                let x_end = line.find(",").unwrap();
                let x: i32 = line[x_start..x_end].trim().parse().ok().unwrap();

                let y_start = line.find("Y").unwrap() + 2;
                let y: i32 = line[y_start..].trim().parse().ok().unwrap();

                target = Target { x, y };
            }
        }

        tokens_needed += compute_lowest_price(button_a, button_b, target);
    }

    let mut solution_file = fs::File::create("solutions/day13_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 13:")?;
    writeln!(
        solution_file,
        "{} tokens are needed to obtain all obtainable prizes.",
        tokens_needed
    )?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 13...");

    let solution = 0; // TODO

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day13_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 13:")?;
    writeln!(solution_file, "TODO {}.", solution)?;

    Ok(())
}
