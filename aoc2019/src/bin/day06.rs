use ::aoc2019::*;
use std::collections::{HashMap, HashSet};

pub struct Orbit {
    center: String,
    orbiter: String
}

#[derive(Debug)]
pub struct Body {
    parent_name: Option<String>,
    count: Option<i32>
}

use crate::graph::{Graph, NodeData, NodeIndex, EdgeData};

type OrbitGraph = (HashMap<String, NodeIndex>, Graph);
fn create_orbit_graph<'a>(orbits: Vec<Orbit>) -> OrbitGraph {
    let mut graph = Graph::new();
    let mut bodies = HashMap::new();
    for orbit in orbits.into_iter() 
    {
        if !bodies.contains_key(&orbit.center)
        {
            bodies.insert(orbit.center.clone(), graph.add_node());
        }
        if !bodies.contains_key(&orbit.orbiter)
        {
            bodies.insert(orbit.orbiter.clone(), graph.add_node());
        }
        let center_node = * bodies.get(&orbit.center).unwrap();
        let orbit_node = * bodies.get(&orbit.orbiter).unwrap();
        graph.add_edge(center_node, orbit_node);
    }
    (bodies, graph)
}
fn create_orbit_hashmap(orbits: &Vec<Orbit>) -> HashMap<String, Body> {
    let mut graph = HashMap::new();
    graph.insert("COM".to_string(), Body{parent_name: None, count: Some(0)});
    for orbit in orbits.iter()
    {
        if graph.contains_key(&orbit.orbiter)
        {
            panic!("{} already exists in graph!", orbit.orbiter);
        }
        let new_body = Body {
            count: None,
            parent_name: Some(orbit.center.clone())
        };
        graph.insert(orbit.orbiter.clone(), new_body);
    }
    graph
}
fn parse_data(line: &str) -> Option<Orbit>
{
    if line.len() == 0 { 
        None
    }else{
        let mut parts = line.split(')');
    
        let orbit = Orbit{
            center: parts.next().expect("Failed to parse center body").to_string(), 
            orbiter: parts.next().expect("Failed to parse orbiting body").to_string()
        };
        Some(orbit)
    }
}

fn find_transfer_length(graph: &OrbitGraph, start: &str, dest: &str) -> usize
{
    let bodies = &graph.0;
    let graph = &graph.1;

    println!("Bodies: {:?}", bodies);
    let start_node = *bodies.get(start).expect("Start node not found in graph");
    println!("start node is {}", start_node);
    
    let goal_node = *bodies.get(dest).expect("Goal node not found in graph");
    println!("goal node is {}", goal_node);

    let path = graph.djikstra(start_node, goal_node);
    println!("PATH is {:?}", path);
    path.len()
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
        loop{
            if let Some(body_name) = &body.parent_name
            {
                body = count_map.get(body_name).unwrap();
                count +=1;
            }else{
                break;
            }
        }
        total += count;
    }

    println!("Part A: {}", total);

    let graph = create_orbit_graph(orbits);
    let path_len = find_transfer_length(&graph, "YOU", "SAN");
    println!("Part B: {}", path_len - 2);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn demo_orbits() -> Vec<Orbit>
    {
        let data_str = vec!["COM)B",
            "B)C",
            "C)D",
            "D)E",
            "E)F",
            "B)G",
            "G)H",
            "D)I",
            "E)J",
            "J)K",
            "K)L"];
        data_str.into_iter().map(parse_data).flatten().collect()
    }
    fn demo_graph() -> OrbitGraph
    {
        let orbits = demo_orbits();
        create_orbit_graph(orbits)
    }
    #[test]
    fn test_day06_find_path_length()
    {
        let graph = demo_graph();
        assert_eq!(find_transfer_length(&graph, "COM", "COM"), 1);
        assert_eq!(find_transfer_length(&graph, "COM", "B"), 2);
    }
}