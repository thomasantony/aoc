mod utils;

fn main() {
    let buffer = utils::read_input();
    let (mut two_count, mut three_count) = (0, 0);

    for line in &buffer {
        let freq = utils::get_char_frequency(line);

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
