use std::{collections::HashMap, hash::Hash};

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
/// Implementing as iterator: The steps are as follows with stepping in "i"
/// 
/// turn, step, turn, step, turn step step, turn step step, turn step step step, etc.
/// Part 2
/// 
/// Start with "1" in (0, 0)
/// Go around the spiral, and in each cell put the sum of neighboring cells.
/// 
/// Might be worth converting the spiral function into an iterator


struct Spiral
{
    delta: [i32; 2],
    pos: [i32; 2],
    i: i32,
    step_size: i32,
    sub_step: i32,

}
impl Spiral {
    pub fn new() -> Self {
        Self {
            delta: [0, -1],
            pos: [0, 0],
            i: 1,           // This is the index 
            step_size: 0,   // This is the step size of current leg. Goes up as 1, 2, 3 etc.
            sub_step: 0,    // this is counting up through each step, 0->2*step_size
        }
    }
}

impl Iterator for Spiral {
    type Item=(i32, i32);
    fn next(&mut self) -> Option<(i32, i32)>
    {
        // Special case for start (can probably be added to general case somehow)
        if self.step_size == 0
        {
            self.step_size = 1;
            self.i = 2;
            self.sub_step = 0;
            return Some((self.pos[0], self.pos[1]));
        }
        // Turn at beginning of a step, and halfway through
        if self.sub_step == 0 || self.sub_step == self.step_size {
            self.delta = [-self.delta[1], self.delta[0]]; // turn left
        }

        // Step
        self.pos[0] = self.pos[0] + self.delta[0];
        self.pos[1] = self.pos[1] + self.delta[1];
        self.i += 1;
        self.sub_step += 1;

        // Increment step size
        if self.sub_step == 2*self.step_size
        {
            self.step_size += 1;
            self.sub_step = 0;
        }
        return Some((self.pos[0], self.pos[1]));
    }
}
fn main() 
{
    let mut spiral = Spiral::new();
    let target = 277678;
    let target_pos = spiral.nth(target-1).unwrap();
    println!("Part 1: {}", target_pos.0.abs() + target_pos.1.abs());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_spiral_generator() {
        let mut spiral = Spiral::new();
        assert_eq!(spiral.next(), Some((0, 0)));
        assert_eq!(spiral.next(), Some((1, 0)));
        assert_eq!(spiral.next(), Some((1, 1)));
        assert_eq!(spiral.next(), Some((0, 1)));
        assert_eq!(spiral.next(), Some((-1, 1)));
        assert_eq!(spiral.next(), Some((-1, 0)));
        assert_eq!(spiral.next(), Some((-1, -1)));
        assert_eq!(spiral.next(), Some(( 0, -1)));
        assert_eq!(spiral.next(), Some(( 1, -1)));
        assert_eq!(spiral.next(), Some(( 2, -1)));
    }
}
