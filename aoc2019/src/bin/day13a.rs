use ::aoc2019::parse_numbers_with_delimiter;
use ::aoc2019::arcade::Arcade;

fn main() {
    let input = include_str!("../../inputs/day13.txt").to_string();
    let program: Vec<i64> = parse_numbers_with_delimiter(&input, ',').collect();
    let mut arcade = Arcade::new(&program);

    arcade.run_once();
    println!("Part A: {}", arcade.num_blocks_remaining());

    arcade.insert_quarter();

}