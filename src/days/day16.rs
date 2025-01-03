use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::BinaryHeap; // I want to only use the standard library
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;
use std::io::Error;
use std::io::Write;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum Orientation {
    North,
    East,
    South,
    West,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Location {
    x: i32,
    y: i32, //measured from top-left, with y down
}
impl Location {
    fn new() -> Location {
        Location { x: 0, y: 0 }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct ReindeerState {
    location: Location,
    orientation: Orientation,
}
impl ReindeerState {
    fn new() -> ReindeerState {
        ReindeerState {
            location: Location::new(),
            orientation: Orientation::East,
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct HeapEntry {
    reindeer_state: ReindeerState,
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

struct Maze {
    wall_pos: HashSet<Location>,
    start_state: ReindeerState,
    end_location: Location,
    turning_cost: u32,
    moving_cost: u32,
}
impl Maze {
    fn from_str(input: &str) -> Maze {
        let mut wall_pos = HashSet::new();
        let mut start_state = ReindeerState::new();

        let mut end_location = Location::new();

        for (row, line) in input.lines().enumerate() {
            for (col, char) in line.char_indices() {
                match char {
                    '#' => {
                        wall_pos.insert(Location {
                            x: col as i32,
                            y: row as i32,
                        });
                    }
                    'S' => {
                        start_state = ReindeerState {
                            location: Location {
                                x: col as i32,
                                y: row as i32,
                            },
                            orientation: Orientation::East,
                        }
                    }
                    'E' => {
                        end_location = Location {
                            x: col as i32,
                            y: row as i32,
                        }
                    }
                    _ => {}
                }
            }
        }

        Maze {
            wall_pos,
            start_state,
            end_location,
            turning_cost: 1000,
            moving_cost: 1,
        }
    }

    fn dijkstra(&self) -> (i32, HashSet<Location>) {
        let mut priority_queue: BinaryHeap<HeapEntry> = BinaryHeap::new();
        let mut state_costs: HashMap<ReindeerState, u32> = HashMap::new();
        let mut predecessors: HashMap<ReindeerState, Vec<ReindeerState>> = HashMap::new();
        state_costs.insert(self.start_state, 0);
        priority_queue.push(HeapEntry {
            reindeer_state: self.start_state,
            cost: 0,
        });
        predecessors.insert(self.start_state, Vec::new());

        while let Some(HeapEntry {
            reindeer_state,
            cost,
        }) = priority_queue.pop()
        {
            if let Some(&current_cost) = state_costs.get(&reindeer_state) {
                if cost > current_cost {
                    continue; // ignore stale distance in heap
                }
            }
            if reindeer_state.location == self.end_location {
                let mut any_shortest_path_locations: HashSet<Location> = HashSet::new();
                let mut states_to_add: Vec<ReindeerState> = vec![reindeer_state];

                while let Some(state_to_add) = states_to_add.pop() {
                    any_shortest_path_locations.insert(state_to_add.location);
                    for predecessor in predecessors.get(&state_to_add).unwrap() {
                        states_to_add.push(*predecessor);
                    }
                }

                return (cost as i32, any_shortest_path_locations);
            }

            let turn_left;
            let turn_right;
            let move_forward;

            match reindeer_state.orientation {
                Orientation::North => {
                    turn_left = ReindeerState {
                        location: reindeer_state.location,
                        orientation: Orientation::West,
                    };
                    turn_right = ReindeerState {
                        location: reindeer_state.location,
                        orientation: Orientation::East,
                    };
                    move_forward = ReindeerState {
                        location: Location {
                            x: reindeer_state.location.x,
                            y: reindeer_state.location.y - 1,
                        },
                        orientation: reindeer_state.orientation,
                    };
                }
                Orientation::East => {
                    turn_left = ReindeerState {
                        location: reindeer_state.location,
                        orientation: Orientation::North,
                    };
                    turn_right = ReindeerState {
                        location: reindeer_state.location,
                        orientation: Orientation::South,
                    };
                    move_forward = ReindeerState {
                        location: Location {
                            x: reindeer_state.location.x + 1,
                            y: reindeer_state.location.y,
                        },
                        orientation: reindeer_state.orientation,
                    };
                }
                Orientation::South => {
                    turn_left = ReindeerState {
                        location: reindeer_state.location,
                        orientation: Orientation::East,
                    };
                    turn_right = ReindeerState {
                        location: reindeer_state.location,
                        orientation: Orientation::West,
                    };
                    move_forward = ReindeerState {
                        location: Location {
                            x: reindeer_state.location.x,
                            y: reindeer_state.location.y + 1,
                        },
                        orientation: reindeer_state.orientation,
                    };
                }
                Orientation::West => {
                    turn_left = ReindeerState {
                        location: reindeer_state.location,
                        orientation: Orientation::South,
                    };
                    turn_right = ReindeerState {
                        location: reindeer_state.location,
                        orientation: Orientation::North,
                    };
                    move_forward = ReindeerState {
                        location: Location {
                            x: reindeer_state.location.x - 1,
                            y: reindeer_state.location.y,
                        },
                        orientation: reindeer_state.orientation,
                    };
                }
            }

            let turn_left_cost = cost + self.turning_cost;
            let turn_right_cost = cost + self.turning_cost;
            let move_forward_cost = cost + self.moving_cost;

            if let Some(&prev_turn_left_cost) = state_costs.get(&turn_left) {
                match turn_left_cost.cmp(&prev_turn_left_cost) {
                    Less => {
                        state_costs.insert(turn_left, turn_left_cost);
                        priority_queue.push(HeapEntry {
                            cost: turn_left_cost,
                            reindeer_state: turn_left,
                        });
                        predecessors.insert(turn_left, vec![reindeer_state]);
                    }
                    Equal => {
                        predecessors
                            .get_mut(&turn_left)
                            .unwrap()
                            .push(reindeer_state);
                    }
                    Greater => {}
                }
            } else {
                state_costs.insert(turn_left, turn_left_cost);
                priority_queue.push(HeapEntry {
                    cost: cost + self.turning_cost,
                    reindeer_state: turn_left,
                });
                predecessors.insert(turn_left, vec![reindeer_state]);
            }

            if let Some(&prev_turn_right_cost) = state_costs.get(&turn_right) {
                match turn_right_cost.cmp(&prev_turn_right_cost) {
                    Less => {
                        state_costs.insert(turn_right, turn_right_cost);
                        priority_queue.push(HeapEntry {
                            cost: turn_right_cost,
                            reindeer_state: turn_right,
                        });
                        predecessors.insert(turn_right, vec![reindeer_state]);
                    }
                    Equal => {
                        predecessors
                            .get_mut(&turn_right)
                            .unwrap()
                            .push(reindeer_state);
                    }
                    Greater => {}
                }
            } else {
                state_costs.insert(turn_right, cost + self.turning_cost);
                priority_queue.push(HeapEntry {
                    cost: cost + self.turning_cost,
                    reindeer_state: turn_right,
                });
                predecessors.insert(turn_right, vec![reindeer_state]);
            }

            if !self.wall_pos.contains(&move_forward.location) {
                if let Some(&prev_move_forward_cost) = state_costs.get(&move_forward) {
                    match move_forward_cost.cmp(&prev_move_forward_cost) {
                        Less => {
                            state_costs.insert(move_forward, cost + self.turning_cost);
                            priority_queue.push(HeapEntry {
                                cost: move_forward_cost,
                                reindeer_state: move_forward,
                            });
                            predecessors.insert(move_forward, vec![reindeer_state]);
                        }
                        Equal => {
                            predecessors
                                .get_mut(&move_forward)
                                .unwrap()
                                .push(reindeer_state);
                        }
                        Greater => {}
                    }
                } else {
                    state_costs.insert(move_forward, cost + self.moving_cost);
                    priority_queue.push(HeapEntry {
                        cost: move_forward_cost,
                        reindeer_state: move_forward,
                    });
                    predecessors.insert(move_forward, vec![reindeer_state]);
                }
            }
        }

        (i32::MAX, HashSet::new()) // end location unreachable
    }
}

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 16!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 16...");

    let input_data = fs::read_to_string("input_data/day16_input.txt")?;

    let maze = Maze::from_str(&input_data);
    let (lowest_cost, _) = maze.dijkstra();

    let mut solution_file = fs::File::create("solutions/day16_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 16:")?;
    writeln!(
        solution_file,
        "The lowest cost to move from Start to End is {}.",
        lowest_cost
    )?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 16...");

    let input_data = fs::read_to_string("input_data/day16_input.txt")?;

    let maze = Maze::from_str(&input_data);
    let (_, any_best_path_locations) = maze.dijkstra();

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day16_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 16:")?;
    writeln!(
        solution_file,
        "There are {} locations that are on at least one closest path.",
        any_best_path_locations.len()
    )?;

    Ok(())
}
