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

// struct SplitInternal<'a, P: Pattern<'a>> {
//     start: usize,
//     end: usize,
//     matcher: P::Searcher,
//     allow_trailing_empty: bool,
//     finished: bool,
// }

// impl<'a, P: Pattern<'a>> SplitInternal<'a, P> {
//     #[inline]
//     fn next(&mut self) -> Option<&'a str> {
//         if self.finished { return None }

//         let haystack = self.matcher.haystack();
//         match self.matcher.next_match() {
//             Some((a, b)) => unsafe {
//                 let elt = haystack.get_unchecked(self.start..a);
//                 self.start = b;
//                 Some(elt)
//             },
//             None => self.get_end(),
//         }
//     }

//generate_pattern_iterators! {
//    forward:
//        /// Created with the method [`split`].
//        ///
//        /// [`split`]: ../../std/primitive.str.html#method.split
//        struct Split;
//    reverse:
//        /// Created with the method [`rsplit`].
//        ///
//        /// [`rsplit`]: ../../std/primitive.str.html#method.rsplit
//        struct RSplit;
//    stability:
//        #[stable(feature = "rust1", since = "1.0.0")]
//    internal:
//        SplitInternal yielding (&'a str);
//    delegate double ended;
//}

// pub struct $forward_iterator<'a, P: Pattern<'a>>($internal_iterator<'a, P>);

// $(#[$common_stability_attribute])*
// impl<'a, P: Pattern<'a>> Iterator for $forward_iterator<'a, P> {
//     type Item = $iterty;

//     #[inline]
//     fn next(&mut self) -> Option<$iterty> {
//         self.0.next()
//     }
// }

// pub fn split<'a, P: Pattern<'a>>(&'a self, pat: P) -> Split<'a, P> {
//         Split(SplitInternal {
//             start: 0,
//             end: self.len(),
//             matcher: pat.into_searcher(self),
//             allow_trailing_empty: true,
//             finished: false,
//         })
//     }

struct Groups<'a> {
    start_index: usize,
    end_index: usize,
    group_size: usize,
    str: &'a str,
}

fn groups<'a>(str: &'a str, group_size: usize) -> Groups<'a> {
    Groups {
        start_index: 0,
        end_index: str.len(),
        group_size,
        str,
    }
}

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

enum IntersperseState<Item> {
    PrevIteratorItem,
    PrevSeparater(Item),
}

struct Intersperse<I> where
    I: Iterator
{
    intersperse_state: IntersperseState<<I as Iterator>::Item>,
    iterator: I,
    separator: <I as Iterator>::Item,
}

fn intersperse<I>(mut iterator: I, separator: <I as Iterator>::Item) -> Intersperse<I> where
    I: Iterator{
    // create the initial state
    let intersperse_state =
        iterator
            .next()
            .map_or(IntersperseState::PrevIteratorItem, IntersperseState::PrevSeparater);
    Intersperse {
        intersperse_state,
        iterator,
        separator,
    }
}

impl<I> Iterator for Intersperse<I> where
    I: Iterator,
    <I as Iterator>::Item: Copy
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        match self.intersperse_state {
            // The previous thing we returned was an iterator item, so
            // we need to return a separator now, but only if we have
            // at least one more iterator item in the queue.
            IntersperseState::PrevIteratorItem => {
                // let option_next_val = self.iterator.next();
                // match option_next_val {
                //     // There are no more iterator items, so we don't need
                //     // to return anything.  We are done.
                //     None => None,
                //     // There is at least one more iterator item, so
                //     // we queue it up and return a separator.
                //     Some(next_val) => {
                //         self.intersperse_state =
                //             IntersperseState::PrevSeparater(next_val);
                //         Some(self.separator)
                //     }
                // }
                self.iterator.next().map(|next_val| {
                  self.intersperse_state = IntersperseState::PrevSeparater(next_val);
                  self.separator
                }
                )
            }
            // The previous thing we returned was a separator, so
            // we need to return the queued up iterator item now.
            IntersperseState::PrevSeparater(next_val) => {
                self.intersperse_state = IntersperseState::PrevIteratorItem;
                Some(next_val)
            }
            
        }
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let encoded: String = encode_cipher_chars(normalize(plain)).collect();
    let encoded_str: &str = &encoded;
    let str_groups: Groups = groups(encoded_str, 5);
    // TODO: I can probably get rid of my groupby code, since itertools implements a chunks
    // operation.
    // str_groups.intersperse(" ").collect()
    intersperse(str_groups, " ").collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    unimplemented!("Decoding of {:?} in Atbash cipher.", cipher);
}
