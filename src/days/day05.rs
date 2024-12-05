use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::io::Write;
use std::io::{Error, ErrorKind};

pub fn run() -> Result<(), Error> {
    let (before_than, invalid_updates) = task1()?;
    let _ = task2(before_than, invalid_updates);

    println!("Completed solutions for Day 4!");

    Ok(())
}

type RuleSet = HashMap<u32, HashSet<u32>>;
type UpdateList = Vec<Vec<u32>>;

fn task1() -> Result<(RuleSet, UpdateList), Error> {
    println!("Computing solution for task 1 of Day 5...");

    let input_data = fs::read_to_string("input_data/day05_input.txt")?;
    let input_parts: Vec<&str> = input_data.split("\n\n").collect();
    let rules = input_parts[0];
    let updates = input_parts[1];

    let mut before_than: HashMap<u32, HashSet<u32>> = HashMap::new(); // for each page, which pages have to come later
    for rule in rules.lines() {
        let (before_str, after_str) = rule.split_once("|").expect("Cannot split rule!");

        let before = before_str.trim().parse::<u32>().map_err(|e| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Failed to parse first part: {}", e),
            )
        })?;
        let after = after_str.trim().parse::<u32>().map_err(|e| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Failed to parse first part: {}", e),
            )
        })?;

        if let Entry::Vacant(e) = before_than.entry(before) {
            e.insert(HashSet::from([after]));
        } else {
            before_than.get_mut(&before).unwrap().insert(after);
        }
    }

    let mut sum_of_middle_pages: u32 = 0;
    let mut invalid_updates: Vec<Vec<u32>> = Vec::new();
    for update in updates.lines() {
        let pages = update
            .split(',')
            .map(|part| part.trim().parse::<u32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                Error::new(
                    ErrorKind::InvalidData,
                    format!("Failed to parse first part: {}", e),
                )
            })?;

        let mut valid = true;
        for (id, page) in pages.iter().enumerate() {
            for prev_page in &pages[..id] {
                if before_than[page].contains(prev_page) {
                    valid = false;
                    break;
                }
            }
            if !valid {
                break;
            }
        }

        if valid {
            sum_of_middle_pages += pages[pages.len() / 2];
        } else {
            invalid_updates.push(pages);
        }
    }

    let mut solution_file = fs::File::create("solutions/day05_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 05:")?;
    writeln!(
        solution_file,
        "The sum of the middle page numbers of all valid updates is {}.",
        sum_of_middle_pages
    )?;

    Ok((before_than, invalid_updates))
}

struct Dag {
    // nodes have u32 IDs
    nodes: HashSet<u32>,
    incoming_edges: HashMap<u32, HashSet<u32>>,
    outgoing_edges: HashMap<u32, HashSet<u32>>,
}
impl Dag {
    pub fn new() -> Dag {
        Dag {
            nodes: HashSet::new(),
            incoming_edges: HashMap::new(),
            outgoing_edges: HashMap::new(),
        }
    }
    pub fn add_edge(&mut self, source: u32, sink: u32) -> Result<(), Error> {
        if !self.nodes.contains(&source) {
            self.nodes.insert(source);
            self.incoming_edges.insert(source, HashSet::new());
            self.outgoing_edges.insert(source, HashSet::from([sink]));
        } else {
            self.outgoing_edges
                .get_mut(&source)
                .expect("Source not in Graph!")
                .insert(sink);
        }
        if !self.nodes.contains(&sink) {
            self.nodes.insert(sink);
            self.incoming_edges.insert(sink, HashSet::from([source]));
            self.outgoing_edges.insert(sink, HashSet::new());
        } else {
            self.incoming_edges
                .get_mut(&sink)
                .expect("Sink not in Graph!")
                .insert(source);
        }

        Ok(())
    }

    pub fn topological_sort(&mut self) -> Result<Vec<u32>, Error> {
        let mut topological_sort: Vec<u32> = Vec::new();
        let mut no_incoming_edges: VecDeque<u32> = VecDeque::new();
        let mut in_degree: HashMap<u32, usize> = HashMap::new();

        for (node, in_edges) in self.incoming_edges.iter() {
            if in_edges.is_empty() {
                in_degree.insert(*node, 0);
                no_incoming_edges.push_back(*node);
            } else {
                in_degree.insert(*node, in_edges.len());
            }
        }

        while let Some(current_node) = no_incoming_edges.pop_front() {
            topological_sort.push(current_node);
            let neighbours = self
                .outgoing_edges
                .get(&current_node)
                .expect("No outgoing edges for current_node");
            for neighbour in neighbours {
                match in_degree[neighbour].cmp(&1) {
                    Ordering::Greater => {
                        in_degree
                            .entry(*neighbour)
                            .and_modify(|value| *value -= 1)
                            .or_insert(0);
                    }
                    Ordering::Equal => {
                        in_degree
                            .entry(*neighbour)
                            .and_modify(|value| *value -= 1)
                            .or_insert(0);
                        no_incoming_edges.push_back(*neighbour);
                    }
                    Ordering::Less => {
                        eprintln!("Found cycle!");
                        // Do nothing (or handle the case explicitly if needed)
                    }
                }
            }
        }

        if topological_sort.len() < self.nodes.len() {
            eprintln!("Graph is not acyclic!");
        }

        Ok(topological_sort)
    }
}

fn task2(before_than: RuleSet, invalid_updates: UpdateList) -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 5...");

    let mut sum_of_middle_pages: u32 = 0;
    for update in invalid_updates {
        let mut graph = Dag::new();
        for page in update.iter() {
            let successor_pages = before_than.get(page).unwrap();
            for other_page in update.iter() {
                if successor_pages.contains(other_page) {
                    let _ = graph.add_edge(*page, *other_page);
                }
            }
        }
        let repaired_update = graph.topological_sort()?;
        sum_of_middle_pages += repaired_update[repaired_update.len() / 2];
    }

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day05_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 05:")?;
    writeln!(
        solution_file,
        "The sum of the middle page numbers of all repaired invalid updates is {}.",
        sum_of_middle_pages
    )?;
    Ok(())
}
