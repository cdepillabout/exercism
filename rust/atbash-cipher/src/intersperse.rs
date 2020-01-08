// Create an .intersperse() function for inserting an element between elements of an Iterator.
//
// I could have just used .intersperse() from itertools, but I wanted to try reimplementing it for
// practice.

pub struct Intersperse<I> where
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

pub trait Interspersable: Iterator {
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
