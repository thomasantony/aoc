/// Day 14 - Extended Polymerization
/// 
/// Figure out the number of times a certain letter repeats after applying repeated find/replace ops
/// /// 
/// Part 1 & Part 2
/// 
/// First one works with bruteforce method. Second one requires a better way.
/// Found hint on reddit about using "Counter" in Python and determining how the rules themselves evolve with each "step"
/// before applying it to the input
/// 
use std::{collections::HashMap};
use counter::Counter;
use std::iter;
type PairCounter = Counter<(char, char), u64>;
type CharCounter = Counter<char, u64>;

fn pairwise<I>(right: I) -> impl Iterator<Item = (Option<I::Item>, I::Item)>
where
    I: IntoIterator + Clone,
{
    let left = iter::once(None).chain(right.clone().into_iter().map(Some));
    left.zip(right)
}

fn main()
{
    let input = include_str!("../../../inputs/day14.txt");
    let mut lines_iter = input.lines();

    let poly_template = lines_iter.next().unwrap();
    lines_iter.next();

    let mut rules = HashMap::new();
    
    let mut count_by_rule: HashMap<(char, char), PairCounter> = HashMap::new();
    for rule in lines_iter
    {
        let rule_data: Vec<_> = rule.split(" -> ").collect();
        let pair = rule_data[0].to_string();
        let insert = rule_data[1].chars().next().unwrap();
        let a = pair.chars().nth(0).unwrap();
        let b  = pair.chars().nth(1).unwrap();
        // let replacement = format!("{}{}{}", pair.chars().nth(0).unwrap(), insert, pair.chars().nth(1).unwrap());
        // rules.insert(pair, replacement);
        rules.insert((a, b), insert);

        // Initialize counter hashmap with single count of both chars
        count_by_rule.insert((a, b), PairCounter::init([
            (a, b)
        ]));
    }

    // For each step
    for _ in 0..1 {
        // Iterate over counts for each rule
        for (a, counter) in count_by_rule.iter_mut()
        {
            print!("{:?} -> ", a);
            // See if any of the pairs in the counter have rules associated with them
            let keys_in_counter = counter.keys();

            let new_pairs = keys_in_counter.map(|(a, b)| {
                // Returns Option<[(char, char)]>
                rules.get(&(*a, *b)).and_then(|&insert|{
                        Some([
                            (*a, insert),
                            (insert, *b)
                        ])
                })
            }).flatten().flatten().collect::<Vec<_>>();
            println!("Updating with {:?}", new_pairs);
            counter.update(new_pairs);
            counter.entry(*a).and_modify(|e| *e -= 1).or_default();
        }
    }
    
    let mut part01_counter = CharCounter::new();
    let poly_template = "NN".to_string();
    for (a, b) in (poly_template).chars().zip(poly_template.chars().skip(1))
    {
        let mut char_counter =  CharCounter::new();
        print!("{}{} ->", a, b);
        // We have already counted the results for this pair
        if let Some(counts_for_pair) = count_by_rule.get(&(a, b))
        {
            println!("{:?}", counts_for_pair);
            // For each pair that is the result of polymerizing hte starting pair
            // increment the count for each char
            for (&(c,d), &count) in counts_for_pair.iter()
            {
                println!("Updating count for {:?}", (c, d));
                char_counter.entry(c).and_modify(|e| {
                    println!("Adding {} for {}", count, c);
                    *e += count;
                }).or_insert(count);
                char_counter.entry(d).and_modify(|e| {
                    println!("Adding {} for {}", count, d);
                    *e += count;
                }).or_insert(count);
            }
        }else{
            // If no pairs result from this pair, increment count for the chars
            char_counter.entry(a).and_modify(|e| {
                    *e += 1;
                }).or_insert(1);
            char_counter.entry(b).and_modify(|e| {
                    *e += 1;
                }).or_insert(1);
        }
        println!("{:?}", char_counter);
        part01_counter.extend(char_counter.into_map());
    }
    println!("{} {} {}", part01_counter[&'B'], part01_counter[&'C'], part01_counter[&'H']);
}
