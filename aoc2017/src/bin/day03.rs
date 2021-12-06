/// Day 03 - Spiral Memory
/// 
/// Part 1
/// 
/// Given a spiral on a square lattice, find the position of index "277678" and its manhattan distance to the center
/// Assuming that center is (0, 0), the solution is |x| + |y|
/// 
/// 17  16  15  14  13
/// 18   5   4   3  12
/// 19   6   1   2  11
/// 20   7   8   9  10
/// 21  22  23---> ...
/// 
/// The steps in this process are turn-left, move "n+1", turn left, move "n+1", turn left, "move n+2" etc.
/// 
/// 1 -> (0, 0)
/// 2 -> (0, 0) + (1, 0) = (1, 0)
/// 3 -> (0, 0) + (1, 0) + (0, 1) = (1, 1)
/// 5 -> (0, 0) + (1, 0) + (0, 1) + (-2, 0) = (-1, 1)
/// 

fn get_pos_for_index(target: i32) -> (i32, i32)
{
    let mut pos = [0, 0];
    let mut delta = [0, -1]; // Start facing south (turn left to [1, 0])

    let mut i = 1;
    let mut step_size = 1;

    while i < target {
        delta = [-delta[1], delta[0]]; // turn left
        pos[0] = pos[0] + delta[0]*step_size;
        pos[1] = pos[1] + delta[1]*step_size;
        
        i += step_size;
        if i >= target
        {
            break;
        }
        delta = [-delta[1], delta[0]]; // turn left
        pos[0] = pos[0] + delta[0]*step_size;
        pos[1] = pos[1] + delta[1]*step_size;
        i += step_size;
        step_size += 1;
    }
    if i > target {
        let extra = i - target;
        if extra > 0
        {
            pos[0] = pos[0] - delta[0]*extra;
            pos[1] = pos[1] - delta[1]*extra;
            i -= extra;
        }
    }
    (pos[0], pos[1])
}
fn main() 
{
    let target = 277678;
    
    let target_pos = get_pos_for_index(target);
    println!("Part 1: {}", target_pos.0.abs() + target_pos.1.abs());
}
