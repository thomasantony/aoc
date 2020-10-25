/// Reference: http://www-cs-students.stanford.edu/~amitp/Articles/LineOfSight.html
use std::f32::consts::PI;
use std::collections::{HashMap, HashSet};
use ::aoc2019::*;
use itertools::Itertools;


type Point = (usize, usize);
type Gradient = (i32, i32);
type GradMap = HashMap<Gradient, Vec<Point>>;

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
/// Computes a HashMap with a gradient as key and vector of points sorted by distance as the value
fn compute_gradient_map(point: &Point, asteroid_map: &Vec<Point>) -> GradMap
{
    let mut grad_map = HashMap::new();
    for j in 0..asteroid_map.len()
    {
        let p2 = asteroid_map[j];
        if point == &p2
        {
            continue;
        }
        let (gradient, dist) = compute_gradient_and_distance(&point, &p2);  
        
        let entry = grad_map.entry(gradient)
                  .or_insert(Vec::new());
        entry.push((dist, p2));
    }
    // Sort each vector by distance and remove the extra "distance" value in each item
    let output = grad_map.into_iter().map(|(k, mut v)|{
        v.sort();
        let v = v.iter().map(|item| item.1).collect::<Vec<Point>>();
        (k, v)
    }).collect();
    output
}
fn compute_visibility_map(asteroid_map: &Vec<Point>) -> HashMap<Point, GradMap>
{
    let visibility_map = HashMap::new();
    for i in 0..asteroid_map.len()
    {
        let p1 = asteroid_map[i];
        let grad_map = compute_gradient_map(&p1, asteroid_map);
        visibility_map.insert(p1, grad_map);
    }
    visibility_map
}
fn solve_part_a(visibility_map: &HashMap<Point, GradMap>) -> (Point, usize)
{
    let best_asteroid = visibility_map
                        .iter()
                        .max_by_key(|(_, v)| v.len())
                        .unwrap();
    (*best_asteroid.0, best_asteroid.1.len())
}
fn main()
{
    let input = read_stdin();
    let asteroid_map = read_asteroid_map(&input);
    let visibility_map = compute_visibility_map(&asteroid_map);
    let part_a = solve_part_a(&visibility_map);
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