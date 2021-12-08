use std::collections::HashMap;

/// Day 08
///
/// Part 1
/// Count all words with len = 2, 3, 4 or 7
///
/// Part 2
/// What do we know?
///
/// 0: len = 6 x
/// 1: len = 2 *
/// 2: len = 5
/// 3: len = 5
/// 4: len = 4 *
/// 5: len = 5
/// 6: len = 6 x
/// 7: len = 3 *
/// 8: len = 7 *
/// 9: len = 6 x
///
///
/// a: 0 2 3 5 6 7 8 9
/// b: 0 4 5 6 8 9
/// c: 0 1 2 3 4 7 8 9
/// d: 2 3 4 5 6 8 9
/// e: 0 2 6 8
/// f: 0 1 3 4 5 6 7 8 9
/// g: 0 2 3 5 6 8 9
///
/// Remove common chars etc. to figure out every digit's letters
/// Take care to "restore" the items in the iterator before using them (using indices)
///

fn remove_chars(s: &str, chars: &str) -> String {
    chars.chars().fold(s.to_string(), |s, c| {
        let c = c.to_string();
        s.replace(&c, "")
    })
}
fn sort_string(s: &String) -> String {
    let mut c: Vec<_> = s.chars().collect();
    c.sort();
    String::from_iter(c.iter())
}
// dg fadgceb dacbef agfeb gcdbef edcbf gdf ecgd cgbadf defbg | bedcf bgdfac cbfedg abfeg
// 0 6 9
// gcdbef
// dacbef
// cgbadf
fn identify_numbers(input: &Vec<String>) -> HashMap<String, i32> {
    let mut output = HashMap::new();

    // Unique chars
    let one = input
        .iter()
        .filter(|w| w.len() == 2)
        .next()
        .cloned()
        .unwrap();
    let four = input
        .iter()
        .filter(|w| w.len() == 4)
        .next()
        .cloned()
        .unwrap();
    let seven = input
        .iter()
        .filter(|w| w.len() == 3)
        .next()
        .cloned()
        .unwrap();
    let eight = input
        .iter()
        .filter(|w| w.len() == 7)
        .next()
        .cloned()
        .unwrap();

    let cf = one.clone();
    let bd = remove_chars(&four, &one);

    let zero_six_nine: Vec<_> = input.iter().filter(|w| w.len() == 6).collect();
    // Zero - bd should only reduce by one character "d"
    let zero = zero_six_nine
        .iter()
        .enumerate()
        .map(|(i, w)| (i, remove_chars(&w, &bd)))
        .filter(|(_, w)| w.len() == 5)
        .map(|(i, _)| zero_six_nine[i].clone())
        .next()
        .unwrap();

    let six_or_nine: Vec<_> = zero_six_nine
        .iter()
        .enumerate()
        .map(|(i, w)| (i, remove_chars(&w, &bd)))
        .filter(|(_, w)| w.len() == 4)
        .map(|(i, _)| zero_six_nine[i].clone())
        .collect();

    // Six - clearing out cf from six should only reduce len by one
    let six_idx = six_or_nine
        .iter()
        .enumerate()
        .map(|(i, w)| (i, remove_chars(&w, &cf)))
        .filter(|(_, w)| w.len() == 5)
        .map(|(i, _)| i)
        .next()
        .unwrap();
    let six = six_or_nine[six_idx].to_string();

    // Nine - clearing out cf from nine should reduce len by two
    let nine_idx = six_or_nine
        .iter()
        .enumerate()
        .map(|(i, w)| (i, remove_chars(&w, &cf)))
        .filter(|(_, w)| w.len() == 4)
        .map(|(i, _)| i)
        .next()
        .unwrap();
    let nine = six_or_nine[nine_idx].to_string();

    let two_three_five: Vec<_> = input.iter().filter(|w| w.len() == 5).collect();
    // Three - Removing c and f reduced length of list to 3
    let three = two_three_five
        .iter()
        .enumerate()
        .map(|(i, w)| (i, remove_chars(&w, &cf)))
        .filter(|(_, w)| w.len() == 3)
        .map(|(i, _w)| two_three_five[i].clone())
        .next()
        .unwrap();

    let two_five: Vec<_> = two_three_five
        .iter()
        .enumerate()
        .map(|(i, w)| (i, remove_chars(&w, &cf)))
        .filter(|(_, w)| w.len() == 4)
        .map(|(i, _)| two_three_five[i].clone())
        .collect();

    // Five - clearing out cfbd(four) from five should reduce len to 2
    let five_idx = two_five
        .iter()
        .enumerate()
        .map(|(i, w)| (i, remove_chars(&w, &four)))
        .filter(|(_, w)| w.len() == 2)
        .map(|(i, _)| i)
        .next()
        .unwrap();
    let five = two_five[five_idx].to_string();

    // Two - clearing out bd from two should reduce len to 4
    let two_idx = two_five
        .iter()
        .enumerate()
        .map(|(i, w)| (i, remove_chars(&w, &four)))
        .filter(|(_, w)| w.len() == 3)
        .map(|(i, _)| i)
        .next()
        .unwrap();
    let two = two_five[two_idx].to_string();

    output.insert(sort_string(&zero), 0);
    output.insert(sort_string(&one), 1);
    output.insert(sort_string(&two), 2);
    output.insert(sort_string(&three), 3);
    output.insert(sort_string(&four), 4);
    output.insert(sort_string(&five), 5);
    output.insert(sort_string(&six), 6);
    output.insert(sort_string(&seven), 7);
    output.insert(sort_string(&eight), 8);
    output.insert(sort_string(&nine), 9);

    output
}
fn compute_value(number: &Vec<String>, number_map: HashMap<String, i32>) -> i32 {
    let mut output = 0;
    for digit in number.iter() {
        let digit = sort_string(digit);
        if let Some(d) = number_map.get(&digit) {
            output = output * 10 + d;
        } else {
            println!("FAIL!");
        }
    }
    output
}
/// Process a line (with LHS and RHS) and returns the value of the RHS
fn process_line(line: &(&str, &str)) -> i32 {
    let lhs = line
        .0
        .split_ascii_whitespace()
        .map(String::from)
        .collect::<Vec<_>>();
    let number_map = identify_numbers(&lhs);
    let rhs = line
        .1
        .split_ascii_whitespace()
        .map(String::from)
        .collect::<Vec<_>>();
    compute_value(&rhs, number_map)
}
fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .trim()
        .lines()
        .map(|s| {
            let split_parts = s.split(" | ").collect::<Vec<_>>();
            (split_parts[0], split_parts[1])
        })
        .collect()
}
fn main() {
    let input = include_str!("../../../inputs/day08.txt");
    let input_lines = parse_input(input);

    let mut ctr = 0;
    for (_, rhs) in input_lines.iter() {
        ctr += rhs
            .split_ascii_whitespace()
            .filter(|w| {
                let l = w.len();
                l == 2 || l == 3 || l == 4 || l == 7
            })
            .count();
    }
    println!("Part 1: {}", ctr);

    let part02_sol: i32 = input_lines.iter().map(process_line).sum();
    println!("Part 2: {}", part02_sol);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day08_test_part_02() {
        let demo_input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        assert_eq!(
            parse_input(demo_input)
                .iter()
                .map(process_line)
                .sum::<i32>(),
            5353
        );

        let demo_input = "dg fadgceb dacbef agfeb gcdbef edcbf gdf ecgd cgbadf defbg | bedcf bgdfac cbfedg abfeg";
        assert_eq!(
            parse_input(demo_input)
                .iter()
                .map(process_line)
                .sum::<i32>(),
            5092
        );
    }
}
