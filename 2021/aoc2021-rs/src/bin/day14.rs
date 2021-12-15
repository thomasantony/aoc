use std::{collections::HashMap};
#[macro_use]
extern crate cached;


type RuleBook = HashMap<String, String>;
use cached::cached_key;
use cached::SizedCache;

cached_key!{
    LENGTH: SizedCache<String, String> = SizedCache::with_size(50);
    Key = { template.into() };
    fn polymerize(template: &str, rules: &RuleBook) -> String = {
        if template.len() < 2
        {
            return template.into()
        }
        let pair:&str = &template[..2];
        let retval = if rules.contains_key(pair)
        {
            rules[pair].to_string()
        }else{
            pair.to_string()
        };
        // println!("{} -> {}", &pair, &retval);
        retval + &polymerize(template[1..].into(), rules)[1..]
    }
}
fn encode(poly_template: &str) -> Vec<(char, i32)>
{
    let mut output = Vec::new();

    let mut last_char = poly_template.chars().next().unwrap();
    let mut counter = 0;
    for char in poly_template.chars()
    {
        if char == last_char
        {
            counter += 1;
        }else{
            output.push((last_char, counter));
            
            // Reset counter
            counter = 1;
            last_char = char;
        }
    }
    if counter > 0
    {
        output.push((last_char, counter));
    }
    
    output
}

fn main()
{
    let input = include_str!("../../../inputs/day14.txt");
    let mut lines_iter = input.lines();

    let poly_template = lines_iter.next().unwrap();
    lines_iter.next();

    let mut rules = HashMap::new();
    
    for rule in lines_iter
    {
        let rule_data: Vec<_> = rule.split(" -> ").collect();
        let pair = rule_data[0].to_string();
        let insert = rule_data[1].to_string();
        // let a = pair.chars().nth(0).unwrap();
        // let b  = pair.chars().nth(1).unwrap();
        let replacement = format!("{}{}{}", pair.chars().nth(0).unwrap(), insert, pair.chars().nth(1).unwrap());
        rules.insert(pair, replacement);
    }
    // let repeater_rules: HashMap<_, String> = rules.iter()
    //                                         .filter(|(&k , _)| k.0 == k.1)
    //                                         .map(|(&k, v)| (k.0, v.into()))
    //                                         .collect();
    // let nonrepeater_rules: HashMap<_, _> = rules.iter().filter(|(&k , _)| k.0 != k.1).collect();

    // println!("{:?}", repeater_rules);
    // println!("{:?}", nonrepeater_rules);
    // let poly = encode(poly_template);
    // println!("{:?}", &poly);
    let mut poly2: String = poly_template.into();
    for i in 0..40
    {
        poly2 = polymerize(&poly2, &rules);
        println!("{}, {}", i, poly2);
    }
    
}
