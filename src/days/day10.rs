use std::collections::HashSet;
use std::fs;
use std::hash::Hash;
use std::io::Error;
use std::io::Write;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Location {
    row: usize,
    col: usize,
}

fn parse_map(input_data: String) -> (Vec<Vec<u32>>, HashSet<Location>, usize, usize) {
    let mut map = Vec::new();
    let mut trailheads = HashSet::new();
    let mut height = 0;
    let mut width = 0;

    for (row, line) in input_data.lines().enumerate() {
        map.push(Vec::new());
        height = row;
        for (col, ch) in line.chars().enumerate() {
            width = col;
            let digit = ch.to_digit(10).unwrap();
            map[row].push(digit);
            if digit == 0 {
                trailheads.insert(Location { row, col });
            }
        }
    }

    (map, trailheads, height + 1, width + 1)
}

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 10!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 10...");

    let input_data = fs::read_to_string("input_data/day10_input.txt")?;

    let (map, trailheads, height, width) = parse_map(input_data);

    let mut sum_trailhead_scores = 0;
    for &trailhead in trailheads.iter() {
        let mut reachable_peaks: HashSet<Location> = HashSet::new();

        let mut to_visit: HashSet<(u32, Location)> = HashSet::from([(0, trailhead)]);

        while let Some((current_height, current_loc)) = to_visit.iter().next().cloned() {
            to_visit.remove(&(current_height, current_loc));

            if current_height == 9 {
                reachable_peaks.insert(current_loc);
                continue;
            }

            let current_row = current_loc.row;
            let current_col = current_loc.col;

            // Up
            if (current_row > 0) && (map[current_row - 1][current_col] == current_height + 1) {
                to_visit.insert((
                    current_height + 1,
                    Location {
                        row: current_row - 1,
                        col: current_col,
                    },
                ));
            }

            // Down
            if (current_row < height - 1)
                && (map[current_row + 1][current_col] == current_height + 1)
            {
                to_visit.insert((
                    current_height + 1,
                    Location {
                        row: current_row + 1,
                        col: current_col,
                    },
                ));
            }

            // Left
            if (current_col > 0) && (map[current_row][current_col - 1] == current_height + 1) {
                to_visit.insert((
                    current_height + 1,
                    Location {
                        row: current_row,
                        col: current_col - 1,
                    },
                ));
            }

            // Right
            if (current_col < width - 1)
                && (map[current_row][current_col + 1] == current_height + 1)
            {
                to_visit.insert((
                    current_height + 1,
                    Location {
                        row: current_row,
                        col: current_col + 1,
                    },
                ));
            }
        }
        sum_trailhead_scores += reachable_peaks.len();
    }

    let mut solution_file = fs::File::create("solutions/day10_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 10:")?;
    writeln!(
        solution_file,
        "The sum of all trailhead scores is {}.",
        sum_trailhead_scores
    )?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 10...");

    let input_data = fs::read_to_string("input_data/day10_input.txt")?;

    let (map, trailheads, height, width) = parse_map(input_data);

    let mut sum_trailhead_ratings = 0;

    for &trailhead in trailheads.iter() {
        let mut current_rating: u32 = 0;

        let mut to_visit: Vec<(u32, Location)> = vec![(0, trailhead)];

        while let Some((current_height, current_loc)) = to_visit.pop() {
            if current_height == 9 {
                current_rating += 1;
                continue;
            }

            let current_row = current_loc.row;
            let current_col = current_loc.col;

            // Up
            if (current_row > 0) && (map[current_row - 1][current_col] == current_height + 1) {
                to_visit.push((
                    current_height + 1,
                    Location {
                        row: current_row - 1,
                        col: current_col,
                    },
                ));
            }

            // Down
            if (current_row < height - 1)
                && (map[current_row + 1][current_col] == current_height + 1)
            {
                to_visit.push((
                    current_height + 1,
                    Location {
                        row: current_row + 1,
                        col: current_col,
                    },
                ));
            }

            // Left
            if (current_col > 0) && (map[current_row][current_col - 1] == current_height + 1) {
                to_visit.push((
                    current_height + 1,
                    Location {
                        row: current_row,
                        col: current_col - 1,
                    },
                ));
            }

            // Right
            if (current_col < width - 1)
                && (map[current_row][current_col + 1] == current_height + 1)
            {
                to_visit.push((
                    current_height + 1,
                    Location {
                        row: current_row,
                        col: current_col + 1,
                    },
                ));
            }
        }
        sum_trailhead_ratings += current_rating;
    }

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day10_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 10:")?;
    writeln!(
        solution_file,
        "The sum of all trailhead ratings is {}.",
        sum_trailhead_ratings
    )?;

    Ok(())
}
