extern crate regex;
#[macro_use] extern crate lazy_static;

use regex::Regex;
use std::io::{self, Read};
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashSet;

struct Node {
    value_idx: i32,
    children: HashSet<i32>
}

impl Node {
    fn new(value_idx: i32) -> Node {
        Node {
            value_idx,
            children: HashSet::new()
        }
    }
}

fn parse_line(line: &str) -> (String, String)
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Step (\w) must be finished before step (\w) can begin")
                                          .expect("Error initializing regex!");
    }
    let cap = RE.captures(line).expect("Error parsing line");
    return (cap[1].to_string(), cap[2].to_string());
}

fn main() -> io::Result<()> {
    let mut all_nodes: HashMap<String, Node> = HashMap::new();
    let mut all_letters: Vec<String> = Vec::new();
    
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    // let root = Node::from(buffer.lines().next().unwrap().to_string());
    for (lineno, line) in buffer.lines().enumerate()
    {
        let (parent, child) = parse_line(line);
        let node = match all_nodes.entry(parent.clone()) {
            Vacant(entry) => entry.insert(Node::new(parent)),
            Occupied(entry) => entry.into_mut()
        };

        let child_node = match all_nodes.entry(child.clone()) {
            Vacant(entry) => entry.insert(Node::new(child)),
            Occupied(entry) => entry.into_mut(),
        };
        node.children.insert(child_node)
    }
    Ok(())
}
