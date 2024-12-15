use std::fs;
use std::io::Error;
use std::io::Write;
use std::io::{stdin, stdout};

#[derive(Clone, Copy)]
struct Robot {
    pos_x: u32,
    pos_y: u32,
    vel_x: i32,
    vel_y: i32,
}
impl Robot {
    fn from_config(config: &str) -> Robot {
        let (mut pos, mut vel) = config.split_once(" ").unwrap();
        pos = &pos[2..];
        vel = &vel[2..];

        let (pos_x, pos_y) = pos.split_once(",").unwrap();
        let (vel_x, vel_y) = vel.split_once(",").unwrap();

        Robot {
            pos_x: pos_x.parse().ok().unwrap(),
            pos_y: pos_y.parse().ok().unwrap(),
            vel_x: vel_x.parse().ok().unwrap(),
            vel_y: vel_y.parse().ok().unwrap(),
        }
    }
}

enum Quadrant {
    TopRight,
    TopLeft,
    BottomLeft,
    BottomRight,
    Middle,
}

fn simulate_robot_movement(robot: &mut Robot, time_elapsed: i64, map_height: u32, map_width: u32) {
    let delta_x =
        ((robot.vel_x as i64 * time_elapsed) % map_width as i64 + map_width as i64) as u32;
    let delta_y =
        ((robot.vel_y as i64 * time_elapsed) % map_height as i64 + map_height as i64) as u32;

    robot.pos_x = (robot.pos_x + delta_x) % map_width;
    robot.pos_y = (robot.pos_y + delta_y) % map_height;
}

fn get_robot_quadrant(robot: Robot, map_height: u32, map_width: u32) -> Quadrant {
    let lowest_top = map_height / 2 - 1;
    let highest_bottom = (map_height + 1) / 2;

    let rightest_left = map_width / 2 - 1;
    let leftest_right = (map_width + 1) / 2;

    if (robot.pos_x >= leftest_right) && (robot.pos_y <= lowest_top) {
        Quadrant::TopRight
    } else if (robot.pos_x <= rightest_left) && (robot.pos_y <= lowest_top) {
        Quadrant::TopLeft
    } else if (robot.pos_x <= rightest_left) && (robot.pos_y >= highest_bottom) {
        Quadrant::BottomLeft
    } else if (robot.pos_x >= leftest_right) && (robot.pos_y >= highest_bottom) {
        Quadrant::BottomRight
    } else {
        Quadrant::Middle
    }
}

fn print_state(
    robots: &Vec<Robot>,
    map_height: u32,
    map_width: u32,
    filepath: &str,
    prefix: &str,
) -> Result<(), Error> {
    let mut map_state = vec![vec!['.'; map_width as usize]; map_height as usize];

    for robot in robots {
        map_state[robot.pos_y as usize][robot.pos_x as usize] = 'R';
    }
    let lines: Vec<String> = map_state
        .iter()
        .map(|chars| chars.iter().collect::<String>())
        .collect();
    let image = lines.join("\n");

    let mut print_file = fs::File::create(filepath)?;
    writeln!(print_file, "{}", prefix)?;
    writeln!(print_file, "{}", image)?;

    Ok(())
}

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 14!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 14...");

    let input_data = fs::read_to_string("input_data/day14_input.txt")?;

    let mut num_topright_quadrant: u32 = 0;
    let mut num_topleft_quadrant: u32 = 0;
    let mut num_bottomleft_quadrant: u32 = 0;
    let mut num_bottomright_quadrant: u32 = 0;

    let map_height = 103;
    let map_width = 101;
    let simulation_time = 100;

    for robot_config in input_data.lines() {
        let mut robot = Robot::from_config(robot_config);
        simulate_robot_movement(&mut robot, simulation_time, map_height, map_width);

        match get_robot_quadrant(robot, map_height, map_width) {
            Quadrant::TopRight => num_topright_quadrant += 1,
            Quadrant::TopLeft => num_topleft_quadrant += 1,
            Quadrant::BottomLeft => num_bottomleft_quadrant += 1,
            Quadrant::BottomRight => num_bottomright_quadrant += 1,
            Quadrant::Middle => {}
        }
    }

    let safety_factor = num_topright_quadrant
        * num_topleft_quadrant
        * num_bottomleft_quadrant
        * num_bottomright_quadrant;

    let mut solution_file = fs::File::create("solutions/day14_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 14:")?;
    writeln!(
        solution_file,
        "The safety factor after exactly 100 seconds is {}.",
        safety_factor
    )?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 14...");

    let input_data = fs::read_to_string("input_data/day14_input.txt")?;
    let map_height = 103;
    let map_width = 101;

    let mut robots: Vec<Robot> = Vec::new();
    for robot_config in input_data.lines() {
        robots.push(Robot::from_config(robot_config));
    }

    let mut user_command = String::new();
    let mut time_elapsed = 0;
    let delta_t = 1;

    for robot in robots.iter_mut() {
        simulate_robot_movement(robot, time_elapsed, map_height, map_width);
    }
    loop {
        let prefix = format!("Map State after {}s", time_elapsed);
        print_state(
            &robots,
            map_height,
            map_width,
            "scratch/day14_task2_visualisation.txt",
            &prefix,
        )?;

        println!("If you want to quit, please enter 'q'.");
        stdout().flush().unwrap();

        stdin()
            .read_line(&mut user_command)
            .expect("Failed to read input.");

        if user_command.trim() == "q" {
            break;
        }
        for robot in robots.iter_mut() {
            simulate_robot_movement(robot, delta_t, map_height, map_width);
        }
        time_elapsed += delta_t;
    }

    // There are some horizontal patterns at t = 16 + 103 * n, n=0,1,2,... and some vertical patterns at t = 71 + 101 * m, m=0,1,2,...
    // This suggests (which was easily confirmed) that at the first timestep where both those patterns coincide, a stronger pattern (of the Christmas tree) appears.
    // We have 16 + 103n = 71 + 101m => 55 = 103n - 101m => 55 = 2n - 101d for m = n+d => LHS first becomes odd for d = 1, and n = 78, so at t = 8050. This is indeed the correct timestep.
    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day14_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 14:")?;
    writeln!(
        solution_file,
        "The robots look like a Christmas Tree after 8050 seconds."
    )?;

    Ok(())
}
