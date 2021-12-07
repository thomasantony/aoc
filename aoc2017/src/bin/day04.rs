use std::collections::HashSet;

fn is_valid_password_part01(passwd: &str) -> bool {
    let words = passwd.split(' ').collect::<Vec<_>>();
    let uniq = HashSet::<&str>::from_iter(words.iter().cloned());
    return uniq.len() == words.len();
}

fn is_valid_password_part02(passwd: &str) -> bool {
    let words = passwd.split(' ').collect::<Vec<_>>();

    let words_with_sorted_chars = words.iter().map(|&w| {
        let mut c = w.chars().collect::<Vec<_>>();
        c.sort();
        c.into_iter().collect::<String>()
    });
    let uniq = HashSet::<String>::from_iter(words_with_sorted_chars.into_iter());
    return uniq.len() == words.len();
}

fn main() {
    let input = include_str!("../../inputs/day04.txt");

    let passwords = input.trim().lines().collect::<Vec<_>>();
    let part01_sol = passwords
        .iter()
        .filter(|p| is_valid_password_part01(*p))
        .count();
    println!("Part 1: {}", part01_sol);

    let part02_sol = passwords
        .iter()
        .filter(|p| is_valid_password_part02(*p))
        .count();
    println!("Part 2: {}", part02_sol);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_01() {
        assert!(is_valid_password_part01("aa bb cc dd ee"));
        assert!(!is_valid_password_part01("aa bb cc dd ee aa"));
        assert!(is_valid_password_part01("aa bb cc dd aaa"));
    }
    #[test]
    fn test_part_02() {
        assert!(is_valid_password_part02("abcde fghij"));
        assert!(!is_valid_password_part02("abcde xyz ecdab"));
        assert!(is_valid_password_part02("a ab abc abd abf abj"));
        assert!(is_valid_password_part02("iiii oiii ooii oooi oooo"));
        assert!(!is_valid_password_part02("oiii ioii iioi iiio"));
    }
}
