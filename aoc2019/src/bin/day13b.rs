use ::aoc2019::arcade::Arcade;
use ::aoc2019::parse_numbers_with_delimiter;

fn main() {
    let input = include_str!("../../inputs/day13.txt").to_string();
    let program: Vec<i64> = parse_numbers_with_delimiter(&input, ',').collect();
    let mut arcade = Arcade::new(&program);
    arcade.start_game();
    println!("Final score is {}", arcade.score);
}
