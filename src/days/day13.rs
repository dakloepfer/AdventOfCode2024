use std::fs;
use std::io::Error;
use std::io::Write;

struct Button {
    x: i64,
    y: i64,
    cost: i64,
}
impl Button {
    fn new() -> Button {
        Button {
            x: 0,
            y: 0,
            cost: 0,
        }
    }
}
struct Target {
    x: i64,
    y: i64,
}
impl Target {
    fn new() -> Target {
        Target { x: 0, y: 0 }
    }
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    // We'll work with local mutable copies
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1i64, 0i64);
    let (mut old_t, mut t) = (0i64, 1i64);

    while r != 0 {
        let quotient = old_r / r;
        // perform the Euclidean step
        let temp_r = old_r - quotient * r;
        old_r = r;
        r = temp_r;

        let temp_s = old_s - quotient * s;
        old_s = s;
        s = temp_s;

        let temp_t = old_t - quotient * t;
        old_t = t;
        t = temp_t;
    }

    // Now old_r is the gcd, and (old_s, old_t) are the coefficients
    (old_r, old_s, old_t)
}

/// If no solution is possible, return a cost of 0.
fn compute_lowest_price(button_a: Button, button_b: Button, target: Target) -> i64 {
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
        } else if button_a.x != 0 {
            let (gcd, u, v) = extended_gcd(button_a.x, button_b.x);
            if target.x % gcd != 0 {
                // no integer solutions, so no solutions
                return 0;
            }
            let particular_n_a = u * target.x / gcd;
            let particular_n_b = v * target.x / gcd;
            let reduced_a_x = button_a.x / gcd;
            let reduced_b_x = button_b.x / gcd;

            let t_min = -particular_n_a / reduced_b_x;
            let t_max = particular_n_b / reduced_a_x;
            if t_min > t_max {
                // no nonnegative solutions
                return 0;
            }
            let mut min_cost = i64::MAX;
            for t in t_min..=t_max {
                let n_a = particular_n_a + t * reduced_b_x;
                let n_b = particular_n_b - t * reduced_a_x;
                let cost = button_a.cost * n_a + button_b.cost * n_b;
                if (n_a >= 0) && (n_b >= 0) && (cost < min_cost) {
                    min_cost = cost;
                }
            }
            return min_cost;
        } else {
            // button_a.y != 0 must be the case
            let (gcd, u, v) = extended_gcd(button_a.y, button_b.y);
            if target.y % gcd != 0 {
                // no integer solutions, so no solutions
                return 0;
            }
            let particular_n_a = u * target.y / gcd;
            let particular_n_b = v * target.y / gcd;
            let reduced_a_y = button_a.y / gcd;
            let reduced_b_y = button_b.y / gcd;

            let t_min = -particular_n_a / reduced_b_y;
            let t_max = particular_n_b / reduced_a_y;
            if t_min > t_max {
                // no nonnegative solutions
                return 0;
            }
            let mut min_cost = i64::MAX;
            for t in t_min..=t_max {
                let n_a = particular_n_a + t * reduced_b_y;
                let n_b = particular_n_b - t * reduced_a_y;
                let cost = button_a.cost * n_a + button_b.cost * n_b;
                if (n_a >= 0) && (n_b >= 0) && (cost < min_cost) {
                    min_cost = cost;
                }
            }
            return min_cost;
        }
    }
}

fn parse_button_line(line: &str) -> (i64, i64) {
    let x_start = line.find("X").unwrap() + 1;
    let x_end = line.find(",").unwrap();
    let x: i64 = line[x_start..x_end].trim().parse().ok().unwrap();

    let y_start = line.find("Y").unwrap() + 1;
    let y: i64 = line[y_start..].trim().parse().ok().unwrap();

    (x, y)
}
fn parse_target_line(line: &str) -> (i64, i64) {
    let x_start = line.find("X").unwrap() + 2;
    let x_end = line.find(",").unwrap();
    let x: i64 = line[x_start..x_end].trim().parse().ok().unwrap();

    let y_start = line.find("Y").unwrap() + 2;
    let y: i64 = line[y_start..].trim().parse().ok().unwrap();

    (x, y)
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
        let mut button_a = Button::new();
        let mut button_b = Button::new();
        let mut target = Target::new();

        for line in claw_machine_config {
            if line.starts_with("Button A:") {
                let (x, y) = parse_button_line(line);
                button_a = Button { x, y, cost: 3 };
            } else if line.starts_with("Button B:") {
                let (x, y) = parse_button_line(line);
                button_b = Button { x, y, cost: 1 };
            } else if line.starts_with("Prize:") {
                let (x, y) = parse_target_line(line);
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
        let mut button_a = Button::new();
        let mut button_b = Button::new();
        let mut target = Target::new();

        for line in claw_machine_config {
            if line.starts_with("Button A:") {
                let (x, y) = parse_button_line(line);
                button_a = Button { x, y, cost: 3 };
            } else if line.starts_with("Button B:") {
                let (x, y) = parse_button_line(line);
                button_b = Button { x, y, cost: 1 };
            } else if line.starts_with("Prize:") {
                let (x, y) = parse_target_line(line);
                target = Target {
                    x: x + 10000000000000,
                    y: y + 10000000000000,
                };
            }
        }

        tokens_needed += compute_lowest_price(button_a, button_b, target);
    }

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day13_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 13:")?;
    writeln!(
        solution_file,
        "After correcting the claw positions, {} are required to obtain all obtainable tokens.",
        tokens_needed
    )?;

    Ok(())
}
