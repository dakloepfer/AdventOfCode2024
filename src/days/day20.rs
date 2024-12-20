use std::collections::HashSet;
use std::fs;
use std::hash::Hash;
use std::io::Error;
use std::io::Write;
use std::ops::Sub;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Location {
    x: i32,
    y: i32, //measured from top-left, with y down
}
impl Location {
    fn new() -> Location {
        Location { x: 0, y: 0 }
    }
    fn magnitude(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}
impl Sub for Location {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Location {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

#[derive(Clone)]
struct RaceTrack {
    track_locations: Vec<Location>,
}
impl RaceTrack {
    fn from_str(input: &str) -> RaceTrack {
        let mut start_location = Location::new();
        let mut unordered_track_locations: HashSet<Location> = HashSet::new();

        for (row, line) in input.lines().enumerate() {
            for (col, char) in line.char_indices() {
                match char {
                    '#' => {}
                    'S' => {
                        start_location = Location {
                            x: col as i32,
                            y: row as i32,
                        }
                    }
                    '.' => {
                        unordered_track_locations.insert(Location {
                            x: col as i32,
                            y: row as i32,
                        });
                    }
                    _ => {}
                }
            }
        }

        let mut track_locations = Vec::new();
        let mut current_location = start_location;
        while !unordered_track_locations.is_empty() {
            track_locations.push(current_location);
            unordered_track_locations.remove(&current_location);

            // The race track is at least one wall from the edge away, so don't need to check for out-of-bounds
            let potential_next_locations = [
                Location {
                    x: current_location.x,
                    y: current_location.y - 1,
                },
                Location {
                    x: current_location.x,
                    y: current_location.y + 1,
                },
                Location {
                    x: current_location.x - 1,
                    y: current_location.y,
                },
                Location {
                    x: current_location.x + 1,
                    y: current_location.y,
                },
            ];
            for potential_next_location in potential_next_locations.iter() {
                if unordered_track_locations.contains(potential_next_location) {
                    current_location = *potential_next_location;
                    break;
                }
            }
        }

        RaceTrack { track_locations }
    }

    fn num_acceptable_shortcuts(&self, min_saving: u32) -> u32 {
        let mut num_acceptable_shortcuts = 0;

        for (shortcut_start_idx, &shortcut_start_location) in
            self.track_locations.iter().enumerate()
        {
            for (shortcut_end_idx, &shortcut_end_location) in
                self.track_locations.iter().enumerate()
            {
                if shortcut_end_idx < shortcut_start_idx + min_saving as usize + 2 {
                    continue;
                }
                if (shortcut_end_location - shortcut_start_location).magnitude() == 2 {
                    num_acceptable_shortcuts += 1;
                }
            }
        }

        num_acceptable_shortcuts
    }
}

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 20!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 20...");

    let input_data = fs::read_to_string("input_data/day20_input.txt")?;

    let racetrack = RaceTrack::from_str(&input_data);
    let num_acceptable_shortcuts = racetrack.num_acceptable_shortcuts(100);

    let mut solution_file = fs::File::create("solutions/day20_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 20:")?;
    writeln!(
        solution_file,
        "There are {} routes where cheating once would save at least 100 picoseconds.",
        num_acceptable_shortcuts
    )?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 20...");

    let solution = 0; // TODO

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day20_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 20:")?;
    writeln!(solution_file, "TODO {}.", solution)?;

    Ok(())
}
