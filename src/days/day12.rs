use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Error;
use std::io::Write;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Location {
    row: usize,
    col: usize,
}

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 12!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 12...");

    let map: Vec<Vec<char>> = fs::read_to_string("input_data/day12_input.txt")?
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let height = map.len();
    let width = map.first().unwrap().len();

    let mut total_price: u32 = 0;
    let mut checked_locs: HashSet<Location> = HashSet::new();

    for (row, line) in map.iter().enumerate() {
        for (col, plant) in line.iter().enumerate() {
            if checked_locs.contains(&Location { row, col }) {
                continue;
            }

            let mut area: u32 = 0;
            let mut perimeter: u32 = 0;
            let mut to_check: HashSet<Location> = HashSet::from([Location { row, col }]);

            while let Some(current_location) = to_check.iter().next().cloned() {
                to_check.remove(&current_location);
                if checked_locs.contains(&current_location) {
                    continue;
                }

                // Up
                if current_location.row > 0
                    && map[current_location.row - 1][current_location.col] == *plant
                {
                    to_check.insert(Location {
                        row: current_location.row - 1,
                        col: current_location.col,
                    });
                } else {
                    perimeter += 1;
                }

                // Down
                if current_location.row < height - 1
                    && map[current_location.row + 1][current_location.col] == *plant
                {
                    to_check.insert(Location {
                        row: current_location.row + 1,
                        col: current_location.col,
                    });
                } else {
                    perimeter += 1;
                }

                // Left
                if current_location.col > 0
                    && map[current_location.row][current_location.col - 1] == *plant
                {
                    to_check.insert(Location {
                        row: current_location.row,
                        col: current_location.col - 1,
                    });
                } else {
                    perimeter += 1;
                }

                // Right
                if current_location.col < width - 1
                    && map[current_location.row][current_location.col + 1] == *plant
                {
                    to_check.insert(Location {
                        row: current_location.row,
                        col: current_location.col + 1,
                    });
                } else {
                    perimeter += 1;
                }

                area += 1;
                checked_locs.insert(current_location);
            }
            total_price += area * perimeter;
        }
    }

    let mut solution_file = fs::File::create("solutions/day12_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 12:")?;
    writeln!(
        solution_file,
        "The total price for all fences is {}.",
        total_price
    )?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 12...");

    let map: Vec<Vec<char>> = fs::read_to_string("input_data/day12_input.txt")?
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let height = map.len();
    let width = map.first().unwrap().len();

    let mut total_price: u32 = 0;
    let mut checked_locs: HashSet<Location> = HashSet::new();
    for (row, line) in map.iter().enumerate() {
        for (col, plant) in line.iter().enumerate() {
            if checked_locs.contains(&Location { row, col }) {
                continue;
            }
            let mut region_locations: HashSet<Location> = HashSet::from([Location { row, col }]);

            let mut area: u32 = 0;
            let mut locs_to_check: HashSet<Location> = HashSet::from([Location { row, col }]);

            // key row, val col
            let mut region_up_edges: HashMap<usize, Vec<usize>> = HashMap::new();
            let mut region_down_edges: HashMap<usize, Vec<usize>> = HashMap::new();

            // key col, val row
            let mut region_left_edges: HashMap<usize, Vec<usize>> = HashMap::new();
            let mut region_right_edges: HashMap<usize, Vec<usize>> = HashMap::new();

            while let Some(current_location) = locs_to_check.iter().next().cloned() {
                locs_to_check.remove(&current_location);
                if checked_locs.contains(&current_location) {
                    continue;
                }

                // Up
                if current_location.row > 0
                    && map[current_location.row - 1][current_location.col] == *plant
                {
                    locs_to_check.insert(Location {
                        row: current_location.row - 1,
                        col: current_location.col,
                    });
                } else {
                    region_up_edges
                        .entry(current_location.row)
                        .or_default()
                        .push(current_location.col);
                }

                // Down
                if current_location.row < height - 1
                    && map[current_location.row + 1][current_location.col] == *plant
                {
                    locs_to_check.insert(Location {
                        row: current_location.row + 1,
                        col: current_location.col,
                    });
                } else {
                    region_down_edges
                        .entry(current_location.row)
                        .or_default()
                        .push(current_location.col);
                }

                // Left
                if current_location.col > 0
                    && map[current_location.row][current_location.col - 1] == *plant
                {
                    locs_to_check.insert(Location {
                        row: current_location.row,
                        col: current_location.col - 1,
                    });
                } else {
                    region_left_edges
                        .entry(current_location.col)
                        .or_default()
                        .push(current_location.row);
                }

                // Right
                if current_location.col < width - 1
                    && map[current_location.row][current_location.col + 1] == *plant
                {
                    locs_to_check.insert(Location {
                        row: current_location.row,
                        col: current_location.col + 1,
                    });
                } else {
                    region_right_edges
                        .entry(current_location.col)
                        .or_default()
                        .push(current_location.row);
                }

                area += 1;
                checked_locs.insert(current_location);
                region_locations.insert(current_location);
            }

            let mut num_sides: u32 = 0;

            // count up sides
            for (_row, mut cols) in region_up_edges {
                cols.sort();
                num_sides += 1;
                let mut prev_col = cols.first().unwrap();
                for col in cols[1..].iter() {
                    if col - prev_col > 1 {
                        num_sides += 1;
                    }
                    prev_col = col;
                }
            }
            // count down sides
            for (_row, mut cols) in region_down_edges {
                cols.sort();
                num_sides += 1;
                let mut prev_col = cols.first().unwrap();
                for col in cols[1..].iter() {
                    if col - prev_col > 1 {
                        num_sides += 1;
                    }
                    prev_col = col;
                }
            }
            // count left sides
            for (_col, mut rows) in region_left_edges {
                rows.sort();
                num_sides += 1;
                let mut prev_row = rows.first().unwrap();
                for row in rows[1..].iter() {
                    if row - prev_row > 1 {
                        num_sides += 1;
                    }
                    prev_row = row;
                }
            }
            // count right sides
            for (_col, mut rows) in region_right_edges {
                rows.sort();
                num_sides += 1;
                let mut prev_row = rows.first().unwrap();
                for row in rows[1..].iter() {
                    if row - prev_row > 1 {
                        num_sides += 1;
                    }
                    prev_row = row;
                }
            }

            total_price += area * num_sides;
        }
    }

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day12_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 12:")?;
    writeln!(
        solution_file,
        "The total price for all fences with the bulk discount is {}.",
        total_price
    )?;

    Ok(())
}
