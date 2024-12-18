use std::cmp::Ordering;
use std::collections::BinaryHeap; // I want to only use the standard library
use std::collections::{HashMap, HashSet};

use std::fs;
use std::io::Error;
use std::io::Write;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Location {
    x: u32,
    y: u32,
}

#[derive(Eq, PartialEq, Hash)]
struct HeapEntry {
    location: Location,
    cost: u32,
}
impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap behavior
        other.cost.cmp(&self.cost)
    }
}
// Implement PartialOrd consistently with Ord
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other)) // Delegate to Ord::cmp
    }
}

struct MemorySpace {
    height: u32,
    width: u32,
    corrupted_pos: HashSet<Location>,
}
impl MemorySpace {
    fn new(height: u32, width: u32) -> MemorySpace {
        MemorySpace {
            height,
            width,
            corrupted_pos: HashSet::new(),
        }
    }

    fn corrupt_position(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            self.corrupted_pos.insert(Location { x, y });
        }
    }

    /// Dijkstra's algorithm
    fn shortest_path_length(&self, start_pos: Location, end_pos: Location) -> u32 {
        let mut priority_queue: BinaryHeap<HeapEntry> = BinaryHeap::new();
        let mut state_costs: HashMap<Location, u32> = HashMap::new();

        state_costs.insert(start_pos, 0);
        priority_queue.push(HeapEntry {
            location: start_pos,
            cost: 0,
        });

        while let Some(HeapEntry { location, cost }) = priority_queue.pop() {
            if let Some(&current_cost) = state_costs.get(&location) {
                if cost > current_cost {
                    continue; // ignore stale distance in heap
                }
            }
            if location == end_pos {
                return cost;
            }

            let mut neighbours = Vec::new();

            // move up
            if location.y > 0 {
                let move_up = Location {
                    x: location.x,
                    y: location.y - 1,
                };
                if !self.corrupted_pos.contains(&move_up) {
                    neighbours.push(move_up);
                }
            }
            // move down
            if location.y < self.height - 1 {
                let move_down = Location {
                    x: location.x,
                    y: location.y + 1,
                };
                if !self.corrupted_pos.contains(&move_down) {
                    neighbours.push(move_down);
                }
            }
            // move left
            if location.x > 0 {
                let move_left = Location {
                    x: location.x - 1,
                    y: location.y,
                };
                if !self.corrupted_pos.contains(&move_left) {
                    neighbours.push(move_left);
                }
            }
            // move right
            if location.x < self.width - 1 {
                let move_right = Location {
                    x: location.x + 1,
                    y: location.y,
                };
                if !self.corrupted_pos.contains(&move_right) {
                    neighbours.push(move_right);
                }
            }

            for &neighbour in neighbours.iter() {
                if let Some(&prev_cost) = state_costs.get(&neighbour) {
                    if (cost + 1) < prev_cost {
                        state_costs.insert(neighbour, cost + 1);
                        priority_queue.push(HeapEntry {
                            cost: cost + 1,
                            location: neighbour,
                        });
                    }
                } else {
                    state_costs.insert(neighbour, cost + 1);
                    priority_queue.push(HeapEntry {
                        cost: cost + 1,
                        location: neighbour,
                    });
                }
            }
        }
        u32::MAX // end location unreachable
    }
}

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 18!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 18...");

    let input_data = fs::read_to_string("input_data/day18_input.txt")?;

    let mut memory_space = MemorySpace::new(71, 71);
    for (i, corrupted_loc_str) in input_data.trim().lines().enumerate() {
        if i >= 1024 {
            break;
        }
        let (corrupted_x_str, corrupted_y_str) = corrupted_loc_str.split_once(',').unwrap();
        let corrupted_x: u32 = corrupted_x_str.parse().unwrap();
        let corrupted_y: u32 = corrupted_y_str.parse().unwrap();

        memory_space.corrupt_position(corrupted_x, corrupted_y);
    }

    let shortest_path_length =
        memory_space.shortest_path_length(Location { x: 0, y: 0 }, Location { x: 70, y: 70 });

    let mut solution_file = fs::File::create("solutions/day18_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 18:")?;
    writeln!(solution_file, "After 1024 bytes have fallen, the shortest path from (0, 0) to (70, 70) avoiding corrupted memory locations has length {}.", shortest_path_length)?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 18...");

    let solution = 0; // TODO

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day18_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 18:")?;
    writeln!(solution_file, "TODO {}.", solution)?;

    Ok(())
}
