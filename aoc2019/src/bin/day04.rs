fn pairwise_iter(password: &str, len: usize) -> impl Iterator<Item=(char, char)> + '_
{
    password[..(len-1)].chars().zip(password[1..].chars())
}
fn is_valid_password(password: &str) -> bool
{
    let has_repeating_digits = pairwise_iter(password, 6).any(|(d1, d2)| d1 == d2);
    let is_increasing = pairwise_iter(password, 6).all(|(d1, d2)| d1 <= d2);
    return has_repeating_digits && is_increasing
}
fn main()
{
    let min_val = 124075;
    let max_val = 580769;
    let range = min_val..max_val;
    let answer = range.map(|num| num.to_string())
                      .filter(|passwd| is_valid_password(passwd))
                      .count();
    println!("Part A: {}", answer)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unit_tests_day_04() {
        assert!(is_valid_password("111111") == true);
        assert!(is_valid_password("111123") == true);
        assert!(is_valid_password("122345") == true);
        assert!(is_valid_password("223450") == false);
        assert!(is_valid_password("123789") == false);
    }
}