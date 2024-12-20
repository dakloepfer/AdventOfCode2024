use std::fs;
use std::io::Error;
use std::io::Write;

#[derive(Clone, Copy)]
struct ContiguousBlock {
    start: u64,
    end: u64,
}

fn parse_diskmap(diskmap: Vec<u32>) -> (Vec<ContiguousBlock>, Vec<ContiguousBlock>) {
    let mut empty_blocks = Vec::new();
    let mut file_locations = Vec::new();

    let mut current_location: u64 = 0;
    for (block_num, length) in diskmap.iter().enumerate() {
        if block_num % 2 == 0 {
            // file
            file_locations.push(ContiguousBlock {
                start: current_location,
                end: current_location + *length as u64,
            });
            current_location += *length as u64;
        } else {
            // empty space
            empty_blocks.push(ContiguousBlock {
                start: current_location,
                end: current_location + *length as u64,
            });
            current_location += *length as u64;
        }
    }

    (empty_blocks, file_locations)
}

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 9!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 9...");

    let input_data = fs::read_to_string("input_data/day09_input.txt")?;
    let diskmap: Vec<u32> = input_data
        .chars()
        .map(|c| {
            c.to_digit(10)
                .expect("Input string contains non-digit characters")
        })
        .collect();

    let (mut empty_blocks, mut file_locations) = parse_diskmap(diskmap);
    let mut checksum = 0;

    let mut file_idx = file_locations.len() - 1;
    let mut empty_space_idx: usize = 0;

    loop {
        if empty_blocks[empty_space_idx].start == empty_blocks[empty_space_idx].end {
            empty_space_idx += 1;
            continue;
        }
        if file_locations[file_idx].start == file_locations[file_idx].end {
            file_idx -= 1;
            continue;
        }
        if (empty_space_idx == empty_blocks.len())
            || (empty_blocks[empty_space_idx].start >= file_locations[file_idx].end)
        {
            // sum up all the remaining file blocks
            for (file_id, &block) in file_locations[..file_idx + 1].iter().enumerate() {
                checksum += (((block.start + block.end - 1) * (block.end - block.start)) / 2)
                    * file_id as u64;
            }
            break;
        }
        if file_idx == 0 {
            break;
        }
        checksum += empty_blocks[empty_space_idx].start * file_idx as u64;
        empty_blocks[empty_space_idx].start += 1;
        file_locations[file_idx].end -= 1;
    }

    let mut solution_file = fs::File::create("solutions/day09_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 09:")?;
    writeln!(
        solution_file,
        "The checksum after moving all the files is {}.",
        checksum
    )?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 9...");

    let input_data = fs::read_to_string("input_data/day09_input.txt")?;
    let diskmap: Vec<u32> = input_data
        .chars()
        .map(|c| {
            c.to_digit(10)
                .expect("Input string contains non-digit characters")
        })
        .collect();

    let (mut empty_blocks, file_locations) = parse_diskmap(diskmap);
    let mut checksum: u64 = 0;

    let mut file_idx = file_locations.len() - 1;

    while file_idx > 0 {
        let file_block = file_locations[file_idx];
        let file_length = file_block.end - file_block.start;

        for empty_block in empty_blocks.iter_mut() {
            if empty_block.start >= file_block.end {
                // file stays in same location
                checksum += file_idx as u64
                    * (((file_block.start + file_block.end - 1)
                        * (file_block.end - file_block.start))
                        / 2);
                break;
            }
            if empty_block.end - empty_block.start >= file_length {
                checksum += file_idx as u64
                    * (((empty_block.start + empty_block.start + file_length - 1) * file_length)
                        / 2);
                empty_block.start += file_length;
                break;
            }
        }

        file_idx -= 1;
    }

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day09_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 09:")?;
    writeln!(
        solution_file,
        "The checksum after moving the files contiguously is {}.",
        checksum
    )?;

    Ok(())
}
