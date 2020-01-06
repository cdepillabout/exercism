use std::iter::Rev;
use std::str::Chars;
use unicode_segmentation::Graphemes;
use unicode_segmentation::UnicodeSegmentation;

// This function doesn't work for unicode strings with special characters.
pub fn reverse_old(input: &str) -> String {
    let s: String = String::from(input);
    let chars: Chars = s.chars();
    let rev_chars: Rev<Chars> = chars.rev();
    let new_str: String = rev_chars.collect();

    new_str
}

pub fn reverse(input: &str) -> String {
    let graphemes_iterator: Graphemes = input.graphemes(true);
    let reverse_graphemes_iterator: Rev<Graphemes> = graphemes_iterator.rev();
    let reversed_string: String = reverse_graphemes_iterator.collect();

    reversed_string
}
