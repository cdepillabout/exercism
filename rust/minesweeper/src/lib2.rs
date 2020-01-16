use std::collections::HashSet;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mines: HashSet<(i32, i32)> =
        minefield
            .iter()
            .enumerate()
            .flat_map(|(row, &str): (usize, &&str)|
                str
                    .chars()
                    .enumerate()
                    .filter_map(move |(column, char): (usize, char)|
                        if char == '*' {
                            Some((row as i32, column as i32))
                        } else {
                            None
                        }
                    )
            )
            .collect();



    todo!();
}
