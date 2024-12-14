use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Error;
use std::io::Write;

fn gcd(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs() // Ensure GCD is always positive
}
fn reduce_location_diff(row_diff: isize, col_diff: isize) -> (isize, isize) {
    let gcd = gcd(row_diff, col_diff);
    (row_diff / gcd, col_diff / gcd)
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Location {
    row: isize,
    col: isize,
}

type AntennaLocations = HashMap<char, Vec<Location>>;
type AntennaPair = (Location, Location);

fn parse_map(map: Vec<Vec<char>>) -> (AntennaLocations, isize, isize) {
    let mut antenna_locations: AntennaLocations = HashMap::new();
    let mut height = 0;
    let mut width = 0;
    for (row, line) in map.iter().enumerate() {
        height = row;
        for (col, ch) in line.iter().enumerate() {
            width = col;
            if *ch == '.' {
                continue;
            }
            if antenna_locations.contains_key(ch) {
                antenna_locations.get_mut(ch).unwrap().push(Location {
                    row: row.try_into().unwrap(),
                    col: col.try_into().unwrap(),
                });
            } else {
                antenna_locations.insert(
                    *ch,
                    vec![Location {
                        row: row.try_into().unwrap(),
                        col: col.try_into().unwrap(),
                    }],
                );
            }
        }
    }
    (
        antenna_locations,
        (height + 1).try_into().unwrap(),
        (width + 1).try_into().unwrap(),
    )
}

fn make_pairs(antenna_locations: AntennaLocations) -> Vec<AntennaPair> {
    let mut antenna_pairs: Vec<AntennaPair> = Vec::new();

    for frequency in antenna_locations.keys() {
        let locations = antenna_locations.get(frequency).unwrap();
        for i in 0..locations.len() {
            for j in (i + 1)..locations.len() {
                antenna_pairs.push((locations[i], locations[j]));
            }
        }
    }

    antenna_pairs
}

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 8!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 8...");

    let input_data: Vec<Vec<char>> = fs::read_to_string("input_data/day08_input.txt")?
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let (antenna_locations, height, width) = parse_map(input_data);
    let antenna_pairs = make_pairs(antenna_locations);

    let mut unique_antinodes: HashSet<Location> = HashSet::new();

    for (location_a, location_b) in antenna_pairs {
        let row_diff = location_b.row - location_a.row;
        let col_diff = location_b.col - location_a.col;

        let new_row1 = location_a.row - row_diff;
        let new_col1 = location_a.col - col_diff;

        if (new_row1 >= 0) && (new_row1 < height) && (new_col1 >= 0) && (new_col1 < width) {
            unique_antinodes.insert(Location {
                row: new_row1,
                col: new_col1,
            });
        }

        let new_row2 = location_b.row + row_diff;
        let new_col2 = location_b.col + col_diff;

        if (new_row2 >= 0) && (new_row2 < height) && (new_col2 >= 0) && (new_col2 < width) {
            unique_antinodes.insert(Location {
                row: new_row2,
                col: new_col2,
            });
        }
    }

    let mut solution_file = fs::File::create("solutions/day08_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 08:")?;
    writeln!(
        solution_file,
        "The map contains {} unique antinodes.",
        unique_antinodes.len()
    )?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 8...");

    let input_data: Vec<Vec<char>> = fs::read_to_string("input_data/day08_input.txt")?
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let (antenna_locations, height, width) = parse_map(input_data);
    let antenna_pairs = make_pairs(antenna_locations);

    let mut resonant_antinodes: HashSet<Location> = HashSet::new();

    for (location_a, location_b) in antenna_pairs {
        let raw_row_diff = location_b.row - location_a.row;
        let raw_col_diff = location_b.col - location_a.col;

        let (row_diff, col_diff) = reduce_location_diff(raw_row_diff, raw_col_diff);

        // Direction 1
        let mut factor = 0;
        loop {
            let new_row = location_a.row - factor * row_diff;
            let new_col = location_a.col - factor * col_diff;

            if (new_row >= 0) && (new_row < height) && (new_col >= 0) && (new_col < width) {
                resonant_antinodes.insert(Location {
                    row: new_row,
                    col: new_col,
                });
            } else {
                break;
            }

            factor += 1;
        }

        // Direction 2
        let mut factor = 0;
        loop {
            let new_row = location_a.row + factor * row_diff; // antinodes could be between antennas
            let new_col = location_a.col + factor * col_diff;

            if (new_row >= 0) && (new_row < height) && (new_col >= 0) && (new_col < width) {
                resonant_antinodes.insert(Location {
                    row: new_row,
                    col: new_col,
                });
            } else {
                break;
            }

            factor += 1;
        }
    }

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day08_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 08:")?;
    writeln!(
        solution_file,
        "The map contains {} unique resonant antinodes.",
        resonant_antinodes.len()
    )?;

    Ok(())
}
