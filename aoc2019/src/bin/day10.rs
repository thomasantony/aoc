use ::aoc2019::*;
use std::collections::HashMap;
/// Reference: https://youtu.be/p-xa-3V5KO8 for ideas about gradients in discrete grid
use std::f32::consts::FRAC_PI_2;

type Point = (usize, usize);
type Gradient = (i32, i32);
type GradMap = HashMap<Gradient, Vec<Point>>;

fn read_asteroid_map(input: &String) -> Vec<Point> {
    let lines = get_lines(input);
    let mut points = Vec::new();
    for (j, line) in lines.enumerate() {
        line.chars()
            .enumerate()
            .filter(|(_, p)| *p == '#')
            .for_each(|(i, _)| points.push((i, j)));
    }
    points
}
fn compute_gradient_and_distance(a: &Point, b: &Point) -> (Gradient, i32) {
    let grad = (a.1 as i32 - b.1 as i32, a.0 as i32 - b.0 as i32);
    // GCD of gradient is proxy for manhattan distance
    // (except when it is zero, in which case it can be considered unity)
    let gcd = num::integer::gcd(grad.0, grad.1);
    if gcd != 0 {
        ((grad.0 / gcd, grad.1 / gcd), gcd)
    } else {
        (grad, 1)
    }
}
/// Computes a HashMap with a gradient as key and vector of points sorted by distance as the value
fn compute_gradient_map(point: &Point, asteroid_map: &Vec<Point>) -> GradMap {
    let mut grad_map = HashMap::new();
    for j in 0..asteroid_map.len() {
        let p2 = asteroid_map[j];
        if point == &p2 {
            continue;
        }
        let (gradient, dist) = compute_gradient_and_distance(&point, &p2);

        let entry = grad_map.entry(gradient).or_insert(Vec::new());
        entry.push((dist, p2));
    }
    // Sort each vector by distance and remove the extra "distance" value in each item
    let output = grad_map
        .into_iter()
        .map(|(k, mut v)| {
            v.sort();
            let v = v.iter().map(|item| item.1).collect::<Vec<Point>>();
            (k, v)
        })
        .collect();
    output
}
fn compute_visibility_map(asteroid_map: &Vec<Point>) -> HashMap<Point, GradMap> {
    let mut visibility_map = HashMap::new();
    for i in 0..asteroid_map.len() {
        let p1 = asteroid_map[i];
        let grad_map = compute_gradient_map(&p1, asteroid_map);
        visibility_map.insert(p1, grad_map);
    }
    visibility_map
}
/// Computes the asteroid with the most other asteroids visible
fn solve_part_a(visibility_map: &HashMap<Point, GradMap>) -> (Point, usize) {
    let best_asteroid = visibility_map.iter().max_by_key(|(_, v)| v.len()).unwrap();
    (*best_asteroid.0, best_asteroid.1.len())
}
/// Compute the order in which asteroids are destroyed based on gradient map
/// based around a point
/// Consumes gradient map that is passed in
fn solve_part_b(mut visibility_map: GradMap, num_asteroids: usize) -> Vec<Point> {
    let mut gradients: Vec<Gradient> = visibility_map.keys().cloned().collect();

    gradients.sort_by_key(|a| {
        let (y1, x1) = (a.0 as f32, a.1 as f32);
        // Transform slope so that "0" is up, and increases clockwise
        let slope = -(FRAC_PI_2 - y1.atan2(x1));
        let mut slope = slope.to_degrees().round() as i32;
        if slope < 0 {
            slope = slope + 360;
        }
        slope
    });

    // Loop through gradients infinitely until n asteroids are hit
    // Assumes there's enough asteroids
    let mut i = 0;

    let mut destroyed_asteroids = Vec::new();
    for grad in gradients.into_iter().cycle() {
        // Get vector of asteroids in current line
        let entry = visibility_map.entry(grad).or_default();

        if entry.len() > 0 {
            let asteroid_target = entry.remove(0);
            destroyed_asteroids.push(asteroid_target);
            // Found an asteroid along this line
            i += 1;
            if i >= num_asteroids {
                break;
            }
        } else {
            continue;
        }
    }
    destroyed_asteroids
}
fn main() {
    let input = read_stdin();
    let asteroid_map = read_asteroid_map(&input);
    let visibility_map = compute_visibility_map(&asteroid_map);
    let part_a = solve_part_a(&visibility_map);
    println!("{:?}", &visibility_map.get(&(11, 19)).unwrap().len());
    println!("{:?}", &visibility_map.get(&(11, 18)).unwrap().len());
    println!("Part A: {:?} with {} visible", part_a.0, part_a.1);

    let grad_map = visibility_map.get(&part_a.0).cloned().unwrap();
    let destroyed_asteroids = solve_part_b(grad_map, 299);
    let part_b = destroyed_asteroids[199];
    println!("Part_B:");
    println!("200th Asteroid: {:?}", part_b);
    println!("Answer: {}", part_b.0 * 100 + part_b.1);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day10_part_a() {
        let test_cases = vec![
            (
                ".#..# \
                           ..... \
                           ##### \
                           ....# \
                           ...##",
                ((3, 4), 8),
            ),
            (
                "......#.#. \
                           #..#.#.... \
                           ..#######. \
                           .#.#.###.. \
                           .#..#..... \
                           ..#....#.# \
                           #..#....#. \
                           .##.#..### \
                           ##...#..#. \
                           .#....####",
                ((5, 8), 33),
            ),
            (
                "#.#...#.#. \
                           .###....#. \
                           .#....#... \
                           ##.#.#.#.# \
                           ....#.#.#. \
                           .##..###.# \
                           ..#...##.. \
                           ..##....## \
                           ......#... \
                           .####.###.",
                ((1, 2), 35),
            ),
            (
                ".#..#..### \
                           ####.###.# \
                           ....###.#. \
                           ..###.##.# \
                           ##.##.#.#. \
                           ....###..# \
                           ..#.#..#.# \
                           #..#.#.### \
                           .##...##.# \
                           .....#.#..",
                ((6, 3), 41),
            ),
            (
                ".#..##.###...####### \
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
                           ###.##.####.##.#..##",
                ((11, 13), 210),
            ),
        ];

        for (input, expected_output) in test_cases.into_iter() {
            let map = read_asteroid_map(&String::from(input));
            let map = compute_visibility_map(&map);
            let output = solve_part_a(&map);
            assert_eq!(output, expected_output);
        }
    }
    #[test]
    fn test_day10_part_b() {
        let input = ".#..##.###...####### \
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
                           ###.##.####.##.#..##"
            .to_string();
        let map = read_asteroid_map(&input);
        let v_map = compute_visibility_map(&map);
        let gradmap = v_map.get(&(11, 13)).cloned().unwrap();
        let result = solve_part_b(gradmap, 299);
        assert_eq!(result[0], (11, 12));
        assert_eq!(result[199], (8, 2));
        assert_eq!(result[298], (11, 1));
    }
}
