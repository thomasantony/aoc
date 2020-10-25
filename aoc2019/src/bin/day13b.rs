use ::aoc2019::parse_numbers_with_delimiter;
use ::aoc2019::arcade::{Arcade, Bounds, Tile, MoveCommand};
use termion::{color, cursor, clear, style};
use termion::raw::IntoRawMode;
use std::io::{Stdout};
use std::io::{Write, Read};
use std::thread;
use std::time::{Instant, Duration};


fn render_arcade<W: Write>(mut out: W, arcade: &Arcade)
{
    for (pos, tile) in arcade.grid.iter()
    {
        use Tile::*;
        match tile
        {
            Block => {
                write!(out, "{}{}█", cursor::Goto((pos.0 as u16)+1, (pos.1 as u16)+1), color::Fg(color::White)).ok();
            },
            Paddle => {
                write!(out, "{}{}=", cursor::Goto((pos.0 as u16)+1, (pos.1 as u16)+1), color::Fg(color::Blue)).ok();
            },
            Ball => {
                write!(out, "{}{}o", cursor::Goto((pos.0 as u16)+1, (pos.1 as u16)+1), color::Fg(color::Yellow)).ok();
            }
            _ => {}
        }
    }
}
fn game_loop(arcade: &mut Arcade, bounds: Bounds)
{
    let mut stdin = termion::async_stdin();
    // let mut stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    let speed = 10;
    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
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
            b'A' | b'a' => arcade.command(MoveCommand::Left),
            b'D' | b'd' => arcade.command(MoveCommand::Right),
            _ => arcade.command(MoveCommand::Neutral),
        }

        arcade.run_once();

        write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
        render_arcade(&mut stdout, &arcade);
        write!(stdout, "{}", style::Reset).ok();
        stdout.flush().unwrap();
    }
    write!(stdout, "{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1)).ok();
    stdout.flush().unwrap();
}
fn main() {
    let input = include_str!("../../inputs/day13.txt").to_string();
    let program: Vec<i64> = parse_numbers_with_delimiter(&input, ',').collect();
    let mut arcade = Arcade::new(&program);

    arcade.insert_quarter();
    arcade.run_once();
    let bounds = arcade.get_bounds();
    game_loop(&mut arcade, bounds)
}