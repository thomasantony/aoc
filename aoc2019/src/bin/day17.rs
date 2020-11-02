use ::aoc2019::intcode::IntComputer;
use ::aoc2019::parse_numbers_with_delimiter;
use std::collections::HashMap;

type Coord = (i32, i32);
type Grid = HashMap<Coord, String>;

fn find_intersections(map: &Grid, n_rows: usize, n_cols: usize) -> Vec<Coord>
{
    
    Vec::new()
}
fn load_map(input: &Vec<i64>) -> (Grid, i32, i32)
{
    let mut row = 0;
    let mut col = 0;
    let mut map = Grid::new();
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
    }
    (map, row+1, col)
}
fn main()
{
    let input = include_str!("../../inputs/day17.txt").to_string();
    let program: Vec<i64> = parse_numbers_with_delimiter(&input, ',').collect();

    let mut vm = IntComputer::new();
    vm.load_program(&program);
    let vm_output = vm.execute();
    
    let map= load_map(&vm_output);
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
                               ..#####...^..";
        let input_vec: Vec<_> = input_str.chars().map(|c| c as i64).collect();
        let (map, nrows, ncols) = load_map(&input_vec);

        assert_eq!(map[&(0,0)], ".");
        assert_eq!(map[&(0,2)], "#");
        assert_eq!(map[&(1,2)], "#");
        assert_eq!(map[&(6,10)], "^");
        assert_eq!(nrows, 7);
        assert_eq!(ncols, 13);
    }
}