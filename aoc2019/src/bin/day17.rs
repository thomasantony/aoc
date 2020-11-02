use ::aoc2019::intcode::IntComputer;
use ::aoc2019::parse_numbers_with_delimiter;
use std::collections::HashMap;

type Coord = (i32, i32);
type Grid = HashMap<Coord, String>;

fn neighbor_indices(pos: Coord, nrows: i32, ncols: i32) -> Vec<Coord>
{
    let x0 = pos.0 - 1;
    let x1 = pos.0 + 1;

    let y0 = pos.1 - 1;
    let y1 = pos.1 + 1;

    let mut output = Vec::new();
    if x0 > -1
    {
        output.push((x0, pos.1));
    }
    if x1 < nrows
    {
        output.push((x1, pos.1));
    }
    if y0 > -1
    {
        output.push((pos.0, y0));
    }
    if y1 < ncols
    {
        output.push((pos.0, y1));
    }
    output
}
fn find_intersections(map: &Grid, nrows: i32, ncols: i32) -> Vec<Coord>
{
    let mut output = Vec::new();
    for row in 0..nrows
    {
        for col in 0..ncols
        {
            let pos = (row, col);
            if map.get(&(row, col)) != Some(& "#".to_string())
            {
                continue;
            }
            let is_intersection = neighbor_indices((row, col), nrows, ncols)
            .iter().all(|node|
            {
                map.get(node) == Some(& "#".to_string())
            });
            if is_intersection{
                output.push(pos);
            }
        }
    }
    output
}
fn load_map(input: &Vec<i64>) -> (Grid, i32, i32)
{
    let mut row = 0;
    let mut col = 0;
    let mut map = Grid::new();

    let mut ncols = 0;
    for element in input
    {
        if *element == 10
        {
            row += 1;
            col = 0;
            continue;
        }else{
            let c = std::char::from_u32(*element as u32).expect("invalid ascii");
            map.insert((row, col), format!("{}", c));
        };
        col += 1; 
        if col > ncols {
            ncols = col;
        }
    }
    (map, row-1, ncols)
}
fn show_map(map: &Grid, nrows: i32, ncols: i32)
{
    for i in 0..nrows {
        for j in 0..ncols {
            print!("{:?}", map.get(&(i, j)).unwrap());
        }
        println!();
    }
}
fn solve_part_a(map: &Grid, nrows: i32, ncols: i32) -> i32
{
    let intersections = find_intersections(&map, nrows, ncols);
    intersections.iter().map(|(i,j)| i * j).sum()
}
fn main()
{
    let input = include_str!("../../inputs/day17.txt").to_string();
    let program: Vec<i64> = parse_numbers_with_delimiter(&input, ',').collect();

    let mut vm = IntComputer::new();
    vm.set_ram_size(104800);
    vm.load_program(&program);
    
    let vm_output = vm.execute();
    let (map, nrows, ncols)= load_map(&vm_output);
    show_map(&map, nrows, ncols);

    let part_a = solve_part_a(&map, nrows, ncols);
    println!("Part A: {}", part_a);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day17_load_map()
    {
        let input_str = "..#..........\n\
                               ..#..........\n\
                               #######...###\n\
                               #.#...#...#.#\n\
                               #############\n\
                               ..#...#...#..\n\
                               ..#####...^..\n";
        let input_vec: Vec<_> = input_str.chars().map(|c| c as i64).collect();
        let (map, nrows, ncols) = load_map(&input_vec);

        assert_eq!(map[&(0,0)], ".");
        assert_eq!(map[&(0,2)], "#");
        assert_eq!(map[&(1,2)], "#");
        assert_eq!(map[&(6,10)], "^");
        assert_eq!(nrows, 7);
        assert_eq!(ncols, 13);

        let intersections = find_intersections(&map, nrows, ncols);
        assert_eq!(intersections.len(), 4);
        assert_eq!(intersections, vec![(2, 2), (4, 2), (4, 6), (4, 10)]);
    }
}