/// Reference: http://www-cs-students.stanford.edu/~amitp/Articles/LineOfSight.html
use std::f32::consts::PI;
use std::collections::{HashMap, HashSet};
use ::aoc2019::*;
use itertools::Itertools;


type Point = (usize, usize);
type Gradient = (i32, i32);
fn approx_equal(a: f32, b: f32, decimal_places: u8) -> bool {
    let factor = 10.0f32.powi(decimal_places as i32);
    let a = (a * factor).trunc();
    let b = (b * factor).trunc();
    a == b
}
fn read_asteroid_map(input: &String) -> Vec<Point>
{
    let lines = get_lines(input);
    let mut points = Vec::new();
    for (j, line) in lines.enumerate() 
    {
        line.chars()
            .enumerate()
            .filter(|(_, p)| *p == '#')
            .for_each(|(i, _)| {
                points.push((i, j))
            });
    }
    points
}
fn find_visible_points(point: &(i32, i32), asteroid_map: &Vec<(i32, i32)>) -> Vec<(i32, i32)>
{
    for b in asteroid_map
    {
        let grad = (b.1-point.1, b.0-point.0);
    }

    Vec::new()
}
fn compute_gradient_and_distance(a: &Point, b: &Point) -> (Gradient, i32)
{
    let grad = (a.1 as i32 - b.1 as i32 , a.0 as i32 - b.0 as i32);
    // GCD of gradient is proxy for manhattan distance 
    // (except when it is zero, in which case it can be considered unity)
    let gcd = num::integer::gcd(grad.0, grad.1);
    if gcd != 0
    {
        ((grad.0/gcd, grad.1/gcd), gcd)
    }else{
        (grad, 1)
    }
    
}
fn solve_part_a(asteroid_map: &Vec<Point>) -> (Point, usize)
{
    let mut visible_count_map: HashMap<Point, usize> = HashMap::new();
    for i in 0..asteroid_map.len()
    {
        let p1 = asteroid_map[i];
        let mut grad_map = HashMap::new();
        for j in 0..asteroid_map.len()
        {
            if i == j{
                continue;
            }
            let p2 = asteroid_map[j];
            
            let (gradient, dist) = compute_gradient_and_distance(&p1, &p2);  
            
            let entry = grad_map.entry(gradient)
                                                                 .or_insert(Vec::new());
            entry.push((dist, p2));
        }
        let num_visible_points = grad_map.values().count();
        visible_count_map.insert(p1, num_visible_points);
    }
    let best_asteroid = visible_count_map
                                                        .iter()
                                                        .max_by_key(|(&k, &v)| v)
                                                        .unwrap();
    (*best_asteroid.0, *best_asteroid.1)
}
fn main()
{
    let input = read_stdin();
    let asteroid_map = read_asteroid_map(&input);

    let part_a = solve_part_a(&asteroid_map);
    println!("Part A: {:?} with {} visible", part_a.0, part_a.1);    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day10_part_a()
    {
        let test_cases = vec![
                         (".#..# \
                           ..... \
                           ##### \
                           ....# \
                           ...##", ((3, 4), 8)),
                         ("......#.#. \
                           #..#.#.... \
                           ..#######. \
                           .#.#.###.. \
                           .#..#..... \
                           ..#....#.# \
                           #..#....#. \
                           .##.#..### \
                           ##...#..#. \
                           .#....####", ((5, 8), 33)),
                         ("#.#...#.#. \
                           .###....#. \
                           .#....#... \
                           ##.#.#.#.# \
                           ....#.#.#. \
                           .##..###.# \
                           ..#...##.. \
                           ..##....## \
                           ......#... \
                           .####.###.", ((1, 2), 35)),
                         (".#..#..### \
                           ####.###.# \
                           ....###.#. \
                           ..###.##.# \
                           ##.##.#.#. \
                           ....###..# \
                           ..#.#..#.# \
                           #..#.#.### \
                           .##...##.# \
                           .....#.#..", ((6, 3), 41)),
                         (".#..##.###...####### \
                           ##.############..##. \
                           .#.######.########.# \
                           .###.#######.####.#. \
                           #####.##.#.##.###.## \
                           ..#####..#.######### \
                           #################### \
                           #.####....###.#.#.## \
                           ##.################# \
                           #####.##.###..####.. \
                           ..######..##.####### \
                           ####.##.####...##..# \
                           .#####..#.######.### \
                           ##...#.##########... \
                           #.##########.####### \
                           .####.#.###.###.#.## \
                           ....##.##.###..##### \
                           .#.#.###########.### \
                           #.#.#.#####.####.### \
                           ###.##.####.##.#..##", ((11, 13), 210))

        ];

        for (input, expected_output) in test_cases.into_iter()
        {
            let map = read_asteroid_map(&String::from(input));
            let output = solve_part_a(&map);
            assert_eq!(output, expected_output);
        }
    }
}