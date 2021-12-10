/// Day 10 - Syntax Scoring
/// 
/// Use a stack to keep track of open tags and match them to close tags
/// 

use std::collections::{HashMap, HashSet};

fn line_score(line: &str) -> i64
{
    let tag_scores = HashMap::from([
        (')', 1),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);

    let tag_map = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);
    let mut score = 0;
    let mut stack = Vec::new();

    for c in line.chars()
    {
        if tag_map.contains_key(&c)
        {
            stack.push(c);
        }else if tag_scores.contains_key(&c)
        {
            let open_tag = stack.pop().unwrap();
            if tag_map[&open_tag] != c
            {
                score = tag_scores[&c];
                break;
            }
        }        
    }

    score
}
fn get_line_score_part_2(line: &str) -> i64
{
    let tag_map = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);
    let tag_scores = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ]);
    let close_tags: HashSet<char> = tag_map.values().cloned().collect();
    let mut stack = Vec::new();

    for c in line.chars()
    {
        if tag_map.contains_key(&c)
        {
            stack.push(c);
        }else if close_tags.contains(&c)
        {
            let o = stack.pop().unwrap();
            if tag_map[&o] != c
            {
                break;
            }
        }
    }
    stack.iter().rev().fold(0, |score, c| score*5 + tag_scores[&c])
}
fn main()
{
    let input = include_str!("../../../inputs/day10.txt");

    let part01_sol:i64 = input.lines().map(line_score).sum();
    println!("Part 1: {}", part01_sol);

    let valid_lines : Vec<_> = input.lines().filter(|l| line_score(l)==0).collect();
    let mut scores: Vec<_> = valid_lines.into_iter().map(get_line_score_part_2).collect();
    scores.sort();
    let part02_sol = scores[scores.len()/2];
    println!("Part 2: {}", part02_sol);
}
