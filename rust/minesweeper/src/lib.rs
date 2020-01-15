#[macro_use]
extern crate itertools;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
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

// Calculate the updates for a given (row, col).
fn create_updates(row_index: usize, col_index: usize) -> Vec<BoardUpdate> {
    let row_min = if row_index == 0 { 0 } else { row_index - 1 };
    let col_min = if col_index == 0 { 0 } else { col_index - 1 };

    iproduct!(row_min..(row_index + 2), col_min..(col_index + 2))
        .map(|(row, col)| {
            if row == row_index && col == col_index {
                BoardUpdate::BoardUpdate(BoardUpdateType::Mine, row, col)
            } else {
                BoardUpdate::BoardUpdate(BoardUpdateType::Update, row, col)
            }
        })
        .collect()
}

// Calculate all the updates for a single row.
fn calc_updates_for_row(row_index: usize, row: &str) -> Vec<BoardUpdate> {
    row.chars()
        .enumerate()
        .flat_map(|(col_index, item): (usize, char)| match item {
            '*' => create_updates(row_index, col_index),
            _ => Vec::new(),
        })
        .collect()
}

// Create a vec with one element.
fn singleton_vec<A>(a: A) -> Vec<A> {
    let mut vec: Vec<A> = Vec::new();
    vec.push(a);
    vec
}

// Turn an association list into a HashMap.
// fn hashmap_from_assoc_list<A, B>(v: &[(A, B)]) -> HashMap<A, Vec<B>>
// where
//     A: Copy + Eq + Hash,
//     B: Copy,
// {
//     let mut hashmap: HashMap<A, Vec<B>> = HashMap::new();

//     v.into_iter()
//         .for_each(|&(a, b): &(A, B)| match hashmap.entry(a) {
//             Entry::Occupied(mut entry) => {
//                 entry.get_mut().push(b);
//             }
//             Entry::Vacant(entry) => {
//                 entry.insert(singleton_vec(b));
//             }
//         });

//     hashmap
// }
fn hashmap_from_assoc_list<I, A, B>(i: I) -> HashMap<A, Vec<B>>
where
    I: Iterator<Item = (A, B)>,
    A: Copy + Eq + Hash,
    B: Copy,
{
    let mut hashmap: HashMap<A, Vec<B>> = HashMap::new();

    i.for_each(|(a, b): (A, B)| match hashmap.entry(a) {
        Entry::Occupied(mut entry) => {
            entry.get_mut().push(b);
        }
        Entry::Vacant(entry) => {
            entry.insert(singleton_vec(b));
        }
    });

    hashmap
}

// Turn a list of board updates into a hashmap indexed by locations and values
// of a list of the board updates for that location.
fn invert_board_update(
    board_updates: &[BoardUpdate],
) -> HashMap<(usize, usize), Vec<BoardUpdateType>> {
    hashmap_from_assoc_list(board_updates.iter().map(
        |&BoardUpdate::BoardUpdate(board_update_type, row, col)| ((row, col), board_update_type),
    ))
}

// Fold function for applying BoardUpdateTypes to Piece.
fn add_update(piece: Piece, board_update_type: BoardUpdateType) -> Piece {
    match (piece, board_update_type) {
        (_, BoardUpdateType::Mine) => Piece::Mine,
        (Piece::Mine, _) => Piece::Mine,
        (Piece::Empty, BoardUpdateType::Update) => Piece::Hit(1),
        (Piece::Hit(curr), BoardUpdateType::Update) => Piece::Hit(curr + 1),
    }
}

// Fold a bunch of BoardUpdateTypes to a Piece.
fn board_updates_to_piece(board_updates: &[BoardUpdateType]) -> Piece {
    board_updates
        .iter()
        .fold(Piece::Empty, |piece, &board_update_type| {
            add_update(piece, board_update_type)
        })
}

// Convert a Piece to a String.
fn piece_to_str(piece: Option<&Piece>) -> String {
    match piece {
        Some(Piece::Mine) => String::from("*"),
        Some(Piece::Hit(i)) => i.to_string(),
        _ => String::from(" "),
    }
}

// Convert a hashmap of piece locations to a Board.
fn pieces_to_board(
    max_row: usize,
    max_col: usize,
    pieces: &HashMap<(usize, usize), Piece>,
) -> Vec<String> {
    (0..max_row)
        .map(|r: usize| {
            (0..max_col)
                .map(|c: usize| piece_to_str(pieces.get(&(r, c))))
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let num_row: usize = minefield.len();
    let num_col: usize = match minefield.get(0) {
        None => return Vec::new(),
        Some(row) => row.len(),
    };

    // Calculate all the updates for each square on the board.
    let all_updates: Vec<BoardUpdate> = minefield
        .iter()
        .enumerate()
        .flat_map(|(row_index, &row): (usize, &&str)| calc_updates_for_row(row_index, row))
        .collect();

    // Invert the updates and index by the square being updated.
    let inverted_board_updates: HashMap<(usize, usize), Vec<BoardUpdateType>> =
        invert_board_update(&all_updates);

    // Calculate the final piece for each square that will get an update.
    let pieces: HashMap<(usize, usize), Piece> = inverted_board_updates
        .into_iter()
        .map(
            |(idx, board_updates): ((usize, usize), Vec<BoardUpdateType>)| {
                (idx, board_updates_to_piece(&board_updates))
            },
        )
        .collect();

    pieces_to_board(num_row, num_col, &pieces)
}
