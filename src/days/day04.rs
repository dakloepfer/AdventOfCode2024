use std::fs;
use std::io::Error;
use std::io::Write;

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 4!");

    Ok(())
}

#[derive(PartialEq, Clone)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}
impl Direction {
    fn all() -> &'static [Direction] {
        &[
            Direction::Up,
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
            Direction::DownLeft,
            Direction::Left,
            Direction::UpLeft,
        ]
    }
}

fn check_for_next_char(
    word_grid: &[Vec<char>],
    source_row: usize,
    source_col: usize,
    target_char: char,
    direction: Direction,
) -> Result<(bool, usize, usize), Error> {
    let target_row: usize;
    let target_col: usize;
    match direction {
        Direction::Up => {
            if source_row == 0 {
                return Ok((false, source_row, source_col));
            }
            target_row = source_row - 1;
            target_col = source_col;
        }
        Direction::UpRight => {
            if source_row == 0 {
                return Ok((false, source_row, source_col));
            }
            target_row = source_row - 1;
            if source_col == word_grid[target_row].len() - 1 {
                return Ok((false, source_row, source_col));
            }
            target_col = source_col + 1;
        }
        Direction::Right => {
            target_row = source_row;
            if source_col == word_grid[target_row].len() - 1 {
                return Ok((false, source_row, source_col));
            }
            target_col = source_col + 1;
        }
        Direction::DownRight => {
            if source_row == word_grid.len() - 1 {
                return Ok((false, source_row, source_col));
            }
            target_row = source_row + 1;
            if source_col == word_grid[target_row].len() - 1 {
                return Ok((false, source_row, source_col));
            }
            target_col = source_col + 1;
        }
        Direction::Down => {
            if source_row == word_grid.len() - 1 {
                return Ok((false, source_row, source_col));
            }
            target_row = source_row + 1;
            target_col = source_col;
        }
        Direction::DownLeft => {
            if (source_row == word_grid.len() - 1) || (source_col == 0) {
                return Ok((false, source_row, source_col));
            }
            target_row = source_row + 1;
            target_col = source_col - 1;
        }
        Direction::Left => {
            if source_col == 0 {
                return Ok((false, source_row, source_col));
            }
            target_row = source_row;
            target_col = source_col - 1;
        }
        Direction::UpLeft => {
            if (source_row == 0) || (source_col == 0) {
                return Ok((false, source_row, source_col));
            }
            target_row = source_row - 1;
            target_col = source_col - 1;
        }
    }

    if word_grid[target_row][target_col] == target_char {
        Ok((true, target_row, target_col))
    } else {
        Ok((false, target_row, target_col))
    }
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 4...");

    let input_data = fs::read_to_string("input_data/day04_input.txt")?;
    let word_grid: Vec<Vec<char>> = input_data
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let target_word = "XMAS";
    let first_target_char = target_word.chars().next().unwrap();

    let mut num_words: u32 = 0;
    for (row, line) in word_grid.iter().enumerate() {
        for (col, ch) in line.iter().enumerate() {
            if *ch == first_target_char {
                for direction in Direction::all() {
                    let mut next_row: usize = row;
                    let mut next_col: usize = col;
                    let mut success: bool = true;
                    for next_char in target_word.chars().skip(1) {
                        (success, next_row, next_col) = check_for_next_char(
                            &word_grid,
                            next_row,
                            next_col,
                            next_char,
                            direction.clone(),
                        )?;
                        if !success {
                            break;
                        }
                    }
                    if success {
                        num_words += 1;
                    }
                }
            }
        }
    }

    let mut solution_file = fs::File::create("solutions/day04_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 04:")?;
    writeln!(
        solution_file,
        "The word search contains {} instances of {}.",
        num_words, target_word
    )?;

    Ok(())
}
fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 4...");

    let input_data = fs::read_to_string("input_data/day04_input.txt")?;
    let grid: Vec<Vec<char>> = input_data
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut num_words: u32 = 0;
    for (row, line) in grid.iter().enumerate().skip(1).take(grid.len() - 2) {
        for (col, ch) in line.iter().enumerate().skip(1).take(line.len() - 2) {
            if (*ch == 'A')
                & ((((grid[row - 1][col - 1] == 'M') & (grid[row + 1][col + 1] == 'S'))
                    || ((grid[row - 1][col - 1] == 'S') & (grid[row + 1][col + 1] == 'M')))
                    & (((grid[row - 1][col + 1] == 'M') & (grid[row + 1][col - 1] == 'S'))
                        || ((grid[row - 1][col + 1] == 'S') & (grid[row + 1][col - 1] == 'M'))))
            {
                num_words += 1;
            }
        }
    }

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day04_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 04:")?;
    writeln!(
        solution_file,
        "The word search contains {} X-MASes.",
        num_words
    )?;

    Ok(())
}
