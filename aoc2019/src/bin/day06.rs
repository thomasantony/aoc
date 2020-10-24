use ::aoc2019::*;
use std::collections::HashMap;

pub struct Orbit {
    center: String,
    orbiter: String
}

#[derive(Debug)]
pub struct Body {
    parent_name: Option<String>,
    count: Option<i32>
}

fn parse_data(line: &str) -> Orbit
{
    let mut parts = line.split(')');
    Orbit{center: parts.next().expect("Failed to parse center body").to_string(), 
                  orbiter: parts.next().expect("Failed to parse orbiting body").to_string()}
}

fn main() {
    let input = read_stdin();

    let mut graph = HashMap::new();
    graph.insert("COM".to_string(), Body{parent_name: None, count: Some(0)});
    let data: Vec<Orbit> = get_lines(&input)
                            .map(|line| parse_data(line))
                            .collect();
    for orbit in data
    {
        if graph.contains_key(&orbit.orbiter)
        {
            panic!("{} already exists in graph!", orbit.orbiter);
        }
        let new_body = Body {
            count: None,
            parent_name: Some(orbit.center)
        };
        graph.insert(orbit.orbiter, new_body);
    }
    // Calculate counts
    let mut total = 0;
    for (k, b) in graph.iter() {
        let mut count = 0;
        let mut body = b;
        loop{
            if let Some(body_name) = &body.parent_name
            {
                body = graph.get(body_name).unwrap();
                count +=1;
            }else{
                break;
            }
        }
        total += count;
    }

    println!("Part A: {}", total);
}

