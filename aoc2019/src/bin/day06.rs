use crate::tree::Tree;
use ::aoc2019::*;
use std::collections::HashMap;

pub struct Orbit {
    center: String,
    orbiter: String,
}

#[derive(Debug)]
pub struct Body {
    parent_name: Option<String>,
    count: Option<i32>,
}

type OrbitGraph = (Vec<String>, Tree);
fn create_orbit_tree<'a>(orbits: Vec<Orbit>) -> OrbitGraph {
    let mut tree = Tree::new();
    let mut bodies = HashMap::new();
    let mut bodies_vec = Vec::new();

    for orbit in orbits.into_iter() {
        if !bodies.contains_key(&orbit.center) {
            bodies.insert(orbit.center.clone(), tree.add_node());
            bodies_vec.push(orbit.center.clone());
        }
        if !bodies.contains_key(&orbit.orbiter) {
            bodies.insert(orbit.orbiter.clone(), tree.add_node());
            bodies_vec.push(orbit.orbiter.clone());
        }
        let center_node = *bodies.get(&orbit.center).unwrap();
        let orbit_node = *bodies.get(&orbit.orbiter).unwrap();
        tree.set_parent(orbit_node, center_node);
    }
    (bodies_vec, tree)
}
fn create_orbit_hashmap(orbits: &Vec<Orbit>) -> HashMap<String, Body> {
    let mut graph = HashMap::new();
    graph.insert(
        "COM".to_string(),
        Body {
            parent_name: None,
            count: Some(0),
        },
    );
    for orbit in orbits.iter() {
        if graph.contains_key(&orbit.orbiter) {
            panic!("{} already exists in graph!", orbit.orbiter);
        }
        let new_body = Body {
            count: None,
            parent_name: Some(orbit.center.clone()),
        };
        graph.insert(orbit.orbiter.clone(), new_body);
    }
    graph
}
fn parse_data(line: &str) -> Option<Orbit> {
    if line.len() == 0 {
        None
    } else {
        let mut parts = line.split(')');

        let orbit = Orbit {
            center: parts
                .next()
                .expect("Failed to parse center body")
                .to_string(),
            orbiter: parts
                .next()
                .expect("Failed to parse orbiting body")
                .to_string(),
        };
        Some(orbit)
    }
}

fn find_transfer_length(graph: &OrbitGraph, start: &str, dest: &str) -> usize {
    let bodies = &graph.0;
    let tree = &graph.1;

    let start_node = bodies
        .iter()
        .position(|x| x == start)
        .expect("Start node not found in graph");
    let goal_node = bodies
        .iter()
        .position(|x| x == dest)
        .expect("Goal node not found in graph");

    let mut path_1: Vec<&String> = tree.ancestors(start_node).map(|i| &bodies[i]).collect();
    let mut path_2: Vec<&String> = tree.ancestors(goal_node).map(|i| &bodies[i]).collect();

    path_1.reverse();
    path_2.reverse();

    let mut last_common_node = (0, &path_1[0]);
    for ((i1, node1), node2) in path_1.iter().enumerate().zip(path_2.iter()) {
        if node1 != node2 {
            break;
        } else {
            last_common_node = (i1, &node1);
        }
    }

    let common_node_index = last_common_node.0;
    let path_len_1 = path_1.len() - common_node_index;
    let path_len_2 = path_2.len() - common_node_index;
    let path_len = path_len_1 + path_len_2;
    path_len - std::cmp::min(path_len, 4)
}

fn main() {
    let input = read_stdin();
    let orbits: Vec<Orbit> = input.split("\n").map(parse_data).flatten().collect();
    let count_map = create_orbit_hashmap(&orbits);

    // Calculate counts
    let mut total = 0;
    for (_, b) in count_map.iter() {
        let mut count = 0;
        let mut body = b;
        loop {
            if let Some(body_name) = &body.parent_name {
                body = count_map.get(body_name).unwrap();
                count += 1;
            } else {
                break;
            }
        }
        total += count;
    }

    println!("Part A: {}", total);

    let graph = create_orbit_tree(orbits);
    let path_len = find_transfer_length(&graph, "YOU", "SAN");
    println!("Part B: {}", path_len);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn demo_orbits() -> Vec<Orbit> {
        let data_str = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ];
        data_str.into_iter().map(parse_data).flatten().collect()
    }
    fn demo_tree() -> OrbitGraph {
        let orbits = demo_orbits();
        create_orbit_tree(orbits)
    }
    #[test]
    fn test_day06_find_path_length() {
        let graph = demo_tree();
        assert_eq!(find_transfer_length(&graph, "COM", "COM"), 0);
        assert_eq!(find_transfer_length(&graph, "YOU", "SAN"), 4);
    }
}
