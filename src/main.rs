mod days;

use days::day01;
use days::day02;
use days::day03;
use days::day04;
use days::day05;
use days::day06;
use days::day07;
use days::day08;
use days::day09;
use days::day10;
use days::day11;
use days::day12;
use days::day13;
use days::day14;
use days::day15;
use days::day16;
use days::day17;
use days::day18;
use days::day19;
use days::day20;
use days::day21;
use days::day22;
use days::day23;
use days::day24;
use days::day25;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: advent_of_code <day1> [day2] [day3] ...");
        return;
    }

    let days_to_run: Vec<u32> = args[1..]
        .iter()
        .filter_map(|arg| arg.parse::<u32>().ok())
        .collect();

    if days_to_run.is_empty() {
        eprintln!("No valid days specified.");
        return;
    }

    for day in days_to_run {
        let _ = match day {
            1 => day01::run(),
            2 => day02::run(),
            3 => day03::run(),
            4 => day04::run(),
            5 => day05::run(),
            6 => day06::run(),
            7 => day07::run(),
            8 => day08::run(),
            9 => day09::run(),
            10 => day10::run(),
            11 => day11::run(),
            12 => day12::run(),
            13 => day13::run(),
            14 => day14::run(),
            15 => day15::run(),
            16 => day16::run(),
            17 => day17::run(),
            18 => day18::run(),
            19 => day19::run(),
            20 => day20::run(),
            21 => day21::run(),
            22 => day22::run(),
            23 => day23::run(),
            24 => day24::run(),
            25 => day25::run(),

            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Day {} not implemented", day),
            )),
        };
    }
}
