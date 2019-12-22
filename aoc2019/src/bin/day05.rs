use ::aoc2019::intcode::*;
use ::aoc2019::*;

fn main() {
    let input = read_stdin();
    let data: Vec<i32> = parse_numbers_with_delimiter(&input, ',').collect();
    let mut vm = IntComputer::new();

    let output_a = vm.load_program(&data).set_input(1).execute();
    println!("Part A: {:?}", output_a.1);

    let output_b = vm.load_program(&data).set_input(5).execute();
    println!("Part B: {:?}", output_b.1);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unit_tests_day_05() {
        let mut vm = IntComputer::new();
        // Program that outputs 1 if input equal to 8 and zero otherwise
        assert_eq!(
            vm.load_program(&vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8])
                .set_input(8)
                .execute()
                .1,
            1
        );
        assert_eq!(
            vm.load_program(&vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8])
                .set_input(7)
                .execute()
                .1,
            0
        );

        // Program that outputs 1 if input is less than 8 and zero otherwise
        assert_eq!(
            vm.load_program(&vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8])
                .set_input(7)
                .execute()
                .1,
            1
        );
        assert_eq!(
            vm.load_program(&vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8])
                .set_input(8)
                .execute()
                .1,
            0
        );

        // Program that outputs 1 if input equal to 8 and zero otherwise
        assert_eq!(
            vm.load_program(&vec![3, 3, 1108, -1, 8, 3, 4, 3, 99])
                .set_input(8)
                .execute()
                .1,
            1
        );
        assert_eq!(
            vm.load_program(&vec![3, 3, 1108, -1, 8, 3, 4, 3, 99])
                .set_input(7)
                .execute()
                .1,
            0
        );

        // Program that outputs 1 if input is less than 8 and zero otherwise
        assert_eq!(
            vm.load_program(&vec![3, 3, 1107, -1, 8, 3, 4, 3, 99])
                .set_input(7)
                .execute()
                .1,
            1
        );
        assert_eq!(
            vm.load_program(&vec![3, 3, 1107, -1, 8, 3, 4, 3, 99])
                .set_input(8)
                .execute()
                .1,
            0
        );

        // Program that outputs zero if input is zero and one if non-zero
        assert_eq!(
            vm.load_program(&vec![
                3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9
            ])
            .set_input(0)
            .execute()
            .1,
            0
        );
        assert_eq!(
            vm.load_program(&vec![
                3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9
            ])
            .set_input(1)
            .execute()
            .1,
            1
        );

        // Program that outputs zero if input is zero and one if non-zero
        assert_eq!(
            vm.load_program(&vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1])
                .set_input(0)
                .execute()
                .1,
            0
        );
        assert_eq!(
            vm.load_program(&vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1])
                .set_input(1)
                .execute()
                .1,
            1
        );

        let large_program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(
            vm.load_program(&large_program).set_input(7).execute().1,
            999
        );
        assert_eq!(
            vm.load_program(&large_program).set_input(8).execute().1,
            1000
        );
        assert_eq!(
            vm.load_program(&large_program).set_input(9).execute().1,
            1001
        );
    }
}
