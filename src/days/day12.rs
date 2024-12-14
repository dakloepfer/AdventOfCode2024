use std::collections::HashSet;
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

                // Right
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

    let solution = 0; // TODO

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day12_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 12:")?;
    writeln!(solution_file, "TODO {}.", solution)?;

    Ok(())
}
