use multiset::HashMultiSet;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let lowercase_word_multi_set = HashMultiSet::from_iter(lowercase_word.chars());

    possible_anagrams
        .iter()
        .cloned()
        .filter(
            |&str| {
                let lower_str: String = str.to_lowercase();
                let lower_str_multi_set = HashMultiSet::from_iter(lower_str.chars());
                lower_str_multi_set == lowercase_word_multi_set && lower_str != lowercase_word
            })
        .collect()
}
