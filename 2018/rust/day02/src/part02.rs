mod utils;

// Reference: https://users.rust-lang.org/t/composing-a-list-of-all-pairs/15475/3
fn pairs<I: IntoIterator>(x: I) -> impl Iterator<Item = (I::Item, I::Item)>
where
    I::Item: Clone,
    I: Copy,
{
    x.into_iter()
        .enumerate()
        .flat_map(move |t| std::iter::repeat(t.1).zip(x.into_iter().skip(t.0 + 1)))
}


fn is_good_pair(a: &String, b: &String) -> bool {
    let mut bad_count = 0;
    for (c_a, c_b) in a.chars().zip(b.chars())
    {
        if c_a != c_b
        {
            bad_count += 1;
            if bad_count > 1
            {
                return false;
            }
        }
    }
    true
}
fn main() {
    let all_lines = utils::read_input();
    for (a, b) in pairs(&all_lines) {
        if !is_good_pair(a, b)
        {
            continue;
        }
        let output = a.chars().zip(b.chars())
                                        .filter(|(c_a, c_b)| c_a == c_b)
                                        .map(|(c_a, _)| { c_a })
                                        .collect::<String>();;
        println!("Answer is : {}", output);
    }
}
