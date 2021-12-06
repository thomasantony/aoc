/// Day 04 - Giant Squid
/// 
/// Play bingo with a squid
/// 
/// Part 1
/// 
/// Find the board that wins first. Board keeps track of marked numbers in each row/col in a Vec<HashSet>
/// The board has won if any hashset reaches 5 elements. Unmarked numbers are found by filtering out the 
/// numbers present in the hashsets
/// 
/// Part 2
/// Find the board that wins last. Use same procedures as part 1 but keep running until all numbers drawn.
/// Ignore any board that wins in between.
/// 
use std::collections::{HashMap, HashSet, VecDeque};

use itermore::IterMore;

// const DEMO_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

// 22 13 17 11  0
//  8  2 23  4 24
// 21  9 14 16  7
//  6 10  3 18  5
//  1 12 20 15 19

//  3 15  0  2 22
//  9 18 13 17  5
// 19  8  7 25 23
// 20 11 10 24  4
// 14 21 16 12  6

// 14 21 17 24  4
// 10 16 15  9 19
// 18  8 23 26 20
// 22 11 13  6  5
//  2  0 12  3  7
// ";

#[derive(Debug, Clone)]
struct Board {
    numbers: HashMap<u32, (usize, usize)>,
    rows_hit: Vec<HashSet<u32>>,
    cols_hit: Vec<HashSet<u32>>,
}
impl Board {
    pub fn from_str_iter(board_def: [&str; 5]) -> Self {
        let number_vec: Vec<_> = board_def
            .iter()
            .map(|row| {
                row.split_ascii_whitespace()
                    .map(str::parse::<u32>)
                    .map(Result::unwrap)
                    .collect::<Vec<_>>()
            })
            .collect();

        let mut numbers = HashMap::new();
        for (row_idx, row) in number_vec.into_iter().enumerate() {
            for (col_idx, num) in row.into_iter().enumerate() {
                numbers.insert(num, (row_idx, col_idx));
            }
        }
        Self {
            numbers,
            rows_hit: (0..5).map(|_| HashSet::new()).collect(),
            cols_hit: (0..5).map(|_| HashSet::new()).collect(),
        }
    }
    pub fn play(&mut self, number: u32) {
        if self.numbers.contains_key(&number) {
            let (row, col) = self.numbers.get(&number).unwrap();

            // Update rows_hit and cols_hit
            self.rows_hit.get_mut(*row).unwrap().insert(number);
            self.cols_hit.get_mut(*col).unwrap().insert(number);
        }
    }
    pub fn has_won(&self) -> bool {
        let has_winning_rows = self
            .rows_hit
            .iter()
            .any(|hit_numbers| hit_numbers.len() == 5);
        let has_winning_cols = self
            .cols_hit
            .iter()
            .any(|hit_numbers| hit_numbers.len() == 5);
        has_winning_rows || has_winning_cols
    }
    pub fn _get_winning_numbers(&self) -> Option<Vec<u32>> {
        let winning_rows = self
            .rows_hit
            .iter()
            .filter(|hit_numbers| hit_numbers.len() == 5)
            .collect::<Vec<_>>();
        let winning_cols = self
            .cols_hit
            .iter()
            .filter(|hit_numbers| hit_numbers.len() == 5)
            .collect::<Vec<_>>();

        if !winning_rows.is_empty() {
            Some(
                winning_rows
                    .into_iter()
                    .flatten()
                    .cloned()
                    .collect::<Vec<_>>(),
            )
        } else if !winning_cols.is_empty() {
            Some(
                winning_cols
                    .into_iter()
                    .flatten()
                    .cloned()
                    .collect::<Vec<_>>(),
            )
        } else {
            None
        }
    }
    pub fn get_unmarked_numbers(&self) -> Vec<u32> {
        let marked_numbers = self
            .rows_hit
            .iter()
            .flatten()
            .chain(self.cols_hit.iter().flatten())
            .collect::<Vec<_>>();
        self.numbers
            .keys()
            .filter(|n| !marked_numbers.contains(n))
            .cloned()
            .collect()
    }
}
fn solve_part_1(mut boards: Vec<Board>, numbers_drawn: Vec<u32>) {
    let mut has_won: bool = false;
    for number in numbers_drawn {
        for b in boards.iter_mut() {
            b.play(number);
            if b.has_won() {
                has_won = true;

                let unmarked_numbers = b.get_unmarked_numbers();
                let solution: u32 = unmarked_numbers.into_iter().sum::<u32>() * number;
                println!("Solution for part 1: {}", solution);
            }
        }
        if has_won {
            break;
        }
    }
}
fn solve_part_2(mut boards: Vec<Board>, numbers_drawn: Vec<u32>) {
    let mut winning_board_index: Option<usize> = None;
    let mut final_score: u32 = 0;
    for number in numbers_drawn {
        for (idx, b) in boards.iter_mut().enumerate() {
            // Ignore boards that have already won
            if !b.has_won() {
                b.play(number);
                if b.has_won() {
                    winning_board_index = Some(idx);
                }
            }
        }
        if let Some(winning_index) = winning_board_index {
            let board = boards.get(winning_index).unwrap();
            let unmarked_numbers = board.get_unmarked_numbers();
            final_score = unmarked_numbers.into_iter().sum::<u32>() * number;
            winning_board_index = None;
        }
    }
    println!("Solution for part 2: {}", final_score);
}
fn main() {
    let input = include_str!("../../../inputs/day04.txt");
    // let input = DEMO_INPUT.to_owned();

    let mut input_lines: VecDeque<_> = input.lines().filter(|&s| !s.is_empty()).collect();
    let numbers_drawn = input_lines
        .pop_front()
        .expect("Badly formatted input")
        .split(',')
        .map(|num_s| num_s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut boards: Vec<Board> = Vec::new();

    for rows in input_lines.into_iter().chunks::<5>() {
        boards.push(Board::from_str_iter(rows));
    }

    solve_part_1(boards.clone(), numbers_drawn.clone());
    solve_part_2(boards, numbers_drawn);
}
