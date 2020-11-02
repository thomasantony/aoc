use std::collections::HashMap;
use std::collections::HashSet;

type Coord = (i32, i32);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Door(String);
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Key(String);

impl Door {
    pub fn matches(&self, key: &Key) -> bool 
    {
        self.0.to_ascii_uppercase() == key.0.to_ascii_uppercase()
    }
}

impl Key {
    pub fn matches(&self, door: &Door) -> bool 
    {
        self.0.to_ascii_uppercase() == door.0.to_ascii_uppercase()
    }
}

#[derive(Debug, PartialEq)]
pub enum CellType {
    Wall,
    Empty,
    Door(Door),
    Key(Key),
}
impl From<char> for CellType {
    fn from(c: char) -> Self {
        if c == '#' 
        {
            CellType::Wall
        } else if c == '.' || c == '@'
        {
            CellType::Empty
        } else if c.is_ascii_uppercase()
        {
            CellType::Door(Door(c.to_string()))
        } else if c.is_ascii_lowercase()
        {
            CellType::Key(Key(c.to_string()))
        } else {
            panic!("Invalid character in map {}", c);
        }
    }
}
pub struct Map {
    grid: HashMap<Coord, CellType>,
    passable_cells: HashSet<Coord>,
    start_pos: Coord,
    num_keys: usize,
}
impl Map {
    pub fn from(input: &str) -> Self {
        let mut grid = HashMap::new();
        let mut passable_cells = HashSet::new();
        let mut start_pos = (-1, -1);
        let mut num_keys = 0;
        for (row, line) in input.lines().enumerate()
        {
            for (col, c) in line.chars().enumerate()
            {
                if c == '@'
                {
                    start_pos = (row as i32, col as i32);
                }
                let cell = CellType::from(c);
                if cell != CellType::Wall
                {
                    passable_cells.insert((row as i32, col as i32));
                }

                match cell {
                    CellType::Key(_) => {num_keys += 1},
                    _ => {}
                }
                grid.insert((row as i32, col as i32), cell);
            }
        }
        Self
        {
            grid,
            passable_cells,
            start_pos,
            num_keys
        }
    }
    fn get_new_node_if_passable(&self, node_from: &Node, new_cell_pos: Coord) -> Option<Node>
    {
        if !self.passable_cells.contains(&new_cell_pos)
        {
            None
        }else{
            let available_keys = &node_from.keys;
            let cell_type = self.grid.get(&new_cell_pos).unwrap();
            
            match cell_type {
                CellType::Door(door) => {
                    let key_available = available_keys.iter()
                                                        .any(|key| door.matches(key));
                    if key_available {
                        Some(Node { 
                            pos: new_cell_pos,
                            keys: available_keys.clone()
                        })
                    }else {
                        None
                    }
                },
                CellType::Key(key) => {
                    let mut new_keys = available_keys.clone();
                    new_keys.push(key.clone());
                    new_keys.dedup();
                    Some(Node { 
                        pos: new_cell_pos,
                        keys: new_keys
                    })
                },
                CellType::Empty => {
                    Some(Node { 
                        pos: new_cell_pos,
                        keys: node_from.keys.clone()
                    })
                },
                _ => None
            }
        }
    }
}

use ::aoc2019::graph::BFSGraph;
impl BFSGraph<Node> for Map {
    fn successors(&self, node: &Node) -> Vec<Node>
    {
        let above_pos = (node.pos.0 - 1, node.pos.1);
        let below_pos = (node.pos.0 + 1, node.pos.1);
        let left_pos = (node.pos.0, node.pos.1 - 1);
        let right_pos = (node.pos.0, node.pos.1 + 1);

        let new_nodes = vec![above_pos, below_pos, left_pos, right_pos];
        new_nodes.into_iter()
                .map(|pos| self.get_new_node_if_passable(node, pos))
                .flatten()
                .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Node
{
    pos: Coord,
    keys: Vec<Key>
}

fn main()
{
    let input = include_str!("../../inputs/day18.txt").to_string();
    let map = Map::from(&input);

    let start_node = Node {
        pos: map.start_pos.clone(),
        keys: Vec::new()
    };
    let path = map.search_with(&start_node, |n| {
        n.keys.len() == map.num_keys
    });
    println!("{:?}", path);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_day18_search()
    {

    }
}