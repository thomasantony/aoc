use ::aoc2019::intcode::IntComputer;
use ::aoc2019::parse_numbers_with_delimiter;
use std::collections::HashMap;
use std::iter::FromIterator;

type Coord = (i32, i32);
type Grid = HashMap<Coord, String>;

#[derive(Debug, PartialEq)]
pub enum RobotDirection {
    Up,
    Down,
    Left,
    Right,
}
impl RobotDirection {
    pub fn turn_right(&self) -> Self {
        use RobotDirection::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
    pub fn turn_left(&self) -> Self {
        use RobotDirection::*;
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }
}

fn neighbor_indices(pos: Coord, nrows: i32, ncols: i32) -> Vec<(Coord, RobotDirection)> {
    let x0 = pos.0 - 1;
    let x1 = pos.0 + 1;

    let y0 = pos.1 - 1;
    let y1 = pos.1 + 1;

    let mut output = Vec::new();
    if x0 > -1 {
        output.push(((x0, pos.1), RobotDirection::Up));
    }
    if x1 < nrows {
        output.push(((x1, pos.1), RobotDirection::Down));
    }
    if y0 > -1 {
        output.push(((pos.0, y0), RobotDirection::Left));
    }
    if y1 < ncols {
        output.push(((pos.0, y1), RobotDirection::Right));
    }
    output
}
fn find_intersections(map: &Grid, nrows: i32, ncols: i32) -> Vec<Coord> {
    let mut output = Vec::new();
    for row in 0..nrows {
        for col in 0..ncols {
            let pos = (row, col);
            if map.get(&(row, col)) != Some(&"#".to_string()) {
                continue;
            }
            let is_intersection = neighbor_indices((row, col), nrows, ncols)
                .iter()
                .all(|(node, _)| map.get(node) == Some(&"#".to_string()));
            if is_intersection {
                output.push(pos);
            }
        }
    }
    output
}
fn load_map(input: &Vec<i64>) -> (Grid, i32, i32) {
    let mut row = 0;
    let mut col = 0;
    let mut map = Grid::new();

    let mut ncols = 0;
    for element in input {
        if *element == 10 {
            row += 1;
            col = 0;
            continue;
        } else {
            let c = std::char::from_u32(*element as u32).expect("invalid ascii");
            map.insert((row, col), format!("{}", c));
        };
        col += 1;
        if col > ncols {
            ncols = col;
        }
    }
    (map, row - 1, ncols)
}
fn show_map(map: &Grid, nrows: i32, ncols: i32) {
    for i in 0..nrows {
        for j in 0..ncols {
            print!("{}", map.get(&(i, j)).unwrap_or(&" ".to_string()));
        }
        println!();
    }
}
fn solve_part_a(map: &Grid, nrows: i32, ncols: i32) -> i32 {
    let intersections = find_intersections(&map, nrows, ncols);
    intersections.iter().map(|(i, j)| i * j).sum()
}
fn main() {
    let input = include_str!("../../inputs/day17.txt").to_string();
    let mut program: Vec<i64> = parse_numbers_with_delimiter(&input, ',').collect();

    let mut vm = IntComputer::new();
    vm.set_ram_size(10485760);
    vm.load_program(&program);

    let vm_output = vm.execute();
    let (map, nrows, ncols) = load_map(&vm_output);
    // show_map(&map, nrows, ncols);

    let part_a = solve_part_a(&map, nrows, ncols);
    println!("Part A: {}", part_a);

    let commands = vec![
        ("L", 6),
        ("L", 4),
        ("R", 12),
        ("L", 6),
        ("R", 12),
        ("R", 12),
        ("L", 8),
        ("L", 6),
        ("L", 4),
        ("R", 12),
        ("L", 6),
        ("L", 10),
        ("L", 10),
        ("L", 6),
        ("L", 6),
        ("R", 12),
        ("R", 12),
        ("L", 8),
        ("L", 6),
        ("L", 4),
        ("R", 12),
        ("L", 6),
        ("L", 10),
        ("L", 10),
        ("L", 6),
        ("L", 6),
        ("R", 12),
        ("R", 12),
        ("L", 8),
        ("L", 6),
        ("L", 4),
        ("R", 12),
        ("L", 6),
        ("L", 10),
        ("L", 10),
        ("L", 6),
    ];

    use itertools::Itertools;
    use std::collections::HashSet;
    let uniq: HashSet<(&'static str, i32)> = HashSet::from_iter(commands.iter().cloned());
    let uniq: Vec<_> = uniq.iter().cloned().sorted().collect();
    // println!("uniq: {:?}", uniq);

    // let cmd = commands.iter().map(|c| format!("{}", uniq.iter().position(|p| p==c).unwrap()))
    //                .join("");
    // 104 1442 104 1331 1442 104 1331 1442 104 1331
    // A   B    A   C    B    A   C    B    A   C
    let cmd_a = vec![uniq[1], uniq[0], uniq[4]];
    let cmd_b = vec![uniq[1], uniq[4], uniq[4], uniq[2]];
    let cmd_c = vec![uniq[1], uniq[3], uniq[3], uniq[1]];

    let cmd0 = "A,B,A,C,B,A,C,B,A,C\n".to_string();
    let cmd_a = cmd_a.iter().map(|c| format!("{},{}", c.0, c.1)).join(",") + "\n";
    let cmd_b = cmd_b.iter().map(|c| format!("{},{}", c.0, c.1)).join(",") + "\n";
    let cmd_c = cmd_c.iter().map(|c| format!("{},{}", c.0, c.1)).join(",") + "\n";

    // Activate robot
    program[0] = 2;
    vm.reset();
    vm.load_program(&program);

    let output = vm.execute();
    let (map, nrows, ncols) = load_map(&output);
    show_map(&map, nrows, ncols);

    // Command
    vm.set_input(&cmd0.chars().map(|c| c as i64).collect());
    let output = vm.execute();
    let output_u8: Vec<_> = output.iter().map(|&c| c as u8).collect();
    println!("{}{}", std::str::from_utf8(&output_u8).unwrap(), &cmd_a);

    // Function A
    vm.set_input(&cmd_a.chars().map(|c| c as i64).collect());
    let output = vm.execute();
    let output_u8: Vec<_> = output.iter().map(|&c| c as u8).collect();
    println!("{}{}", std::str::from_utf8(&output_u8).unwrap(), &cmd_b);

    // Function B
    vm.set_input(&cmd_b.chars().map(|c| c as i64).collect());
    let output = vm.execute();
    let output_u8: Vec<_> = output.iter().map(|&c| c as u8).collect();
    println!("{}{}", std::str::from_utf8(&output_u8).unwrap(), &cmd_c);

    // Function C
    vm.set_input(&cmd_c.chars().map(|c| c as i64).collect());
    let output = vm.execute();
    let output_u8: Vec<_> = output.iter().map(|&c| c as u8).collect();
    println!("{}{}", std::str::from_utf8(&output_u8).unwrap(), "y");

    // vm.push_input(121); // 'y' for video feed
    vm.push_input(110); // 'n' for video feed
    vm.push_input(10);

    let output = vm.execute();
    let part_b = output[output.len() - 1];
    let (map, nrows, ncols) = load_map(&output);
    show_map(&map, nrows, ncols);
    println!("Part B: {}", part_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day17_load_map() {
        let input_str = "..#..........\n\
                               ..#..........\n\
                               #######...###\n\
                               #.#...#...#.#\n\
                               #############\n\
                               ..#...#...#..\n\
                               ..#####...^..\n\n";
        let input_vec: Vec<_> = input_str.chars().map(|c| c as i64).collect();
        let (map, nrows, ncols) = load_map(&input_vec);

        assert_eq!(map[&(0, 0)], ".");
        assert_eq!(map[&(0, 2)], "#");
        assert_eq!(map[&(1, 2)], "#");
        assert_eq!(map[&(6, 10)], "^");
        assert_eq!(nrows, 7);
        assert_eq!(ncols, 13);

        let intersections = find_intersections(&map, nrows, ncols);
        assert_eq!(intersections.len(), 4);
        assert_eq!(intersections, vec![(2, 2), (4, 2), (4, 6), (4, 10)]);

        // let graph = make_graph(&map, nrows, ncols);
        // println!("{:?}", graph);
    }
}
