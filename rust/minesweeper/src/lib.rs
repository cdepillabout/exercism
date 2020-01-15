#[macro_use]
extern crate itertools;

fn create_updates(row_index: usize, col_index: usize) -> Vec<(usize, usize)> {
    let row_min = if row_index == 0 { 0 } else { row_index - 1 };
    let col_min = if col_index == 0 { 0 } else { col_index - 1 };

    iproduct!(row_min..(row_index + 2), col_min..(col_index + 2))
        .filter(|(row, col)| *row != row_index || *col != col_index)
        .collect()
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let num_row = minefield.len();

    let all_updates: Vec<(usize, usize)> =
        minefield
            .iter()
            .enumerate()
            .flat_map(|(row_index, row): (usize, &str)| {
                row.chars()
                    .enumerate()
                    .flat_map(|(col_index, item): (usize, char)| match item {
                        '*' => create_updates(row_index, col_index),
                        _ => Vec::new(),
                    })
            });

    // let x: usize = 3;
    // let y = x - 10;

    // print!("{:?}", create_updates(0,0));

    todo!();
}
