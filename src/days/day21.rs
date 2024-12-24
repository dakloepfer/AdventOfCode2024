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

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum Button {
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
    Up,
    Down,
    Left,
    Right,
}

type CommandSequence = Vec<Button>;

trait Keypad {
    fn button_coords(&self) -> HashMap<Button, Coordinates>;

    fn set_commands_between_buttons(
        &mut self,
        commands_between_buttons: HashMap<(Button, Button), Vec<CommandSequence>>,
    );

    fn compute_commands_between_buttons(&mut self) {
        let mut all_commands_between_all_buttons: HashMap<(Button, Button), Vec<CommandSequence>> =
            HashMap::new();

        for (&start_button, &start_coordinates) in self.button_coords().iter() {
            for (&end_button, &end_coordinates) in self.button_coords().iter() {
                let mut possible_commands: Vec<CommandSequence> = Vec::new();

                // either up/down first or left/right first

                // Up/Down first
                let mut updown_first_commands: CommandSequence = Vec::new();

                if end_coordinates.y > start_coordinates.y {
                    updown_first_commands.extend(vec![
                        Button::Up;
                        (end_coordinates.y - start_coordinates.y)
                            as usize
                    ]);
                }
                if start_coordinates.y > end_coordinates.y {
                    updown_first_commands.extend(vec![
                        Button::Down;
                        (start_coordinates.y - end_coordinates.y)
                            as usize
                    ]);
                }
                if start_coordinates.x > end_coordinates.x {
                    updown_first_commands.extend(vec![
                        Button::Left;
                        (start_coordinates.x - end_coordinates.x)
                            as usize
                    ]);
                }
                if end_coordinates.x > start_coordinates.x {
                    updown_first_commands.extend(vec![
                        Button::Right;
                        (end_coordinates.x - start_coordinates.x)
                            as usize
                    ]);
                }
                updown_first_commands.push(Button::Activate);

                // special cases to avoid gap:
                if !(start_button == Button::One && start_coordinates.y > end_coordinates.y
                    || start_button == Button::Four
                        && start_coordinates.y.saturating_sub(end_coordinates.y) > 1
                    || start_button == Button::Seven
                        && start_coordinates.y.saturating_sub(end_coordinates.y) > 2
                    || start_button == Button::Left && start_coordinates.y < end_coordinates.y)
                {
                    possible_commands.push(updown_first_commands);
                }

                // Left/Right first
                let mut leftright_first_commands: CommandSequence = Vec::new();

                if start_coordinates.x > end_coordinates.x {
                    leftright_first_commands.extend(vec![
                        Button::Left;
                        (start_coordinates.x - end_coordinates.x)
                            as usize
                    ]);
                }
                if end_coordinates.x > start_coordinates.x {
                    leftright_first_commands.extend(vec![
                        Button::Right;
                        (end_coordinates.x - start_coordinates.x)
                            as usize
                    ]);
                }
                if end_coordinates.y > start_coordinates.y {
                    leftright_first_commands.extend(vec![
                        Button::Up;
                        (end_coordinates.y - start_coordinates.y)
                            as usize
                    ]);
                }
                if start_coordinates.y > end_coordinates.y {
                    leftright_first_commands.extend(vec![
                        Button::Down;
                        (start_coordinates.y - end_coordinates.y)
                            as usize
                    ]);
                }
                leftright_first_commands.push(Button::Activate);

                // special cases to avoid gap:
                if !(start_button == Button::Activate
                    && start_coordinates.x.saturating_sub(end_coordinates.x) > 1
                    || start_button == Button::Zero && start_coordinates.x > end_coordinates.x
                    || start_button == Button::Up && start_coordinates.x > end_coordinates.x)
                {
                    possible_commands.push(leftright_first_commands);
                }

                all_commands_between_all_buttons
                    .insert((start_button, end_button), possible_commands);
            }
        }
        self.set_commands_between_buttons(all_commands_between_all_buttons);
    }
}

struct NumericKeypad {
    button_coords: HashMap<Button, Coordinates>,
    commands_between_buttons: HashMap<(Button, Button), Vec<CommandSequence>>,
}
impl NumericKeypad {
    fn new() -> NumericKeypad {
        let mut button_coords = HashMap::new();

        button_coords.insert(Button::Activate, Coordinates { x: 2, y: 0 });
        button_coords.insert(Button::Zero, Coordinates { x: 1, y: 0 });
        button_coords.insert(Button::One, Coordinates { x: 0, y: 1 });
        button_coords.insert(Button::Two, Coordinates { x: 1, y: 1 });
        button_coords.insert(Button::Three, Coordinates { x: 2, y: 1 });
        button_coords.insert(Button::Four, Coordinates { x: 0, y: 2 });
        button_coords.insert(Button::Five, Coordinates { x: 1, y: 2 });
        button_coords.insert(Button::Six, Coordinates { x: 2, y: 2 });
        button_coords.insert(Button::Seven, Coordinates { x: 0, y: 3 });
        button_coords.insert(Button::Eight, Coordinates { x: 1, y: 3 });
        button_coords.insert(Button::Nine, Coordinates { x: 2, y: 3 });

        let mut new_keypad = NumericKeypad {
            button_coords,
            commands_between_buttons: HashMap::new(),
        };
        new_keypad.compute_commands_between_buttons();
        new_keypad
    }
}
impl Keypad for NumericKeypad {
    fn button_coords(&self) -> HashMap<Button, Coordinates> {
        self.button_coords.clone()
    }

