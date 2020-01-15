#[macro_use]
extern crate itertools;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;

#[derive(Debug, Copy, Clone)]
enum BoardUpdateType {
    Mine,
    Update,
}

#[derive(Debug, Copy, Clone)]
enum BoardUpdate {
    BoardUpdate(BoardUpdateType, usize, usize),
}

#[derive(Debug, Copy, Clone)]
enum Piece {
    Mine,
    Hit(u32),
    Empty,
}


fn create_updates(row_index: usize, col_index: usize) -> Vec<BoardUpdate> {
    let row_min = if row_index == 0 { 0 } else { row_index - 1 };
    let col_min = if col_index == 0 { 0 } else { col_index - 1 };

    iproduct!(row_min..(row_index + 2), col_min..(col_index + 2))
        .map(|(row, col)|
            if row == row_index && col == col_index {
                BoardUpdate::BoardUpdate(BoardUpdateType::Mine, row, col)
            } else {
                BoardUpdate::BoardUpdate(BoardUpdateType::Update, row, col)
            })
        .collect()
}

fn map_row(row_index: usize, row: &str) -> Vec<BoardUpdate> {
    row.chars()
        .enumerate()
        .flat_map(|(col_index, item): (usize, char)| match item {
            '*' => create_updates(row_index, col_index),
            _ => Vec::new(),
        })
        .collect()
}

fn singleton_vec<A>(a: A) -> Vec<A> {
    let mut vec: Vec<A> = Vec::new();
    vec.push(a);
    vec
}

fn hashmap_from_assoc_list<A, B> (v: Vec<(A, B)>) -> HashMap<A, Vec<B>> where
    A: Eq + Hash
{
    let mut hashmap: HashMap<A, Vec<B>> = HashMap::new();

    for (a, b) in v {
        match hashmap.entry(a) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().push(b);
            }
            Entry::Vacant(entry) => {
                entry.insert(singleton_vec(b));
            }
        }
    }

    hashmap
}

fn invert_board_update(board_updates: &[BoardUpdate]) -> HashMap<(usize, usize), Vec<BoardUpdateType>> {
    hashmap_from_assoc_list(
        board_updates
            .iter()
            .map(|BoardUpdate::BoardUpdate(board_update_type, row, col)|
                ((*row, *col), *board_update_type)
                )
            .collect()
    )
}

fn add_update(piece: Piece, board_update_type: &BoardUpdateType) -> Piece {
    match (piece, board_update_type) {
        (_, BoardUpdateType::Mine) => Piece::Mine,
        (Piece::Mine, _) => Piece::Mine,
        (Piece::Empty, BoardUpdateType::Update) => Piece::Hit(1),
        (Piece::Hit(curr), BoardUpdateType::Update) => Piece::Hit(curr + 1),
    }
}

fn board_updates_to_piece(board_updates: &[BoardUpdateType]) -> Piece {
    board_updates
        .iter()
        .fold(Piece::Empty, add_update)
}

fn piece_to_str(piece: Option<&Piece>) -> String {
    match piece {
        Some(Piece::Mine) => String::from("*"),
        Some(Piece::Hit(i)) => i.to_string(),
        _ => String::from(" "),
    }
}

fn pieces_to_board(max_row: usize, max_col: usize, pieces: &HashMap<(usize, usize), Piece>) -> Vec<String> {
    (0..max_row)
        .map(|r|
            (0..max_col)
                .map(|c| piece_to_str(pieces.get(&(r, c))))
                .collect::<String>()
        )
        .collect::<Vec<String>>()
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let num_row: usize = minefield.len();
    let num_col: usize =
        match minefield.get(0) {
            None => return Vec::new(),
            Some(row) => row.len(),
        };

    let all_updates: Vec<BoardUpdate> =
        minefield
            .iter()
            .enumerate()
            .flat_map(|(row_index, row): (usize, &&str)| {
                map_row(row_index, *row)
            })
            .collect();

    let mut inverted_board_updates: HashMap<(usize, usize), Vec<BoardUpdateType>> =
        invert_board_update(&all_updates);

    let pieces: HashMap<(usize, usize), Piece> =
        inverted_board_updates
            .iter_mut()
            .map(|(idx, board_updates): (&(usize, usize), &mut Vec<BoardUpdateType>)|
                (*idx, board_updates_to_piece(board_updates)))
            .collect();

    pieces_to_board(num_row, num_col, &pieces)
}
