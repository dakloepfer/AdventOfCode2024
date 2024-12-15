use std::fs;
use std::io::Error;
use std::io::Write;

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

fn simulate_robot_movement(
    robot: Robot,
    time_elapsed: i64,
    map_height: u32,
    map_width: u32,
) -> Robot {
    let delta_x =
        ((robot.vel_x as i64 * time_elapsed) % map_width as i64 + map_width as i64) as u32;
    let delta_y =
        ((robot.vel_y as i64 * time_elapsed) % map_height as i64 + map_height as i64) as u32;

    Robot {
        pos_x: (robot.pos_x + delta_x) % map_width,
        pos_y: (robot.pos_y + delta_y) % map_height,
        vel_x: robot.vel_x,
        vel_y: robot.vel_y,
    }
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
        let robot = Robot::from_config(robot_config);
        let new_robot = simulate_robot_movement(robot, simulation_time, map_height, map_width);

        match get_robot_quadrant(new_robot, map_height, map_width) {
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

    let solution = 0; // TODO

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day14_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 14:")?;
    writeln!(solution_file, "TODO {}.", solution)?;

    Ok(())
}