    fn set_commands_between_buttons(
        &mut self,
        commands_between_buttons: HashMap<(Button, Button), Vec<CommandSequence>>,
    ) {
        self.commands_between_buttons = commands_between_buttons;
    }
}

#[derive(Clone)]
struct DirectionalKeypad {
    button_coords: HashMap<Button, Coordinates>,
    commands_between_buttons: HashMap<(Button, Button), Vec<CommandSequence>>,
}
impl DirectionalKeypad {
    fn new() -> DirectionalKeypad {
        let mut button_coords = HashMap::new();

        button_coords.insert(Button::Activate, Coordinates { x: 2, y: 1 });
        button_coords.insert(Button::Up, Coordinates { x: 1, y: 1 });
        button_coords.insert(Button::Left, Coordinates { x: 0, y: 0 });
        button_coords.insert(Button::Down, Coordinates { x: 1, y: 0 });
        button_coords.insert(Button::Right, Coordinates { x: 2, y: 0 });

        let mut new_keypad = DirectionalKeypad {
            button_coords,
            commands_between_buttons: HashMap::new(),
        };
        new_keypad.compute_commands_between_buttons();
        new_keypad
    }
}
impl Keypad for DirectionalKeypad {
    fn button_coords(&self) -> HashMap<Button, Coordinates> {
        self.button_coords.clone()
    }

    fn set_commands_between_buttons(
        &mut self,
        commands_between_buttons: HashMap<(Button, Button), Vec<CommandSequence>>,
    ) {
        self.commands_between_buttons = commands_between_buttons;
    }
}

struct RecursionData {
    memoization: HashMap<u32, HashMap<(Button, Button), u64>>,
    previous_button_at_level: HashMap<u32, Button>,
}

fn recurse_shortest_between_buttons(
    start_button: Button,
    end_button: Button,
    current_level: u32,
    max_level: u32,
    numeric_keypad: &NumericKeypad,
    directional_keypad: &DirectionalKeypad,
    recursion_data: &mut RecursionData,
) -> u64 {
    if current_level == max_level {
        return 1; // at final level, can just press the button
    }
    if let Some(current_level_path_cost) = recursion_data.memoization.get(&current_level) {
        if let Some(cost) = current_level_path_cost.get(&(start_button, end_button)) {
            return *cost;
        }
    }

    let possible_paths = if current_level == 0 {
        numeric_keypad
            .commands_between_buttons
            .get(&(start_button, end_button))
            .unwrap()
    } else {
        directional_keypad
            .commands_between_buttons
            .get(&(start_button, end_button))
            .unwrap()
    };

    let previous_button = *recursion_data
        .previous_button_at_level
        .entry(current_level)
        .or_insert(Button::Activate);

    let mut updated_previous_button = previous_button;
    let mut total_length = u64::MAX;

    for possible_path in possible_paths.iter() {
        let mut current_total = 0;
        let mut current_previous_button = previous_button;

        for current_button in possible_path.iter() {
            let added_cost = recurse_shortest_between_buttons(
                current_previous_button,
                *current_button,
                current_level + 1,
                max_level,
                numeric_keypad,
                directional_keypad,
                recursion_data,
            );
            current_previous_button = *current_button;
            current_total += added_cost;
        }
        if current_total < total_length {
            total_length = current_total;
            updated_previous_button = *possible_path.last().unwrap();
        }
    }

    recursion_data
        .previous_button_at_level
        .insert(current_level, updated_previous_button);
    let saved_path = recursion_data.memoization.entry(current_level).or_default();
    saved_path.insert((start_button, end_button), total_length);

    total_length
}

fn min_control_sequence_length(code: Vec<Button>, num_directional_keypads: u32) -> u64 {
    let numeric_keypad = NumericKeypad::new();
    let directional_keypad = DirectionalKeypad::new();

    let mut total_length = 0;
    let mut previous_button = Button::Activate;

    for &button in code.iter() {
        let mut recursion_data = RecursionData {
            memoization: HashMap::new(),
            previous_button_at_level: HashMap::new(),
        };
        total_length += recurse_shortest_between_buttons(
            previous_button,
            button,
            0,
            num_directional_keypads,
            &numeric_keypad,
            &directional_keypad,
            &mut recursion_data,
        );
        previous_button = button
    }

    total_length
}

fn string_to_numeric_code(input: &str) -> Vec<Button> {
    let mut numeric_code = Vec::new();

    for char in input.trim().chars() {
        let button = match char {
            'A' => Button::Activate,
            '0' => Button::Zero,
            '1' => Button::One,
            '2' => Button::Two,
            '3' => Button::Three,
            '4' => Button::Four,
            '5' => Button::Five,
            '6' => Button::Six,
            '7' => Button::Seven,
            '8' => Button::Eight,
            '9' => Button::Nine,
            _ => unreachable!(),
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

    let mut total_complexity: u64 = 0;
    for line in input_data.lines() {
        let code: Vec<Button> = string_to_numeric_code(line);
        let command_length = min_control_sequence_length(code, 3);
        total_complexity += command_length * line[..line.len() - 1].parse::<u64>().ok().unwrap();
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

    let input_data = fs::read_to_string("input_data/day21_input.txt")?;

    let mut total_complexity: u64 = 0;
    for line in input_data.lines() {
        let code: Vec<Button> = string_to_numeric_code(line);
        let command_length = min_control_sequence_length(code, 26);
        total_complexity += command_length * line[..line.len() - 1].parse::<u64>().ok().unwrap();
    }

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day21_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 21:")?;
    writeln!(
        solution_file,
        "The total complexity of entering the code to the door via 25 directional keypads is {}.",
        total_complexity
    )?;

    Ok(())
}
