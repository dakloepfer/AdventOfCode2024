use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::io::Write;
use std::vec::Vec; // I want to only use the standard library

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
enum Stripe {
    White,
    Blue,
    Black,
    Red,
    Green,
}

type Design = Vec<Stripe>;

#[derive(Eq, PartialEq, Hash)]
struct HeapEntry {
    design: Design,
}
impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap behavior
        other.design.len().cmp(&self.design.len())
    }
}
// Implement PartialOrd consistently with Ord
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other)) // Delegate to Ord::cmp
    }
}

fn parse_single_design(design_str: &str) -> Design {
    let mut design = Vec::new();
    for stripe_color in design_str.trim().chars() {
        let stripe = match stripe_color {
            'w' => Stripe::White,
            'u' => Stripe::Blue,
            'b' => Stripe::Black,
            'r' => Stripe::Red,
            'g' => Stripe::Green,
            _ => Stripe::White, // shouldn't happen
        };

        design.push(stripe);
    }
    design
}

fn parse_designs(design_str: &str) -> Vec<Design> {
    let mut designs = Vec::new();

    for single_design in design_str.lines() {
        designs.push(parse_single_design(single_design));
    }

    designs
}

/// Maps final Strip in Towel to the full towel
fn parse_towels(towels_str: &str) -> HashMap<Stripe, Vec<Design>> {
    let mut towels = HashMap::new();

    let towel_design_str: Vec<&str> = towels_str.split(",").collect();

    for single_towel_design_str in towel_design_str.iter() {
        let towel_design = parse_single_design(single_towel_design_str);

        towels
            .entry(*towel_design.last().unwrap())
            .or_insert(Vec::new())
            .push(towel_design);
    }

    towels
}

fn possible_remaining_design(remaining_design: Design, towel: Design) -> Option<Design> {
    let mut possible_remaining_design = remaining_design.clone();
    let mut towel_is_possible = true;

    for stripe in towel.iter().rev() {
        if let Some(required_next_stripe) = possible_remaining_design.pop() {
            if *stripe == required_next_stripe {
            } else {
                towel_is_possible = false;
                break;
            }
        } else {
            towel_is_possible = false;
            break;
        }
    }
    if towel_is_possible {
        Some(possible_remaining_design)
    } else {
        None
    }
}

fn is_design_possible(towels: HashMap<Stripe, Vec<Design>>, design: Design) -> bool {
    let mut priority_queue: BinaryHeap<HeapEntry> = BinaryHeap::new();

    priority_queue.push(HeapEntry { design });

    while let Some(HeapEntry {
        design: remaining_design,
    }) = priority_queue.pop()
    {
        if remaining_design.is_empty() {
            return true;
        }

        if let Some(possible_next_towels) = towels.get(remaining_design.last().unwrap()) {
            for possible_next_towel in possible_next_towels.iter() {
                if let Some(next_remaining_design) =
                    possible_remaining_design(remaining_design.clone(), possible_next_towel.clone())
                {
                    priority_queue.push(HeapEntry {
                        design: next_remaining_design,
                    });
                }
            }
        }
    }

    false
}

fn num_possible_arrangements(towels: HashMap<Stripe, Vec<Design>>, design: Design) -> u64 {
    fn dfs(
        towels: HashMap<Stripe, Vec<Design>>,
        design: Design,
        memo: &mut HashMap<Design, u64>,
    ) -> u64 {
        if let Some(&num_paths) = memo.get(&design) {
            return num_paths;
        }
        if design.is_empty() {
            return 1;
        }

        let mut total_paths = 0;
        if let Some(possible_next_towels) = towels.get(design.last().unwrap()) {
            for possible_next_towel in possible_next_towels.iter() {
                if let Some(next_remaining_design) =
                    possible_remaining_design(design.clone(), possible_next_towel.clone())
                {
                    total_paths += dfs(towels.clone(), next_remaining_design, memo);
                }
            }
        }
        memo.insert(design, total_paths);
        total_paths
    }

    let mut memo = HashMap::new();

    dfs(towels, design, &mut memo)
}

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 19!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 19...");

    let input_data = fs::read_to_string("input_data/day19_input.txt")?;

    let (towels_str, design_str) = input_data.split_once("\n\n").unwrap();
    let towels = parse_towels(towels_str);
    let designs: Vec<Design> = parse_designs(design_str);

    let mut num_possible_designs: u32 = 0;
    for design in designs.iter() {
        if is_design_possible(towels.clone(), design.to_vec()) {
            num_possible_designs += 1;
        }
    }

    let mut solution_file = fs::File::create("solutions/day19_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 19:")?;
    writeln!(
        solution_file,
        "{} of the designs provided are possible.",
        num_possible_designs
    )?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 19...");

    let input_data = fs::read_to_string("input_data/day19_input.txt")?;

    let (towels_str, design_str) = input_data.split_once("\n\n").unwrap();
    let towels = parse_towels(towels_str);
    let designs: Vec<Design> = parse_designs(design_str);

    let mut sum_num_possible_arrangements: u64 = 0;
    for design in designs.iter() {
        if is_design_possible(towels.clone(), design.to_vec()) {
            sum_num_possible_arrangements +=
                num_possible_arrangements(towels.clone(), design.to_vec());
        }
    }

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day19_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 19:")?;
    writeln!(
        solution_file,
        "The sum of all the ways to arrange each design is {}.",
        sum_num_possible_arrangements
    )?;

    Ok(())
}
