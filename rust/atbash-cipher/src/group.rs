// Create an .groups() function for splitting a str into groups of substrings.
//
// I could have just used .chunks() from itertools, but I wanted to try reimplementing it for
// practice.

pub struct Groups<'a> {
    start_index: usize,
    end_index: usize,
    group_size: usize,
    str: &'a str,
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
            // This actually isn't safe, since it may split into a unicode character boundary.  It
            // would be slightly safer to use .chars().chunked() from itertools on the original String.
            let option_next_str =
                self.str.get(self.start_index .. (self.start_index + next_step_size));
            self.start_index = self.start_index + self.group_size;
            option_next_str
        }
    }
}

pub trait Groupable {
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
