mod utils;
use std::collections::HashMap;

fn get_char_frequency(line : &String) -> HashMap<char, i32> {
    let mut frequency: HashMap<char, i32> = HashMap::new();
    for c in line.chars() {
        *frequency.entry(c).or_insert(0) += 1;
    }
    frequency
}

fn main() {
    let all_lines = utils::read_input();
    let (mut two_count, mut three_count) = (0, 0);

    for line in &all_lines {
        let freq = get_char_frequency(line);

        if freq.values().any(|x| *x == 2){
            two_count += 1;
        }
        if freq.values().any(|x| *x == 3)
        {
            three_count += 1;
        }
    }
    println!("Answer is: {}", two_count * three_count);
}
