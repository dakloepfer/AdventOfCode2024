use std::collections::HashSet;

use std::fs;
use std::io::Error;
use std::io::Write;

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 6!");

    Ok(())
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Location {
    row: usize,
    col: usize,
}

struct Map {
    barriers: HashSet<Location>,
    height: usize,
    width: usize,
}
impl Map {
    pub fn from_string(input_data: String) -> Result<(Map, Location, Direction), Error> {
        let mut barriers = HashSet::new();
        let mut guard_location = Location { row: 0, col: 0 };
        let mut guard_direction: Direction = Direction::Up;

        let mut height = 0;
        let mut width = 0;
        for (row, line) in input_data.lines().enumerate() {
            height = row;
            for (col, val) in line.chars().enumerate() {
                width = col;
                match val {
                    '#' => {
                        barriers.insert(Location { row, col });
                    }
                    '^' => {
                        guard_location = Location { row, col };
                        guard_direction = Direction::Up;
                    }
                    '>' => {
                        guard_location = Location { row, col };
                        guard_direction = Direction::Right;
                    }
                    'v' => {
                        guard_location = Location { row, col };
                        guard_direction = Direction::Down;
                    }
                    '<' => {
                        guard_location = Location { row, col };
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
                barriers,
                height,
                width,
            },
            guard_location,
            guard_direction,
        ))
    }
}

fn walk_guard(
    map: Map,
    mut guard_location: Location,
    mut guard_direction: Direction,
) -> Result<usize, Error> {
    let mut visited_locations: HashSet<Location> = HashSet::new();

    'walking_loop: loop {
        visited_locations.insert(guard_location);

        match guard_direction {
            Direction::Up => {
                if guard_location.row == 0 {
                    break 'walking_loop;
                } else {
                    guard_location.row -= 1;
                }
            }
            Direction::Down => {
                if guard_location.row == map.height - 1 {
                    break 'walking_loop;
                } else {
                    guard_location.row += 1;
                }
            }
            Direction::Left => {
                if guard_location.col == 0 {
                    break 'walking_loop;
                } else {
                    guard_location.col -= 1;
                }
            }
            Direction::Right => {
                if guard_location.col == map.width - 1 {
                    break 'walking_loop;
                } else {
                    guard_location.col += 1;
                }
            }
        }

        if map.barriers.contains(&guard_location) {
            match guard_direction {
                Direction::Up => {
                    guard_location.row += 1;
                    guard_direction = Direction::Right;
                }
                Direction::Down => {
                    guard_location.row -= 1;
                    guard_direction = Direction::Left;
                }
                Direction::Left => {
                    guard_location.col += 1;
                    guard_direction = Direction::Up;
                }
                Direction::Right => {
                    guard_location.col -= 1;
                    guard_direction = Direction::Down;
                }
            }
        }
    }

    Ok(visited_locations.len())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 6...");

    let input_data = fs::read_to_string("input_data/day06_input.txt")?;

    // guard location is row, col
    let (map, guard_location, guard_direction) = Map::from_string(input_data)?;
    let num_visited_locations = walk_guard(map, guard_location, guard_direction)?;

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
