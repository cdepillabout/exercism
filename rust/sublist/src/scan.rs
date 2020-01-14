pub struct Scanning<'a, T: 'a> {
    next_item: &'a [T],
}

impl<'a, T> Iterator for Scanning<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<&'a [T]> {
        if self.next_item.len() > 0 {
            let ret: Option<&[T]> = Some(self.next_item);
            self.next_item = &self.next_item[1..];
            ret
        } else {
            None
        }
    }
}

pub trait Scanable<'a> {
    type Output;
    fn scan(self) -> Scanning<'a, Self::Output>;
}

impl<'a, T> Scanable<'a> for &'a [T] {
    type Output = T;

    fn scan(self) -> Scanning<'a, T> {
        Scanning { next_item: self }
    }
}
