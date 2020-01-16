#[macro_use]
extern crate itertools;

use std::collections::HashSet;

fn count_mines(mines: &HashSet<(i32, i32)>, row: i32, col: i32) -> usize {
    iproduct!((row - 1)..=(row + 1), (col - 1)..=(col + 1))
        .filter(|idx: &(i32, i32)| mines.contains(idx))
        .count()
}

fn calc_piece(mines: &HashSet<(i32, i32)>, row: i32, col: i32) -> String {
    if mines.contains(&(row, col)) {
        "*".to_string()
    } else {
        match count_mines(mines, row, col) {
            0 => " ".to_string(),
            n => n.to_string(),
        }
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut max_row_index: i32 = -1;
    let mut max_col_index: i32 = -1;

    let mines: HashSet<(i32, i32)> =
        minefield
            .iter()
            .enumerate()
            .flat_map(|(row, &str): (usize, &&str)| {
                max_row_index = row as i32;
                str
                    .chars()
                    .enumerate()
                    .filter_map(|(col, char): (usize, char)| {
                        max_col_index = col as i32;
                        if char == '*' {
                            Some((row as i32, col as i32))
                        } else {
                            None
                        }
                    })
                    .collect::<HashSet<(i32, i32)>>()
            })
            .collect();

    (0..=max_row_index)
        .map(|row: i32| {
            (0..=max_col_index)
                .map(|col: i32| calc_piece(&mines, row, col))
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}
