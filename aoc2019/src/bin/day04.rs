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
fn get_feasible_number(numstr: String) -> String
{
    let first_digit = '0';
    let init_state = (first_digit, true);
    numstr.chars().scan(init_state, |state, x| {
        let (last_digit, number_ok) = *state;
        if x >= last_digit && number_ok
        {
            *state = (x, true);
            Some(x)
        }else{
            // If one digit fails, then just fill in remaining digits
            // by setting number_ok to false 
            *state = (last_digit, false);
            Some(last_digit)
        }
    }).collect::<String>()
}
fn get_numbers_with_increasing_digits(lo: i32, hi: i32) -> Vec<String>
{
    let mut output = Vec::new();
    let mut num = lo;
    while num <= hi
    {
        let numstr = get_feasible_number(num.to_string());
        num = numstr.parse().unwrap();
        output.push(numstr);
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
                        .filter(|numstr| part_a_test(&numstr))
                        .count();
    let answer_b = candidates.iter()
                        .filter(|numstr| part_b_test(&numstr))
                        .count();
    println!("Part A: {}", answer_a);
    println!("Part B: {}", answer_b);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unit_tests_day_04_get_feasible_number()
    {
        assert_eq!(get_feasible_number("111111".to_string()), "111111");
        assert_eq!(get_feasible_number("123456".to_string()), "123456");
        assert_eq!(get_feasible_number("121111".to_string()), "122222");
        assert_eq!(get_feasible_number("123111".to_string()), "123333");
        assert_eq!(get_feasible_number("123411".to_string()), "123444");
        assert_eq!(get_feasible_number("123451".to_string()), "123455");
    }
    #[test]
    fn unit_tests_day_04_part_a() {
        assert!(part_a_test("111111") == true);
        assert!(part_a_test("111123") == true);
        assert!(part_a_test("122345") == true);
        // assert!(is_valid_password_A("223450") == false);
        assert!(part_a_test("123789") == false);
    }
    #[test]
    fn unit_tests_day_04_part_b() {
        assert!(part_b_test("112233") == true);
        assert!(part_b_test("111122") == true);
        assert!(part_b_test("123444") == false);
    }
}