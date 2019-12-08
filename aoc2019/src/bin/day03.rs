use ::aoc2019::*;
#[macro_use]
extern crate itertools;

use std::convert::From;
use std::ops;
use std::cmp::{min, max};
use std::fmt;

#[derive(Debug, Clone, Copy)]
enum StepDirection {
    LEFT,
    RIGHT,
    UP,
    DOWN
}

impl From<&str> for StepDirection{
    fn from(dir_str: &str) -> StepDirection
    {
        match dir_str {
            "L" => StepDirection::LEFT,
            "R" => StepDirection::RIGHT,
            "U" => StepDirection::UP,
            "D" => StepDirection::DOWN,
            _ => panic!("Invalid step direction!")
        }
    }
}
#[derive(Copy, Clone)]
struct Point{
    x: i32,
    y: i32
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, _rhs: Point) -> Point {
        Point{
            x: self.x + _rhs.x,
            y: self.y + _rhs.y
        }
    }
}
impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, _rhs: Point) -> Point {
        Point{
            x: self.x - _rhs.x,
            y: self.y - _rhs.y
        }
    }
}
impl ops::Mul<i32> for Point {
    type Output = Point;

    fn mul(self, _rhs: i32) -> Point {
        Point{
            x: self.x * _rhs,
            y: self.y * _rhs
        }
    }
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point
}
#[derive(Debug)]
struct Path {
    end: Point,
    v_lines: Vec<Line>,
    h_lines: Vec<Line>,
}

impl Path
{
    fn new() -> Self{
        Path {
            end: Point{x:0, y:0},
            v_lines: Vec::new(),
            h_lines: Vec::new()
        }
    }
    fn get_unit_vector(direction: StepDirection) -> Point
    {
        match direction {
            StepDirection::UP => Point{x:0, y:1},
            StepDirection::DOWN => Point{x:0, y:-1},
            StepDirection::LEFT => Point{x:-1, y:0},
            StepDirection::RIGHT => Point{x:1, y:0},
        }
    }
    fn step(&mut self, direction: StepDirection, size: i32)
    {
        let unit_vec = Path::get_unit_vector(direction);
        let new_point = self.end + unit_vec * size;
        let new_line = Line{p1: self.end, p2: new_point};

        match direction {
            StepDirection::UP | StepDirection::DOWN => self.v_lines.push(new_line),
            StepDirection::LEFT | StepDirection::RIGHT => self.h_lines.push(new_line)
        };
        self.end = new_point;
    }
}

fn h_v_line_intersection(h_line: &Line, v_line: &Line) -> Option<Point> {
    assert_eq!(h_line.p1.y, h_line.p2.y);
    assert_eq!(v_line.p1.x, v_line.p2.x);

    //Check if lines intersect and returns intersection if they do
    let x_min = min(h_line.p1.x, h_line.p2.x);
    let x_max = max(h_line.p1.x, h_line.p2.x);
    let y_min = min(v_line.p1.y, v_line.p2.y);
    let y_max = max(v_line.p1.y, v_line.p2.y);

    let lines_cross = x_min < v_line.p1.x && v_line.p1.x < x_max 
                    && y_min < h_line.p1.y && h_line.p1.y < y_max;
    if lines_cross
    {
        Some(Point{
            x: v_line.p1.x,
            y: h_line.p1.y
        })
    }else{
        None
    }
}

fn parse_paths(lines: Vec<&str>) -> Vec<Path>
{
    let path_def_list = lines.iter().map(|line_def_string| line_def_string.split(","));
    let mut paths = Vec::new();
    for path_def in path_def_list 
    {
        let mut path = Path::new();
        
        path_def.map(|step_def_string|
        {
            let dir_str = &step_def_string[0..1];
            let dir = StepDirection::from(dir_str);
            let size = &step_def_string[1..].parse::<i32>().unwrap();
            (dir, *size)
        }).for_each(|(dir, size)| {
            path.step(dir, size);
        });
        paths.push(path)
    }
    paths
}
fn calc_min_dist(paths: Vec<Path>) -> Option<i32>
{
    let h1 = &paths[0].h_lines;
    let h2 = &paths[1].h_lines;
    let v1 = &paths[0].v_lines;
    let v2 = &paths[1].v_lines;
    
    let h1v2 = iproduct!(h1, v2);
    let h2v1 = iproduct!(h2, v1);

    let all_intersections = h1v2.chain(h2v1)
                                .map(|(h, v)| h_v_line_intersection(h, v))
                                .flatten()     
                                .filter(|pt| pt.x != 0 && pt.y != 0);
    let dist = all_intersections.map(|pt| pt.x.abs() + pt.y.abs());
    dist.min()
}
fn main() 
{
    let input = read_stdin();
    let lines:Vec<&str> = input.split_ascii_whitespace().collect();
    let paths = parse_paths(lines);
    let dist = calc_min_dist(paths);
    println!("Part A: {:?}", dist.expect("Failed to find min distance"));
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unit_tests_day_03() {
        let lines = vec!("R8,U5,L5,D3", "U7,R6,D4,L4");
        let paths = parse_paths(lines);
        assert_eq!(calc_min_dist(paths), Some(6));

        let lines = vec!("R75,D30,R83,U83,L12,D49,R71,U7,L72",
                            "U62,R66,U55,R34,D71,R55,D58,R83");
        let paths = parse_paths(lines);
        assert_eq!(calc_min_dist(paths), Some(159));

        let lines = vec!("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        let paths = parse_paths(lines);
        assert_eq!(calc_min_dist(paths), Some(135));
    }
}