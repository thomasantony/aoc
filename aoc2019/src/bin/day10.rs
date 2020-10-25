/// Reference: http://www-cs-students.stanford.edu/~amitp/Articles/LineOfSight.html
use std::f32::consts::PI;
use std::collections::{HashMap, HashSet};
use ::aoc2019::*;

type Point = (i32, i32);
fn approx_equal(a: f32, b: f32, decimal_places: u8) -> bool {
    let factor = 10.0f32.powi(decimal_places as i32);
    let a = (a * factor).trunc();
    let b = (b * factor).trunc();
    a == b
}
fn read_asteroid_map<'a, V: Iterator<Item=&'a str>>(lines: V) -> Vec<Point>
{
    let mut points = Vec::new();
    for (j, line) in lines.enumerate() 
    {
        line.chars()
            .enumerate()
            .filter(|(_, p)| *p == '#')
            .for_each(|(i, _)| {
                points.push((i as i32, j as i32))
            });
    }
    points
}
fn main()
{
    let grid_size = (5, 5);
    let num_rays = grid_size.0 * grid_size.1;
    let angle_step = 2.0 * PI / (num_rays as f32);
    let angles = (0..num_rays).map(|t| (t as f32)* angle_step);

    let input = read_stdin();
    let lines = get_lines(&input);

    let asteroid_map = read_asteroid_map(lines);
    println!("Points: {:?}", asteroid_map);
}