use ::aoc2019::intcode::*;
use ::aoc2019::{parse_numbers_with_delimiter};
use std::collections::HashMap;

type Coord = (i64, i64);
type Heading = (i64, i64);
type Grid = HashMap<Coord, (PanelColor, usize)>;

#[derive(Debug, Copy, Clone)]
    enum PanelColor {
    Black = 0,
    White = 1,
}
impl PanelColor {
    fn from_i64(value: i64) -> PanelColor {
        match value {
            0 => PanelColor::Black,
            1 => PanelColor::White,
            _ => panic!("Unknown value: {}", value),
        }
    }
}
#[derive(Debug, Copy, Clone)]
enum TurnCommand {
    Left = 0,
    Right = 1,
}

impl TurnCommand {
    fn from_i64(value: i64) -> TurnCommand {
        match value {
            0 => TurnCommand::Left,
            1 => TurnCommand::Right,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

struct Robot {
    grid: Grid,
    current_heading: Heading,
    current_pos: Coord,
    vm: IntComputer
}


fn rotate(heading: Heading, turn: TurnCommand) -> Heading
{
    use TurnCommand::*;
    // Column-major form
    let transform = match turn {
        Left => (0, -1, 1, 0),
        Right => (0, 1, -1, 0),
    };

    let hdg_0 = transform.0 * heading.0  + transform.1 * heading.1;
    let hdg_1 = transform.2 * heading.0  + transform.3 * heading.1;
    (hdg_0, hdg_1)
}
impl Robot 
{
    pub fn new(program: &Vec<i64>, starting_color: PanelColor) -> Self {
        let mut vm = IntComputer::new();
        vm.set_ram_size(2048);
        vm.load_program(program);

        let mut grid = Grid::new();
        // Set starting color at origin
        grid.insert((0, 0), (starting_color, 0));
        Self {
            vm,
            current_heading: (0, 1), //Up
            current_pos: (0, 0),
            grid
        }
    }
    fn get_current_panel_color(&mut self) -> PanelColor
    {
        let entry = self.grid.entry(self.current_pos)
                    .or_insert((PanelColor::Black, 0));
        entry.0
    }
    fn set_current_panel_color(&mut self, color: PanelColor)
    {
        self.grid.entry(self.current_pos)
                 .and_modify(|v|{
                    let last_count = v.1;
                    *v = (color, last_count + 1);
                });
    }
    pub fn get_command(&mut self, color_of_panel: PanelColor) -> (PanelColor, TurnCommand)
    {
        self.vm.push_input(color_of_panel as i64);
        let output = self.vm.execute();
        let paint_color = PanelColor::from_i64(output[0]);
        let turn_command = TurnCommand::from_i64(output[1]);

        (paint_color, turn_command)
    }
    fn step_forward(&mut self)
    {
        self.current_pos = (self.current_pos.0 + self.current_heading.0,
                            self.current_pos.1 + self.current_heading.1)
    }
    fn turn(&mut self, turn_command: TurnCommand)
    {
        let new_heading = rotate(self.current_heading, turn_command);
        self.current_heading = new_heading;
    }
    pub fn execute(&mut self) -> Grid
    {
        while !self.vm.is_halted() {
            let color = self.get_current_panel_color();
            let (paint_color, turn_command) = self.get_command(color);
            
            // Paint the panel
            self.set_current_panel_color(paint_color);

            // Turn and move forward
            self.turn(turn_command);
            self.step_forward();
            
        }
        
        self.grid.clone()
    }
}

fn show_grid(grid: Grid)
{
    let min_x = grid.keys().map(|k| k.0).min().unwrap();
    let min_y = grid.keys().map(|k| k.1).min().unwrap();
    let max_x = grid.keys().map(|k| k.0).max().unwrap();
    let max_y = grid.keys().map(|k| k.1).max().unwrap();

    let x_range = max_x - min_x;
    let y_range = max_y - min_y;
    for j in 0..(y_range+1)
    {
        let j = min_y + ( y_range - j);
        for i in 0..x_range
        {
            let i = min_x + i;
            use PanelColor::*;
            let (color, _) = grid.get(&(i, j)).unwrap_or(&(PanelColor::Black, 0));
            match color {
                Black => print!("."),
                White => print!("#"),
            }
        }
        println!();
    }
}
fn main()
{
    let input = include_str!("../../inputs/day11.txt").to_string();
    let program: Vec<i64> = parse_numbers_with_delimiter(&input, ',').collect();

    let mut robot = Robot::new(&program, PanelColor::Black);
    let grid = robot.execute();
    let painted_cells = grid.iter().filter(|(_, v)| v.1 > 0).count();
    // show_grid(grid);
    println!("Part A: {:?}", painted_cells);

    println!("\nPart B:");
    let mut robot = Robot::new(&program, PanelColor::White);
    let grid = robot.execute();
    show_grid(grid);
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day11_rotate()
    {
        let up = (0, 1);
        let down = (0, -1);
        let left = (-1, 0);
        let right = (1, 0);
        assert_eq!(rotate(up, TurnCommand::Right), right);
        assert_eq!(rotate(up, TurnCommand::Left), left);

        assert_eq!(rotate(down, TurnCommand::Right), left);
        assert_eq!(rotate(down, TurnCommand::Left), right);

        assert_eq!(rotate(left, TurnCommand::Right), up);
        assert_eq!(rotate(left, TurnCommand::Left), down);

        assert_eq!(rotate(right, TurnCommand::Right), down);
        assert_eq!(rotate(right, TurnCommand::Left), up);
    }
}