mod days;

use days::day01; // Add more days as needed

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
            // Add more days here
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Day {} not implemented", day),
            )),
        };
    }
}
