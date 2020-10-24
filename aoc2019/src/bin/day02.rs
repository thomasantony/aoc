use ::aoc2019::intcode::*;
use ::aoc2019::*;

fn main() {
    let input = read_stdin();
    let data: Vec<i64> = parse_numbers_with_delimiter(&input, ',').collect();
    let mut vm = IntComputer::new();
    let output_a = vm.load_program(&data).set_noun(12).set_verb(2).execute();
    println!("Part A: {:?}", output_a);

    for noun in 0..99 {
        for verb in 0..99 {
            let output = vm
                .load_program(&data)
                .set_noun(noun)
                .set_verb(verb)
                .execute();
            if output[0] == 19690720 {
                println!("Part B: {}", noun * 100 + verb);
            }
        }
    }
}
