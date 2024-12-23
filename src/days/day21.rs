// TODO I think I need to actively optimise, I don't think I can derive a simple rule to get the shortest path that is valid for all levels.
// So, return all possible shortest paths (only two each time, either up/down before or after left/right). This has a branching factor of two, but I can memoize the shortest paths to use for each level.

use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::io::Error;
use std::io::Write;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Coordinates {
    // from bottom left, to the right and up
    x: u8,
    y: u8,
}

trait Button: Eq + Hash + Clone + Copy {
    const ACTIVATE: Self;
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum NumericButton {
    Activate,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}
impl Button for NumericButton {
    const ACTIVATE: Self = NumericButton::Activate;
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum DirectionalButton {
    Activate,
    Up,
    Down,
    Left,
    Right,
}
impl Button for DirectionalButton {
    const ACTIVATE: Self = DirectionalButton::Activate;
}

type CommandSequence = Vec<DirectionalButton>;

trait Keypad<B: Button> {
    fn commands_between_buttons(&self) -> HashMap<B, HashMap<B, CommandSequence>>;
}

struct NumericKeypad {
    button_coords: HashMap<NumericButton, Coordinates>,
    commands_between_buttons:
        HashMap<NumericButton, HashMap<NumericButton, Vec<DirectionalButton>>>,
}
impl NumericKeypad {
    fn new() -> NumericKeypad {
        let mut button_coords = HashMap::new();

        button_coords.insert(NumericButton::Activate, Coordinates { x: 2, y: 0 });
        button_coords.insert(NumericButton::Zero, Coordinates { x: 1, y: 0 });
        button_coords.insert(NumericButton::One, Coordinates { x: 0, y: 1 });
        button_coords.insert(NumericButton::Two, Coordinates { x: 1, y: 1 });
        button_coords.insert(NumericButton::Three, Coordinates { x: 2, y: 1 });
        button_coords.insert(NumericButton::Four, Coordinates { x: 0, y: 2 });
        button_coords.insert(NumericButton::Five, Coordinates { x: 1, y: 2 });
        button_coords.insert(NumericButton::Six, Coordinates { x: 2, y: 2 });
        button_coords.insert(NumericButton::Seven, Coordinates { x: 0, y: 3 });
        button_coords.insert(NumericButton::Eight, Coordinates { x: 1, y: 3 });
        button_coords.insert(NumericButton::Nine, Coordinates { x: 2, y: 3 });

        let mut new_keypad = NumericKeypad {
            button_coords,
            commands_between_buttons: HashMap::new(),
        };
        new_keypad.compute_commands_between_buttons();
        new_keypad
    }
    fn compute_commands_between_buttons(&mut self) {
        let mut all_commands_between_all_buttons: HashMap<
            NumericButton,
            HashMap<NumericButton, CommandSequence>,
        > = HashMap::new();

        for (&start_button, &start_coordinates) in self.button_coords.iter() {
            let mut all_commands: HashMap<NumericButton, CommandSequence> = HashMap::new();
            for (&end_button, &end_coordinates) in self.button_coords.iter() {
                let mut commands: CommandSequence = Vec::new();

                // I verified for this directional button arrangement: Do the directions with buttons further away from the Activation button first (subject to avoiding gaps). (Here this means Left, then Down, then Right and Up are equivalent)

                // special cases to avoid gap:
                if (start_button == NumericButton::Activate
                    && start_coordinates.x.saturating_sub(end_coordinates.x) > 1)
                    || (start_button == NumericButton::Zero
                        && start_coordinates.x > end_coordinates.x)
                {
                    // do left last
                    commands.extend(vec![
                        DirectionalButton::Up;
                        (end_coordinates.y - start_coordinates.y) as usize
                    ]);
                    // bottom row, so has to go up
                    commands.extend(vec![
                        DirectionalButton::Left;
                        (start_coordinates.x - end_coordinates.x) as usize
                    ]);
                    commands.push(DirectionalButton::Activate);
                    all_commands.insert(end_button, commands);
                    continue;
                } else if (start_button == NumericButton::One
                    && start_coordinates.y > end_coordinates.y)
                    || (start_button == NumericButton::Four
                        && start_coordinates.y.saturating_sub(end_coordinates.y) > 1)
                    || (start_button == NumericButton::Seven
                        && start_coordinates.y.saturating_sub(end_coordinates.y) > 2)
                {
                    // do right before down; needs to be right because we're in left-most column
                    commands.extend(vec![
                        DirectionalButton::Right;
                        (end_coordinates.x - start_coordinates.x) as usize
                    ]);
                    commands.extend(vec![
                        DirectionalButton::Down;
                        (start_coordinates.y - end_coordinates.y) as usize
                    ]);
                    commands.push(DirectionalButton::Activate);
                    all_commands.insert(end_button, commands);
                    continue;
                }

                if start_coordinates.x > end_coordinates.x {
                    commands.extend(vec![
                        DirectionalButton::Left;
                        (start_coordinates.x - end_coordinates.x) as usize
                    ]);
                }
                if start_coordinates.y > end_coordinates.y {
                    commands.extend(vec![
                        DirectionalButton::Down;
                        (start_coordinates.y - end_coordinates.y) as usize
                    ]);
                }
                if end_coordinates.x > start_coordinates.x {
                    commands.extend(vec![
                        DirectionalButton::Right;
                        (end_coordinates.x - start_coordinates.x) as usize
                    ]);
                }
                if end_coordinates.y > start_coordinates.y {
                    commands.extend(vec![
                        DirectionalButton::Up;
                        (end_coordinates.y - start_coordinates.y) as usize
                    ]);
                }

                commands.push(DirectionalButton::Activate);
                all_commands.insert(end_button, commands);
            }
            all_commands_between_all_buttons.insert(start_button, all_commands);
        }
        self.commands_between_buttons = all_commands_between_all_buttons;
    }
}
impl Keypad<NumericButton> for NumericKeypad {
    fn commands_between_buttons(
        &self,
    ) -> HashMap<NumericButton, HashMap<NumericButton, CommandSequence>> {
        self.commands_between_buttons.clone()
    }
}

struct DirectionalKeypad {
    button_coords: HashMap<DirectionalButton, Coordinates>,
    commands_between_buttons:
        HashMap<DirectionalButton, HashMap<DirectionalButton, Vec<DirectionalButton>>>,
}
impl DirectionalKeypad {
    fn new() -> DirectionalKeypad {
        let mut button_coords = HashMap::new();

        button_coords.insert(DirectionalButton::Activate, Coordinates { x: 2, y: 1 });
        button_coords.insert(DirectionalButton::Up, Coordinates { x: 1, y: 1 });
        button_coords.insert(DirectionalButton::Left, Coordinates { x: 0, y: 0 });
        button_coords.insert(DirectionalButton::Down, Coordinates { x: 1, y: 0 });
        button_coords.insert(DirectionalButton::Right, Coordinates { x: 2, y: 0 });

        let mut new_keypad = DirectionalKeypad {
            button_coords,
            commands_between_buttons: HashMap::new(),
        };
        new_keypad.compute_commands_between_buttons();
        new_keypad
    }

