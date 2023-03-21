use std::{collections::HashMap};

/// Day 19 - Tractor Beam
/// 
/// Another IntCode puzzle.
/// 
/// Pass in 2D coordinates to program to determine if it is in path of tractor beam or not.
/// 
/// Part 1
/// 
/// Figure out how many points are under influence of tractor beam in 50x50 area. 
/// Assume it is single beam and that we can stop looking once it is off for a given line.
/// 
/// Based on reddit comments, I do need to reset the VM after every check
/// Seems like making it use less memory makes it significantly faster. I don't need 1MB RAM!
/// 256 bytes is enough!
/// 
/// Part 2
/// 
/// Find the 100x100 square that fits closest to the emitter.
/// 
/// Find derivative of width w and x w.r.t y
/// Use that to estimate where the beam can accomodate the 100x100 square
/// 
/// To accomodate the bottom of square, the start of the square at y0 should be offset by 
/// however much the beam is offset by in x after 100 rows
/// If y1 is the starting row and y2 is the ending row, assuming beam starts at origin (it does)
/// 
/// y2 = y1 + height of spacecraft = y1 + 100
/// 
/// Width at y1 = (width of spacecraft) + (x-offset of beam at y2 compared to that at y1)
/// 
/// dw_dy * y1 = 100 + dx_dy * (y2 - y1)     ; assumes zero width and zero offset at origin
///            = 100 + dx_dy * 100
/// 
/// => y1* (dw_dy - dx_dy) = 100 + 100 * dx_dy
///    y1 = (100 + 100 * dx_dy)/(dw_dy)
/// 
/// This may be an over-estimate
/// With estimate, start with assuming that we are at lower-left corner
/// Check upper-right corner of square if it is in the beam or not. 
/// 

use ::aoc2019::{parse_numbers_with_delimiter, intcode::IntComputer};

struct DroneComputer
{
    vm: IntComputer,
    program: Vec<i64>,
    pub beam_info: HashMap<i64, (i64, i64)>
}
impl DroneComputer {
    pub fn new(program: &String) -> Self
    {
        let program: Vec<i64> = parse_numbers_with_delimiter(program, ',').collect();
        let mut vm = IntComputer::new();
        vm.set_ram_size(200);
        vm.load_program(&program);
        Self {
            vm,
            program,
            beam_info: HashMap::new()
        }
    }
    pub fn explore_row(&mut self, row: i64) -> Option<(i64, i64)>
    {
        self.explore_row_with_estimate(row, 0, 0)
    }
    /// Explores given "row" for tractor beam
    /// Returns the "x" coordinate of the start of the beam
    /// and the width of the beam
    /// 
    /// Takes in estimats for "x" of left edge of beam and width of beam
    pub fn explore_row_with_estimate(&mut self, row: i64, estimated_slope_x: i64, estimate_slope_w: i64) -> Option<(i64, i64)>
    {
        let mut beam_already_found = false;
        let mut beam_x = None;
        let y = row;

        let mut width = 0;
        let start_x = estimated_slope_x*row;

        let jump_ahead = if estimate_slope_w != 0
        {
            estimate_slope_w * row
        }else{
            1
        };
        for x in [start_x, jump_ahead, ..row) {
            self.vm.reset();
            self.vm.load_program(&self.program);
            self.vm.push_input(x);
            self.vm.push_input(y);
            let output = self.vm.execute();
            if output[0] == 1
            {
                beam_already_found = true;
                width += 1;
                if beam_x.is_none() {
                    beam_x = Some(x);
                }
            }else{
                if beam_already_found
                {
                    break;
                }
            }
        }

        beam_x.map(|beam_x| {
            self.beam_info.insert(y, (beam_x, width));
            (beam_x, width)
        })
    }
}


fn main() {
    let input = include_str!("../../inputs/day19.txt").to_string();

    let mut drone = DroneComputer::new(&input);

    let mut counter = 0;
    
    for y in 0..50 {
        if let Some((_, width)) = drone.explore_row(y)
        {
            counter += width;
        }
    }

    let mut beam_info = drone.beam_info.clone();
    println!("{:?}", &beam_info);
    let print_width = 50;
    // for y in 0..print_width {
    //     if let Some((beam_start, beam_width)) = beam_info.get(&y){
    //         print!("{:.<1$}", "", width=*beam_start as usize);
    //         print!("{:#<1$}", "", width=*beam_width as usize);
    //         println!("{:.<1$}", "", width=(print_width-*beam_start-*beam_width) as usize);
    //     }else{
    //         println!("{:.<1$}", "", width=print_width as usize);
    //     }
    // }
    println!("\n Part 1: {}", counter);

    // Compute slope of "x" and "w" w.r.t y
    let (x1, w1) = beam_info.get(& 49).unwrap();
    let dx_dy = (*x1 as f32)/50f32;
    let dw_dy = (*w1 as f32)/50f32;
    let mut y1_est = ((100. + 100. * dx_dy)/(dw_dy as f32)) as i64;

    println!("y1_est : {}, {}, {}", y1_est, dw_dy, dx_dy);

    // let y1_est = 1337;
    // let (x_at_y1, width_at_y1) = drone.explore_row_with_estimate(y1_est, dx_dy as i64).unwrap();
    // let (x_at_y2, width_at_y2) = drone.explore_row_with_estimate(y1_est+99, dx_dy as i64).unwrap();

    // println!("{} {} {} {} {} {}", y1_est, y1_est+99, x_at_y1, x_at_y2, width_at_y1, width_at_y2);

    // println!("{}", x_at_y1+68+100-width_at_y1);
    // Beam starts at x==918. ship starts at 918+68+1 = 987
    // let print_width = 180;
    // for y in 0..100
    // {
    //     let beam = drone.explore_row_with_estimate(y+y1_est, dx_dy as i64);

    //     if let Some((beam_start, beam_width)) = beam
    //     {
    //         // println!("{} ", beam_start);
    //         print!("{:.<1$}", "", width=((beam_start-915) as usize));
    //         print!("{:#<1$}", "", width=((beam_width-100-y) as usize));
    //         assert!(beam_width-100-y > )
    //         print!("{:O<1$}", "", width=(100 as usize));
    //         println!("{:.<1$}", "", width=(print_width-beam_start-beam_width+910) as usize);
    //     }else{
    //         println!("{:.<1$}", "", width=print_width as usize);
    //     }
    // }
    // Binary search for correct row assuming we went too far down
    // let mut dy = 10;
    // loop {
    //     println!("Trying {}", y1_est);
    //     let (x_at_y1, width_at_y1) = drone.explore_row_with_estimate(y1_est, dx_dy as i64).unwrap();
    //     let (x_at_y2, width_at_y2) = drone.explore_row_with_estimate(y1_est+99, dx_dy as i64).unwrap();
    //     let dx = x_at_y2 - x_at_y1;
    //     println!("dx = {} w1 = {}", dx, width_at_y1);
    //     println!("Extra space = {}", width_at_y1 - 100 - dx);

    //     // We have extra space
    //     if width_at_y1 >= (100 + dx)
    //     {
    //         if dy == 0
    //         {
    //             let ans = y1_est + x_at_y1 * 10000;
    //             println!("{}", ans);
    //             break;
    //         }
    //         y1_est -= dy;
    //     }
    //     else {
    //         y1_est += dy;
    //         dy /= 2;
    //         y1_est -= dy;
    //     }
    // }
    
    println!("y1 = {}", y1_est)
}
