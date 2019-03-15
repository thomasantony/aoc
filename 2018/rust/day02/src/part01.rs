use std::io;
use std::io::Read;
use std::collections::HashMap;

fn read_input() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).ok().expect("read error");
    let _numbers: Vec<String> = buffer
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    _numbers
}

fn get_char_frequency(line : &String) -> HashMap<char, i32> {
    let mut frequency: HashMap<char, i32> = HashMap::new();
    for c in line.chars() {
        *frequency.entry(c).or_insert(0) += 1;
    }
    frequency
}

fn main() {
    let buffer = read_input();
    let (mut two_count, mut three_count) = (0, 0);

    for line in &buffer {
        let freq = get_char_frequency(line);

        if freq.values().any(|x| *x == 2){
            two_count += 1;
        }
        if freq.values().any(|x| *x == 3)
        {
            three_count += 1;
        }
        // let (mut two_found, mut three_found) = (false, false);
        // for val in freq.values()
        // {
        //     if !two_found && *val == 2
        //     {
        //         two_count += 1;
        //         two_found = true;
        //     }
        //     if !three_found && *val == 3
        //     {
        //         three_count += 1;
        //         three_found = true;
        //     }
        // }
    }
    println!("Answer is: {}", two_count * three_count);
}
