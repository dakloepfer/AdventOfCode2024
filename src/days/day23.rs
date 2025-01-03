use std::collections::{HashMap, HashSet};
use std::fs;

use std::io::Error;
use std::io::Write;

type Node = String;
type Triangle = Vec<Node>;

struct Graph {
    nodes: HashSet<Node>,
    neighbours: HashMap<Node, HashSet<Node>>,
}
impl Graph {
    fn from_input(input: String) -> Graph {
        let mut nodes = HashSet::new();
        let mut neighbours: HashMap<String, HashSet<String>> = HashMap::new();

        for edge_string in input.lines() {
            let (node1_str, node2_str) = edge_string.trim().split_once('-').unwrap();
            let node1 = node1_str.to_string();
            let node2 = node2_str.to_string();
            nodes.insert(node1.clone());
            nodes.insert(node2.clone());

            neighbours
                .entry(node1.clone())
                .or_default()
                .insert(node2.clone());
            neighbours.entry(node2).or_default().insert(node1);
        }

        Graph { nodes, neighbours }
    }
}

fn adjacency_list_intersection(
    neighbours1: &HashSet<Node>,
    neighbours2: &HashSet<Node>,
) -> HashSet<Node> {
    let mut intersection: HashSet<Node> = HashSet::new();

    if neighbours1.len() < neighbours2.len() {
        for node in neighbours1.iter() {
            if neighbours2.contains(node) {
                intersection.insert(node.to_string());
            }
        }
    } else {
        for node in neighbours2.iter() {
            if neighbours1.contains(node) {
                intersection.insert(node.to_string());
            }
        }
    }

    intersection
}

fn get_all_triangles(graph: Graph) -> HashSet<Triangle> {
    let mut all_triangles = HashSet::new();
    let mut checked_node_pairs = HashSet::new();
    for node1 in graph.nodes.iter() {
        for node2 in graph.nodes.iter() {
            if !graph.neighbours.get(node1).unwrap().contains(node2) {
                continue;
            }

            if checked_node_pairs.contains(&(node1, node2)) {
                continue;
            } else {
                checked_node_pairs.insert((node1, node2));
                checked_node_pairs.insert((node2, node1));
            }

            let node1_neighbours = graph.neighbours.get(node1).unwrap();
            let node2_neighbours = graph.neighbours.get(node2).unwrap();

            let neighbour_intersection =
                adjacency_list_intersection(node1_neighbours, node2_neighbours);

            for node3 in neighbour_intersection.iter() {
                // sort
                let mut triangle = vec![node1.clone(), node2.clone(), node3.clone()];
                triangle.sort();
                all_triangles.insert(triangle);
            }
        }
    }

    all_triangles
}

/// assumes max clique is unique
fn get_max_clique(graph: &Graph) -> HashSet<Node> {
    recurse_bron_kerbosch(
        graph,
        HashSet::new(),
        graph.nodes.clone(),
        HashSet::new(),
        HashSet::new(),
    )
}

fn recurse_bron_kerbosch(
    graph: &Graph,
    current_clique: HashSet<Node>,
    candidates: HashSet<Node>,
    excluded: HashSet<Node>,
    mut best_clique: HashSet<Node>,
) -> HashSet<Node> {
    if candidates.is_empty() && excluded.is_empty() {
        if current_clique.len() > best_clique.len() {
            return current_clique;
        }
        return best_clique;
    }
    let pivot = if candidates.is_empty() {
        excluded.iter().next().unwrap()
    } else {
        candidates.iter().next().unwrap()
    };

    let pivot_neighbours = graph.neighbours.get(pivot).unwrap();
    for vertex in candidates.iter() {
        if pivot_neighbours.contains(vertex) {
            continue;
        }
        let mut new_current_clique = current_clique.clone();
        new_current_clique.insert(vertex.to_string());

        let vertex_neighbours = graph.neighbours.get(vertex).unwrap();
        let new_candidates = adjacency_list_intersection(&candidates, vertex_neighbours);

        let new_excluded = adjacency_list_intersection(&excluded, vertex_neighbours);

        best_clique = recurse_bron_kerbosch(
            graph,
            new_current_clique,
            new_candidates,
            new_excluded,
            best_clique,
        );
    }

    best_clique
}

pub fn run() -> Result<(), Error> {
    let _ = task1();
    let _ = task2();

    println!("Completed solutions for Day 23!");

    Ok(())
}

fn task1() -> Result<(), Error> {
    println!("Computing solution for task 1 of Day 23...");

    let input_data = fs::read_to_string("input_data/day23_input.txt")?;

    let graph = Graph::from_input(input_data);

    let triangles = get_all_triangles(graph);

    let mut num_triangles_with_t = 0;
    for triangle in triangles.iter() {
        for node in triangle {
            if node.starts_with('t') {
                num_triangles_with_t += 1;
                break;
            }
        }
    }

    let mut solution_file = fs::File::create("solutions/day23_solution.txt")?;
    writeln!(solution_file, "Solution for Task 1 of Day 23:")?;
    writeln!(
        solution_file,
        "There are {} triangles in the network that contain a node starting with 't'.",
        num_triangles_with_t
    )?;

    Ok(())
}

fn task2() -> Result<(), Error> {
    println!("Computing solution for task 2 of Day 23...");

    let input_data = fs::read_to_string("input_data/day23_input.txt")?;

    let graph = Graph::from_input(input_data);

    let max_clique = get_max_clique(&graph);
    let mut max_clique_nodes: Vec<Node> = max_clique.into_iter().collect();
    max_clique_nodes.sort();

    let password = max_clique_nodes.join(",");

    let mut solution_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("solutions/day23_solution.txt")?;
    writeln!(solution_file)?;
    writeln!(solution_file, "Solution for Task 2 of Day 23:")?;
    writeln!(
        solution_file,
        "The elements of the max clique in the network (and hence the password) are {}.",
        password
    )?;

    Ok(())
}
