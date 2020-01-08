// use itertools::Itertools;

fn normalize(str: &str) -> impl Iterator<Item = char> + '_ where
{
    str.chars().filter(|c| c.is_alphanumeric()).flat_map(char::to_lowercase)
}

fn map_char(input: char) -> char {
    if input.is_ascii_lowercase() {
        (('z' as u8) - (input as u8) + ('a' as u8)) as char
    } else {
        input
    }
}

fn encode_cipher_chars<I>(input: I) -> impl Iterator<Item = char> where
    I: Iterator<Item = char>,
{
    input.map(map_char)
}

struct Groups<'a> {
    start_index: usize,
    end_index: usize,
    group_size: usize,
    str: &'a str,
}

// fn groups<'a>(str: &'a str, group_size: usize) -> Groups<'a> {
//     Groups {
//         start_index: 0,
//         end_index: str.len(),
//         group_size,
//         str,
//     }
// }

impl<'a> Iterator for Groups<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        // If the starting character is greater than (or at) the end character,
        // there are no more characters available.
        if self.start_index >= self.end_index {
            None
        } else {
            // This is the next step size to return.  Normally it will be
            // the group_size, but if we don't have enough characters to 
            // form a full group, it will be the remaining characters.
            let next_step_size =
                std::cmp::min(self.group_size, self.end_index - self.start_index);
            let option_next_str =
                self.str.get(self.start_index .. (self.start_index + next_step_size));
            self.start_index = self.start_index + self.group_size;
            option_next_str
        }
    }
}

trait Groupable {
    fn groups<'a>(&'a self, group_size: usize) -> Groups<'a>;
}

impl Groupable for str {
    fn groups<'a>(&'a self, group_size: usize) -> Groups<'a> {
        Groups {
            start_index: 0,
            end_index: self.len(),
            group_size,
            str: self,
        }
    }
}

// impl Groupable for T {
//     fn groups<'a>(&'a self, group_size: usize) -> Groups<'a> {
//         Groups {
//             start_index: 0,
//             end_index: self.len(),
//             group_size,
//             str: self,
//         }
//     }
// }

struct Intersperse<I> where
    I: Iterator
{
    queued_item: Option<<I as Iterator>::Item>,
    iterator: I,
    separator: <I as Iterator>::Item,
}

impl<I> Iterator for Intersperse<I> where
    I: Iterator,
    <I as Iterator>::Item: Copy
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        match self.queued_item {
            // The previous thing we returned was an iterator item, so
            // we need to return a separator now, but only if we have
            // at least one more iterator item.
            None => {
                self.iterator.next().map(|next_val| {
                  // Add the next iterator item to the queue.
                  self.queued_item = Some(next_val);
                  self.separator
                })
            }
            // The previous thing we returned was a separator, so
            // we need to return the queued up iterator item now.
            Some(next_val) => {
                self.queued_item = None;
                Some(next_val)
            }
            
        }
    }
}

trait Interspersable: Iterator {
    fn intersperse(mut self, separator: <Self as Iterator>::Item) -> Intersperse<Self> where
        Self: Sized,
    {
	Intersperse {
	    queued_item: self.next(),
	    iterator: self,
	    separator,
	}
    }
}

// Interspersable works for any T where T is an Iterator.
impl<T> Interspersable for T where T: Iterator { }

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    // let encoded: String = encode_cipher_chars(normalize(plain)).collect();
    // // let encoded_str: &str = &encoded;
    // // let str_groups: Groups = groups(encoded_str, 5);
    // // let str_groups: Groups = encoded_str.groups(5);
    // let str_groups: Groups = encoded.groups(5);
    // // TODO: I can probably get rid of my groupby code, since itertools implements a chunks
    // // operation.
    // // str_groups.intersperse(" ").collect()
    // // intersperse(str_groups, " ").collect()
    // str_groups.intersperse(" ").collect()

    let encoded: String = encode_cipher_chars(normalize(plain)).collect();
    encoded.groups(5).intersperse(" ").collect()

    // encode_cipher_chars(normalize(plain)).groups(5).intersperse(" ").collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    unimplemented!("Decoding of {:?} in Atbash cipher.", cipher);
}
