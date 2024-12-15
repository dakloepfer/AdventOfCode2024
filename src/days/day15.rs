use std::collections::HashSet;
use std::fs;
use std::io::Error;
use std::io::Write;

#[derive(Clone, Copy)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Location {
    row: i32,
    col: i32,
}
impl Location {
    fn new() -> Location {
        Location { row: 0, col: 0 }
    }
}
struct Map {
    robot_pos: Location,
    wall_pos: HashSet<Location>,
    box_pos: HashSet<Location>,
}
impl Map {
    fn from_str(input: &str) -> Map {
        let mut robot_pos = Location::new();
        let mut wall_pos = HashSet::new();
        let mut box_pos = HashSet::new();
        for (row, line) in input.lines().enumerate() {
            for (col, char) in line.char_indices() {
                match char {
                    '#' => wall_pos.insert(Location {
                        row: row as i32,
                        col: col as i32,
                    }),
                    'O' => box_pos.insert(Location {
                        row: row as i32,
                        col: col as i32,
                    }),
                    '@' => {
                        robot_pos = Location {
                            row: row as i32,
                            col: col as i32,
                        };
                        true
                    }
                    _ => true,
                };
            }
        }

        Map {
            robot_pos,
            wall_pos,
            box_pos,
        }
    }

    fn sum_box_gps_coords(&self) -> i32 {
        let mut sum_gps_coords = 0;
        for box_location in self.box_pos.iter() {
            sum_gps_coords += box_location.row.abs() * 100 + box_location.col.abs();
        }
        sum_gps_coords
    }

    fn move_robot(&mut self, movement: Movement) {
        let mut current_pos = self.robot_pos;

        let mut movement_possible = true;
        let mut last_moved_box: Option<Location> = None;
        loop {
            match movement {
                Movement::Up => current_pos.row -= 1,
                Movement::Down => current_pos.row += 1,
                Movement::Left => current_pos.col -= 1,
                Movement::Right => current_pos.col += 1,
            };
            if self.box_pos.contains(&current_pos) {
                last_moved_box = Some(current_pos);
            } else if self.wall_pos.contains(&current_pos) {
                movement_possible = false;
                break;
            } else {
                break;
            }
        }
        if movement_possible {
            match movement {
                Movement::Up => {
                    self.robot_pos.row -= 1;
                    self.box_pos.remove(&self.robot_pos);
                    if let Some(last_moved_box_pos) = last_moved_box {
                        self.box_pos.insert(Location {
                            row: last_moved_box_pos.row - 1,
                            col: last_moved_box_pos.col,
                        });
                    }
                }
                Movement::Down => {
                    self.robot_pos.row += 1;
                    self.box_pos.remove(&self.robot_pos);
                    if let Some(last_moved_box_pos) = last_moved_box {
                        self.box_pos.insert(Location {
                            row: last_moved_box_pos.row + 1,
                            col: last_moved_box_pos.col,
                        });
                    }
                }
                Movement::Left => {
                    self.robot_pos.col -= 1;
                    self.box_pos.remove(&self.robot_pos);
                    if let Some(last_moved_box_pos) = last_moved_box {
                        self.box_pos.insert(Location {
                            row: last_moved_box_pos.row,
                            col: last_moved_box_pos.col - 1,
                        });
                    }
                }
                Movement::Right => {
                    self.robot_pos.col += 1;
                    self.box_pos.remove(&self.robot_pos);
                    if let Some(last_moved_box_pos) = last_moved_box {
                        self.box_pos.insert(Location {
                            row: last_moved_box_pos.row,
                            col: last_moved_box_pos.col + 1,
                        });
                    }
                }
            };
        }
    }
}

fn parse_movements(movement_input: &str) -> Vec<Movement> {
    let mut movements = Vec::new();

    for movement_char in movement_input.chars() {
        match movement_char {
            '^' => movements.push(Movement::Up),
            '>' => movements.push(Movement::Right),
            'v' => movements.push(Movement::Down),
            '<' => movements.push(Movement::Left),
            _ => (),
        }
    }

    movements
}

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 15!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 15...");

    let input_data = fs::read_to_string("input_data/day15_input.txt")?;
    let (map_input, movements_input) = input_data.split_once("\n\n").unwrap();

    let mut map = Map::from_str(map_input);
    let movements = parse_movements(movements_input);

    for movement in movements.iter() {
        map.move_robot(*movement);
    }

    let gps_coord_sum = map.sum_box_gps_coords();

    let mut solution_file = fs::File::create("solutions/day15_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 15:")?;
    writeln!(
        solution_file,
        "The sum of all boxes' GPS coordinates is {}.",
        gps_coord_sum
    )?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 15...");

    let solution = 0; // TODO

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day15_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 15:")?;
    writeln!(solution_file, "TODO {}.", solution)?;

    Ok(())
}
