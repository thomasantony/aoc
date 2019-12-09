#[macro_use]
extern crate itertools;

fn pairwise_iter(password: &str, len: usize) -> impl Iterator<Item=(char, char)> + '_
{
    password[..(len-1)].chars().zip(password[1..].chars())
}
fn part_a_test(password: &str) -> bool
{
    let has_repeating_digits = pairwise_iter(password, 6).any(|(d1, d2)| d1 == d2);
    return has_repeating_digits
}
fn part_b_test(password: &str) -> bool
{
    let mut i:usize = 0;
    loop
    {
        if i >= password.len()
        {
            break;
        }
        let c = password.chars().nth(i).unwrap();
        let repeating_substr:Vec<char> = password[i..].chars()
                                        .take_while(|next_c| *next_c == c)
                                        .collect();

        if repeating_substr.len() == 2
        {
            return true;
        }
        i += repeating_substr.len();
    }
    return false;
}
fn get_numbers_with_increasing_digits(lo: i32, hi: i32) -> Vec<i32>
{
    let mut output = Vec::new();
    let mut num = lo;
    while num <= hi
    {
        let numstr = num.to_string();
        let first = numstr.chars().nth(0).unwrap();
        let valid_chars = numstr.chars().scan((first, true), |state, x| {
            let (last_digit, number_ok) = *state;
            if x >= last_digit && number_ok
            {
                *state = (x, true);
                Some(x)
            }else{
                // If one digit fails, then just fill in remaining digits
                *state = (last_digit, false);
                Some(last_digit)
            }
        });
        let numstr = valid_chars.collect::<String>();

        num = numstr.parse().unwrap();
        output.push(num);
        num += 1;
    }
    output
}
fn main()
{
    let min_val = 124075;
    let max_val = 580769;
    
    let candidates = get_numbers_with_increasing_digits(min_val, max_val);
    
    let answer_a = candidates.iter()
                        .map(|n| n.to_string())
                        .filter(|numstr| part_a_test(&numstr))
                        .count();
    let answer_b = candidates.iter()
                        .map(|n| n.to_string())
                        .filter(|numstr| part_b_test(&numstr))
                        .count();
    println!("Part A: {}", answer_a);
    println!("Part B: {}", answer_b);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unit_tests_day_04_part_A() {
        assert!(part_a_test("111111") == true);
        assert!(part_a_test("111123") == true);
        assert!(part_a_test("122345") == true);
        // assert!(is_valid_password_A("223450") == false);
        assert!(part_a_test("123789") == false);
    }
    #[test]
    fn unit_tests_day_04_part_B() {
        assert!(part_b_test("112233") == true);
        assert!(part_b_test("111122") == true);
        assert!(part_b_test("123444") == false);
    }
}