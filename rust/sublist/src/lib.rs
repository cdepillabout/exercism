mod scan;

use scan::Scanable;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let is_first_sublist_of_second: bool = 
        first_list.scan().any(|l| l.starts_with(second_list));
    let is_second_sublist_of_first: bool = 
        second_list.scan().any(|l| l.starts_with(first_list));

    match (first_list == second_list, is_first_sublist_of_second, is_second_sublist_of_first) {
        (true, _, _) => Comparison::Equal,
        (_, true, false) => Comparison::Superlist,
        (_, false, true) => Comparison::Sublist,
        _ => Comparison::Unequal,
    }
}
