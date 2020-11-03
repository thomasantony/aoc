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
use std::cmp::Ordering;
impl Ord for Key {
    fn cmp(&self, other: &Key) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.0.cmp(&self.0)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Key) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
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
                    new_keys.insert(key.clone());
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
use std::iter::FromIterator;
use std::collections::BTreeSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Node
{
    pos: Coord,
    keys: BTreeSet<Key>
}

fn solve_part_a(map: &Map) -> usize
{
    let start_node = Node {
        pos: map.start_pos.clone(),
        keys: BTreeSet::new()
    };
    let path = map.search_with(&start_node, |n| {
        n.keys.len() == map.num_keys
    });
    path.len() - 1
}
fn main()
{
    let input = include_str!("../../inputs/day18.txt").to_string();
    
    let map = Map::from(&input);
    let part_a = solve_part_a(&map);
    println!("{:?}", part_a);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day18_part_a()
    {
        let input = "#########\n\
                    #b.A.@.a#\n\
                    #########".to_string();
        let map = Map::from(&input);
        assert_eq!(solve_part_a(&map), 8);
        
        let input = "########################\n\
                    #f.D.E.e.C.b.A.@.a.B.c.#\n\
                    ######################.#\n\
                    #d.....................#\n\
                    ########################".to_string();
        let map = Map::from(&input);
        assert_eq!(solve_part_a(&map), 86);

        let input = "########################\n\
                     #...............b.C.D.f#\n\
                     #.######################\n\
                     #.....@.a.B.c.d.A.e.F.g#\n\
                     ########################".to_string();
        let map = Map::from(&input);
        assert_eq!(solve_part_a(&map), 132);

        let input = "#################\n\
                    #i.G..c...e..H.p#\n\
                    ########.########\n\
                    #j.A..b...f..D.o#\n\
                    ########@########\n\
                    #k.E..a...g..B.n#\n\
                    ########.########\n\
                    #l.F..d...h..C.m#\n\
                    #################".to_string();
        let map = Map::from(&input);
        assert_eq!(solve_part_a(&map), 136);

        let input = "########################\n\
                    #@..............ac.GI.b#\n\
                    ###d#e#f################\n\
                    ###A#B#C################\n\
                    ###g#h#i################\n\
                    ########################".to_string();
        let map = Map::from(&input);
        assert_eq!(solve_part_a(&map), 81);
    }
}