    fn compute_commands_between_buttons(&mut self) {
        let mut all_commands_between_all_buttons: HashMap<
            DirectionalButton,
            HashMap<DirectionalButton, CommandSequence>,
        > = HashMap::new();

        for (&start_button, &start_coordinates) in self.button_coords.iter() {
            let mut all_commands: HashMap<DirectionalButton, CommandSequence> = HashMap::new();
            for (&end_button, &end_coordinates) in self.button_coords.iter() {
                let mut commands: CommandSequence = Vec::new();

                // I verified for this directional button arrangement: Do the directions with buttons further away from the Activation button first (subject to avoiding gaps). (Here this means Left, then Down, then Right and Up are equivalent)

                // special cases to avoid gap:
                if (start_button == DirectionalButton::Activate
                    && start_coordinates.x.saturating_sub(end_coordinates.x) > 1)
                    || (start_button == DirectionalButton::Up
                        && start_coordinates.x > end_coordinates.x)
                {
                    // do left last
                    // bottom row, so has to go up
                    commands.extend(vec![
                        DirectionalButton::Down;
                        (start_coordinates.y - end_coordinates.y) as usize
                    ]);
                    commands.extend(vec![
                        DirectionalButton::Left;
                        (start_coordinates.x - end_coordinates.x) as usize
                    ]);
                    commands.push(DirectionalButton::Activate);
                    all_commands.insert(end_button, commands);
                    continue;
                }

                if start_coordinates.x > end_coordinates.x {
                    commands.extend(vec![
                        DirectionalButton::Left;
                        (start_coordinates.x - end_coordinates.x) as usize
                    ]);
                }
                if start_coordinates.y > end_coordinates.y {
                    commands.extend(vec![
                        DirectionalButton::Down;
                        (start_coordinates.y - end_coordinates.y) as usize
                    ]);
                }
                if end_coordinates.x > start_coordinates.x {
                    commands.extend(vec![
                        DirectionalButton::Right;
                        (end_coordinates.x - start_coordinates.x) as usize
                    ]);
                }
                if end_coordinates.y > start_coordinates.y {
                    commands.extend(vec![
                        DirectionalButton::Up;
                        (end_coordinates.y - start_coordinates.y) as usize
                    ]);
                }

                commands.push(DirectionalButton::Activate);
                all_commands.insert(end_button, commands);
            }
            all_commands_between_all_buttons.insert(start_button, all_commands);
        }
        self.commands_between_buttons = all_commands_between_all_buttons;
    }
}
impl Keypad<DirectionalButton> for DirectionalKeypad {
    fn commands_between_buttons(
        &self,
    ) -> HashMap<DirectionalButton, HashMap<DirectionalButton, CommandSequence>> {
        self.commands_between_buttons.clone()
    }
}

fn control_keypad<B: Button, K: Keypad<B>>(
    target_keypad: &K,
    button_sequence: Vec<B>,
) -> CommandSequence {
    let mut command_sequence = Vec::new();

    for (button_idx, next_button) in button_sequence.iter().enumerate() {
        let current_button = if button_idx == 0 {
            B::ACTIVATE
        } else {
            button_sequence[button_idx - 1]
        };

        command_sequence.extend(
            target_keypad
                .commands_between_buttons()
                .get(&current_button)
                .unwrap()
                .get(next_button)
                .unwrap(),
        )
    }

    command_sequence
}

fn string_to_numeric_code(input: &str) -> Vec<NumericButton> {
    let mut numeric_code = Vec::new();

    for char in input.trim().chars() {
        let button = match char {
            'A' => NumericButton::Activate,
            '0' => NumericButton::Zero,
            '1' => NumericButton::One,
            '2' => NumericButton::Two,
            '3' => NumericButton::Three,
            '4' => NumericButton::Four,
            '5' => NumericButton::Five,
            '6' => NumericButton::Six,
            '7' => NumericButton::Seven,
            '8' => NumericButton::Eight,
            '9' => NumericButton::Nine,
            _ => NumericButton::Activate,
        };
        numeric_code.push(button);
    }

    numeric_code
}

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 21!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 21...");

    let input_data = fs::read_to_string("input_data/day21_input.txt")?;

    let numeric_keypad = NumericKeypad::new();
    let directional_keypad1 = DirectionalKeypad::new();
    let directional_keypad2 = DirectionalKeypad::new();

    let mut total_complexity: u32 = 0;
    for line in input_data.lines() {
        let code: Vec<NumericButton> = string_to_numeric_code(line);
        let commands1: CommandSequence = control_keypad(&numeric_keypad, code);
        let commands2: CommandSequence = control_keypad(&directional_keypad1, commands1);
        let commands3: CommandSequence = control_keypad(&directional_keypad2, commands2);
        total_complexity +=
            commands3.len() as u32 * line[..line.len() - 1].parse::<u32>().ok().unwrap();
    }

    let mut solution_file = fs::File::create("solutions/day21_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 21:")?;
    writeln!(
        solution_file,
        "The total complexity of entering the code to the door is {}.",
        total_complexity
    )?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 21...");

    let solution = 0; // TODO

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day21_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 21:")?;
    writeln!(solution_file, "TODO {}.", solution)?;

    Ok(())
}
