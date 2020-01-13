pub struct Scanning<T> {
    next_item: T,
}

impl<T> Iterator for Scanning<T> where T: Scanable + Copy {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let new_next_item = self.next_item;
        if new_next_item.is_end() {
            None
        } else {
            Scanable::update_next_item(self);
            Some(new_next_item)
        }
    }
}

pub trait Scanable where Self: Sized {
    fn is_end(&self) -> bool;

    fn update_next_item(scanning: &mut Scanning<Self>);

    fn scan(self) -> Scanning<Self>;
}

impl<T> Scanable for &[T] {

    fn is_end(&self) -> bool {
        self.is_empty()
    }

    fn update_next_item(scanning: &mut Scanning<Self>) {
        scanning.next_item.split_first().iter().for_each(|(_, next)| {
            scanning.next_item = next;
        })
    }

    fn scan(self) -> Scanning<Self> {
        Scanning {
            next_item: self,
        }
    }
}

