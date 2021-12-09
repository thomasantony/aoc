/// Day 09 - Smoke Basin
/// 
/// Find "basins" in the given map surrounded by higher points
/// 
/// Part 1
/// Create Hashmap of coordinates and heights
/// Bruteforce it
/// 
/// Part 2
/// Take list of low points from part 1
/// Create stack with just the low point.
/// Add surrounding points to stack if they are valid (i.e not seen before and val != 9)
/// Count the number of points until stack runs out
/// 
/// 
use std::collections::{HashMap, HashSet};

fn main()
{
    let input = include_str!("../../../inputs/day09.txt");

    let input :Vec<_> = input.trim().lines().map(|s| s.chars().map(|c| c.to_string()).map(|c| c.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect();

    let y_s = input.len() as i32;
    let x_s = input[0].len() as i32;

    let mut map = HashMap::new();
    for (y, row) in input.iter().enumerate()
    {
        for (x, col) in row.iter().enumerate()
        {
            map.insert((x, y), *col);
        }
    }
    // println!("{:?}", &map);
    let mut min_vals = Vec::new();
    let mut min_pos = Vec::new();
    for y in 0..y_s{
        for x in 0..x_s
        {
            let neih = [(1i32, 0i32), (-1i32, 0i32), (0i32, 1i32), (0i32, -1i32)];

            let val = map.get(&(x as usize, y as usize)).unwrap();
            let mut is_min = true;

            let mut neigh_vals = Vec::new();
            for (n_x, n_y) in neih {
                let p_x = x as i32 + n_x;
                let p_y = y as i32 + n_y;
                let pos = (p_x as usize, p_y as usize);

                if ! map.contains_key(&pos)
                {
                    continue;
                }
                neigh_vals.push(map[&pos]);
                if map[&pos] <= *val{
                    is_min = false;
                    break;
                }
            }
            if is_min{
                min_vals.push(*val);
                min_pos.push((x, y));
            }
        }
    }
    println!("Part 1: {}", min_vals.iter().sum::<i32>());

    let mut basins = HashMap::new();
    for (x, y) in min_pos.iter()
    {
        // Radially check out until hitting 9

        let mut to_check = Vec::new();
        to_check.push((*x, *y));
        
        let mut already_seen = HashSet::new();
        let mut basin_counter = 1;  // start at one with deep point
        while !to_check.is_empty()
        {
            let next_pos = to_check.pop().unwrap();
            already_seen.insert(next_pos.clone());

            let neih = [(1i32, 0i32), (-1i32, 0i32), (0i32, 1i32), (0i32, -1i32)];
            for (n_x, n_y) in neih {
                let p_x = next_pos.0 + n_x;
                let p_y = next_pos.1 + n_y;
                let n_pos = (p_x as usize, p_y as usize);
                if map.contains_key(&n_pos) && map[&n_pos] != 9 && !already_seen.contains(&(p_x, p_y))
                {
                    to_check.push((p_x, p_y));
                    basin_counter += 1;
                    already_seen.insert((p_x, p_y));
                }
            }
        }
        basins.insert((*x, *y), basin_counter);
    }
    
    
    let mut sizes: Vec<_> = basins.iter().map(|(_, v)| v).collect();
    sizes.sort();
    let n = sizes.len();
    println!("Part 2: {}", sizes[n-1]*sizes[n-2]*sizes[n-3]);
}
