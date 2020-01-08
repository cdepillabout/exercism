mod group;
mod intersperse;

use group::Groupable;
use intersperse::Interspersable;

// Removes non-alphanumeric characters from a string and converts all characters to lowercase.
fn normalize(str: &str) -> impl Iterator<Item = char> + '_ where
{
    str.chars().filter(|c| c.is_alphanumeric()).flat_map(char::to_lowercase)
}

// Encode or decode a character in the atbash cipher.
fn encode_char(input: char) -> char {
    if input.is_ascii_lowercase() {
        (('z' as u8) - (input as u8) + ('a' as u8)) as char
    } else {
        input
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let encoded: String = normalize(plain).map(encode_char).collect();
    encoded.groups(5).intersperse(" ").collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().filter(|c| !c.is_whitespace()).map(encode_char).collect()
}
