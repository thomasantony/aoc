use crate::intcode::{CpuState, IntComputer};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::thread;
use std::time::{Duration, Instant};
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

pub type Coord = (i64, i64);
pub type Grid = HashMap<Coord, Tile>;
const SCORE_COORD: (i64, i64) = (-1, 0);
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}
impl Tile {
    fn from_i64(value: i64) -> Tile {
        use Tile::*;
        match value {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => Paddle,
            4 => Ball,
            _ => panic!("Unknown value: {}", value),
        }
    }
}
#[derive(Debug)]
pub enum MoveCommand {
    Neutral = 0,
    Left = -1,
    Right = 1,
}

pub struct Arcade {
    pub grid: Grid,
    program: Vec<i64>,
    vm: IntComputer,
    pub score: i64,
    ball_pos: Coord,
    paddle_pos: Coord,
    autopilot_enabled: bool,
}
impl Arcade {
    pub fn new(program: &Vec<i64>) -> Self {
        let mut vm = IntComputer::new();
        vm.set_ram_size(1048576);
        let mut program = program.clone();
        program[0] = 2; // Insert quarter
        vm.load_program(&program);
        Self {
            grid: Grid::new(),
            program,
            vm,
            score: 0,
            ball_pos: Coord::default(),
            paddle_pos: Coord::default(),
            autopilot_enabled: true,
        }
    }
    pub fn start_game(&mut self) {
        let mut stdin = termion::async_stdin();
        let stdout = std::io::stdout();
        let mut stdout = stdout.lock().into_raw_mode().unwrap();

        let speed = 15;
        write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

        let mut before = Instant::now();

        loop {
            let interval = 1000 / speed;
            let now = Instant::now();
            let dt = (now.duration_since(before).subsec_nanos() / 1_000_000) as u64;

            if dt < interval {
                thread::sleep(Duration::from_millis(interval - dt));
                continue;
            }
            before = now;
            // Update state
            let mut key_bytes = [0];
            stdin.read(&mut key_bytes).unwrap();

            match key_bytes[0] {
                b'q' => break,
                b'A' | b'a' => self.command(MoveCommand::Left),
                b'D' | b'd' => self.command(MoveCommand::Right),
                b'Z' | b'z' => self.toggle_autopilot(),
                _ => self.command(MoveCommand::Neutral),
            }

            self.run_once();
            self.draw(&mut stdout);

            if self.vm.cpu_state == CpuState::HALTED {
                break;
            }
        }
        write!(
            stdout,
            "{}{}{}",
            clear::All,
            style::Reset,
            cursor::Goto(1, 1)
        )
        .ok();
        stdout.flush().unwrap();
    }
    pub fn draw<W: Write>(&self, mut out: W) {
        let offset = (21, 2);
        for (pos, tile) in self.grid.iter() {
            let pos = cursor::Goto(pos.0 as u16 + offset.0, pos.1 as u16 + offset.1);
            use Tile::*;
            let cell_data = match tile {
                Block => {
                    format!("{}{}█", pos, color::Fg(color::Blue))
                }
                Paddle => {
                    format!("{}{}=", pos, color::Fg(color::Red))
                }
                Ball => {
                    format!("{}{}o", pos, color::Fg(color::Yellow))
                }
                Empty => {
                    format!("{}{} ", pos, color::Fg(color::Black))
                }
                Wall => {
                    format!("{}{}█", pos, color::Fg(color::White))
                }
            };
            write!(out, "{}", cell_data).ok();
        }
        write!(
            out,
            "{}{}Score: {}",
            cursor::Goto(2, 2),
            color::Fg(color::White),
            self.score
        )
        .ok();

        let autopilot_status = if self.autopilot_enabled {
            format!("Autopilot: {}{:<3}", color::Fg(color::Green), "On")
        } else {
            format!("Autopilot: {}{:<3}", color::Fg(color::Red), "Off")
        };
        write!(out, "{}{}", cursor::Goto(2, 4), autopilot_status).ok();
        let status = format!(
            "{}{}  {}{}[A]: Left, {}[D]: Right, {}[Z] Toggle Autopilot {}[Q] Quit{:^26}",
            cursor::Goto(1, 25),
            color::Fg(color::White),
            color::Fg(color::Black),
            color::Bg(color::White),
            color::Bg(color::White),
            color::Bg(color::White),
            color::Bg(color::White),
            ' '
        );
        write!(out, "{:^80}", status).ok();
        write!(out, "{}", style::Reset).ok();
        out.flush().unwrap();
    }
    pub fn reset(&mut self) {
        self.vm.reset();
        self.vm.load_program(&self.program);
        self.grid.clear();
    }
    pub fn insert_quarter(&mut self) {
        self.program[0] = 2;
        self.reset();
    }
    pub fn command(&mut self, command: MoveCommand) {
        // Only send if Autopilot is disabled
        if !self.autopilot_enabled {
            let cmd_input = command as i64;
            self.vm.push_input(cmd_input);
        }
    }
    pub fn toggle_autopilot(&mut self) {
        self.autopilot_enabled = !self.autopilot_enabled;
    }
    pub fn num_blocks_remaining(&self) -> usize {
        self.grid.values().filter(|&t| t == &Tile::Block).count()
    }
    pub fn num_empty_remaining(&self) -> usize {
        self.grid.values().filter(|&t| t == &Tile::Empty).count()
    }
    pub fn run_once(&mut self) {
        let output = self.vm.execute();
        self.grid.clear();
        for command in output.chunks(3) {
            if command.len() != 3 {
                continue;
            }
            let pos = (command[0], command[1]);

            if pos == SCORE_COORD {
                self.score = command[2];
            } else {
                let tile = Tile::from_i64(command[2]);
                match tile {
                    Tile::Ball => {
                        self.ball_pos = pos;
                    }
                    Tile::Paddle => {
                        self.paddle_pos = pos;
                    }
                    _ => {}
                }
                self.grid.insert(pos, tile);
            }
        }
        // Simple AI
        if self.autopilot_enabled {
            let mut command = self.ball_pos.0 - self.paddle_pos.0;
            if command != 0 {
                command = command / command.abs();
            }
            self.vm.push_input(command);
        }
    }
}
