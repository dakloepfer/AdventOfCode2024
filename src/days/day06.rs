use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use std::fs;
use std::io::Error;
use std::io::Write;

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 6!");

    Ok(())
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct GuardLocation {
    row: usize,
    col: usize,
}

struct Map {
    barriers_per_row: HashMap<usize, HashSet<usize>>, // for all the rows containing barriers, give the (sorted) columns the barriers exist at
    barriers_per_col: HashMap<usize, HashSet<usize>>, // for all the cols containing barriers, give the (sorted) rows the barriers exist at
    height: usize,
    width: usize,
}
impl Map {
    pub fn from_string(input_data: String) -> Result<(Map, GuardLocation, Direction), Error> {
        let mut barriers_per_row = HashMap::new();
        let mut barriers_per_col = HashMap::new();
        let mut guard_location = GuardLocation { row: 0, col: 0 };
        let mut guard_direction: Direction = Direction::Up;

        let mut height = 0;
        let mut width = 0;
        for (row, line) in input_data.lines().enumerate() {
            height = row;
            for (col, val) in line.chars().enumerate() {
                width = col;
                match val {
                    '#' => {
                        if let Entry::Vacant(e) = barriers_per_row.entry(row) {
                            e.insert(HashSet::from([col]));
                        } else {
                            barriers_per_row.get_mut(&row).unwrap().insert(col);
                        }
                        if let Entry::Vacant(e) = barriers_per_col.entry(col) {
                            e.insert(HashSet::from([row]));
                        } else {
                            barriers_per_col.get_mut(&col).unwrap().insert(row);
                        }
                    }
                    '^' => {
                        guard_location = GuardLocation { row, col };
                        guard_direction = Direction::Up;
                    }
                    '>' => {
                        guard_location = GuardLocation { row, col };
                        guard_direction = Direction::Right;
                    }
                    'v' => {
                        guard_location = GuardLocation { row, col };
                        guard_direction = Direction::Down;
                    }
                    '<' => {
                        guard_location = GuardLocation { row, col };
                        guard_direction = Direction::Left;
                    }
                    _ => {}
                }
            }
        }
        height = height.saturating_add(1); // index to dimension
        width = width.saturating_add(1);

        Ok((
            Map {
                barriers_per_row,
                barriers_per_col,
                height,
                width,
            },
            guard_location,
            guard_direction,
        ))
    }
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 6...");

    let input_data = fs::read_to_string("input_data/day06_input.txt")?;

    // guard location is row, col
    let (map, mut guard_location, mut guard_direction) = Map::from_string(input_data)?;
    let mut visited_locations: HashSet<(usize, usize)> = HashSet::new();
    'walking_loop: loop {
        match guard_direction {
            Direction::Up => {
                if let Some(barrier_rows) = map.barriers_per_col.get(&guard_location.col) {
                    let mut row = guard_location.row;
                    if row == 0 {
                        break 'walking_loop;
                    }
                    loop {
                        if barrier_rows.contains(&row) {
                            guard_location.row = row + 1;
                            guard_direction = Direction::Right;
                            break;
                        }
                        visited_locations.insert((row, guard_location.col));
                        if row == 0 {
                            break 'walking_loop;
                        }
                        row -= 1;
                    }
                } else {
                    for row in 0..guard_location.row {
                        visited_locations.insert((row, guard_location.col));
                    }
                    break 'walking_loop;
                }
            }
            Direction::Down => {
                if let Some(barrier_rows) = map.barriers_per_col.get(&guard_location.col) {
                    let mut row = guard_location.row;
                    if row == map.height - 1 {
                        break 'walking_loop;
                    }
                    loop {
                        if barrier_rows.contains(&row) {
                            guard_location.row = row - 1;
                            guard_direction = Direction::Left;
                            break;
                        }
                        visited_locations.insert((row, guard_location.col));
                        if row == map.height - 1 {
                            break 'walking_loop;
                        }
                        row += 1;
                    }
                } else {
                    for row in guard_location.row..map.height {
                        visited_locations.insert((row, guard_location.col));
                    }
                    break 'walking_loop;
                }
            }
            Direction::Left => {
                if let Some(barrier_cols) = map.barriers_per_row.get(&guard_location.row) {
                    let mut col = guard_location.col;
                    if col == 0 {
                        break 'walking_loop;
                    }
                    loop {
                        if barrier_cols.contains(&col) {
                            guard_location.col = col + 1;
                            guard_direction = Direction::Up;
                            break;
                        }
                        visited_locations.insert((guard_location.row, col));
                        if col == 0 {
                            break 'walking_loop;
                        }
                        col -= 1;
                    }
                } else {
                    for col in 0..guard_location.col {
                        visited_locations.insert((guard_location.row, col));
                    }
                    break 'walking_loop;
                }
            }
            Direction::Right => {
                if let Some(barrier_cols) = map.barriers_per_row.get(&guard_location.row) {
                    let mut col = guard_location.col;
                    if col == map.width - 1 {
                        break 'walking_loop;
                    }
                    loop {
                        if barrier_cols.contains(&col) {
                            guard_location.col = col - 1;
                            guard_direction = Direction::Down;
                            break;
                        }
                        visited_locations.insert((guard_location.row, col));
                        if col == map.width - 1 {
                            break 'walking_loop;
                        }
                        col += 1;
                    }
                } else {
                    for col in guard_location.col..map.width {
                        visited_locations.insert((guard_location.row, col));
                    }
                    break 'walking_loop;
                }
            }
        }
    }

    let num_visited_locations = visited_locations.len();

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day06_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 06:")?;
    writeln!(
        solution_file,
        "The guard visits {} unique locations before exiting.",
        num_visited_locations
    )?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 6...");

    let solution = 0; // TODO

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day06_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 06:")?;
    writeln!(solution_file, "TODO {}.", solution)?;

    Ok(())
}
