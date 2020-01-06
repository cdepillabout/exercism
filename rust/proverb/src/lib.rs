
// fn fold<B, F>(mut self, init: B, f: F) -> B where
//     Self: Sized, F: FnMut(B, Self::Item) -> B,
// {
//     fn ok<B, T>(mut f: impl FnMut(B, T) -> B) -> impl FnMut(B, T) -> Result<B, !> {
//         move |acc, x| Ok(f(acc, x))
//     }

//     self.try_fold(init, ok(f)).unwrap()
// }


fn scan_two_helper<I, F, G, B>(mut iterator: I, this: <I as Iterator>::Item, mut f: F, mut g: G, mut vec: Vec<B>, first: <I as Iterator>::Item) -> Vec<B> where
    I: Iterator,
    F: FnMut(<I as Iterator>::Item, <I as Iterator>::Item) -> B,
    G: FnMut(<I as Iterator>::Item) -> B,
    <I as Iterator>::Item: Copy + std::fmt::Debug,
    B: Clone + std::fmt::Debug,
{
    let option_next = iterator.next();

    println!("scan_two_helper, this: {:?}, option_next: {:?}, vec: {:?}", this, option_next, vec);

    match option_next {
        Some(next) => {
            let fafafa = f(this, next);
            vec.push(fafafa.clone());
            println!("\tscan_two_helper, next thing: {:?}, new vec: {:?}", fafafa.clone(), vec);
            return scan_two_helper(iterator, next, f, g, vec, first);
        }
        None => {
            let fafafa = g(first);
            vec.push(fafafa.clone());
            println!("\tscan_two_helper, final thing: {:?}, final vec: {:?}", fafafa.clone(), vec);
            return vec;
        }
    }
}

fn scan_two<I, F, G, B>(mut iterator: I, f: F, g: G) -> Option<Vec<B>> where
    I: Iterator,
    F: FnMut(<I as Iterator>::Item, <I as Iterator>::Item) -> B,
    G: FnMut(<I as Iterator>::Item) -> B,
    <I as Iterator>::Item: Copy + std::fmt::Debug,
    B: Clone + std::fmt::Debug,
{
    let option_next = iterator.next();

    match option_next {
        Some(next) =>
            Some(scan_two_helper(iterator, next, f, g, Vec::new(), next)),
        None => None,
    }
}

fn for_want(str1: &&str, str2: &&str) -> String {
    format!("For want of a {} the {} was lost.", *str1, *str2)
}

fn and_all(str1: &&str) -> String {
    format!("And all for the want of a {}.", str1)
}

pub fn build_proverb_mine(list: &[&str]) -> String {
    // Given a list of inputs, generate the relevant proverb. For example, given the list `["nail",
    // "shoe", "horse", "rider", "message", "battle", "kingdom"]`, you will output the full text of
    // this proverbial rhyme:
    //
    // For want of a nail the shoe was lost.
    // For want of a shoe the horse was lost.
    // For want of a horse the rider was lost.
    // For want of a rider the message was lost.
    // For want of a message the battle was lost.
    // For want of a battle the kingdom was lost.
    // And all for the want of a nail.

    let who: Option<Vec<String>> = scan_two(list.into_iter(), for_want, and_all);

    match who {
        None => String::new(),
        Some(vec_str) => vec_str.join("\n"),
    }
}

pub fn build_proverb(list: &[&str]) -> String {
    // build_proverb_2(list.to_vec())
    build_proverb_iterator(list)
}

// impl<A: Iterator, B: Iterator> Zip<A, B> {
//     pub(in super::super) fn new(a: A, b: B) -> Zip<A, B> {
//         ZipImpl::new(a, b)
//     }
//     fn super_nth(&mut self, mut n: usize) -> Option<(A::Item, B::Item)> {
//         while let Some(x) = Iterator::next(self) {
//             if n == 0 { return Some(x) }
//             n -= 1;
//         }
//         None
//     }
// }

enum FoldTwo<A> {
    Two(A, A),
    Last(A),
}

struct FoldTwoIter<A> where A: Iterator {
    iter: A,
    option_this_val: Option<<A as Iterator>::Item>,
}


impl<I> Iterator for FoldTwoIter<I> where
  I: Iterator,
  <I as Iterator>::Item: Copy,
{
    type Item = FoldTwo<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        // ZipImpl::next(self)
        match self.option_this_val {
            None => None,
            Some(this_val) => {
                let option_next_val = self.iter.next();
                match option_next_val {
                    None => {
                        self.option_this_val = None;
                        Some(FoldTwo::Last(this_val))
                    }
                    Some(next_val) => {
                        self.option_this_val = Some(next_val);
                        Some(FoldTwo::Two(this_val, next_val))
                    }
                }
            }
        }
    }
}

// impl<I: Iterator> FoldTwoIter<I> {
//     fn fold_two(mut i: I) -> Self {
//         let option_first_val = i.next();
//         Self{iter: i, option_this_val: option_first_val}
//     }
// }

impl<A> std::slice::Iter<'_, A> {
    fn fold_two(&mut self) -> FoldTwoIter<std::slice::Iter<A>> {
        let option_first_val = self.next();
        FoldTwoIter{iter: self, option_this_val: option_first_val}
    }
}

fn into_fold_two<I>(mut i: I) -> FoldTwoIter<I> where
    I: Iterator
{
    let option_first_val = i.next();
    FoldTwoIter{iter: i, option_this_val: option_first_val}
}

fn yo_my_map(fold_two: FoldTwo<&&str>) -> String {
    match fold_two {
        FoldTwo::Last(str) =>
            format!("And all for the want of a {}.", str),
        FoldTwo::Two(this, next) =>
            format!("For want of a {} the {} was lost.", this, next),
    }
}

pub fn build_proverb_iterator(list: &[&str]) -> String {
    let list_iter: std::slice::Iter<&str> = list.into_iter();

    let fold_two_iter: FoldTwoIter<std::slice::Iter<&str>> = into_fold_two(list_iter);
    // let fold_two_iter: FoldTwoIter<std::slice::Iter<&str>> = list_iter.fold_two();

    let after_map: std::iter::Map<FoldTwoIter<std::slice::Iter<&str>>, _> =
        fold_two_iter.map(yo_my_map);

    let after_map_vec: Vec<String> = after_map.collect();

    after_map_vec.join("\n")
}

pub fn build_proverb_2(list: Vec<&str>) -> String {
    if list.is_empty() {
        return String::new();
    }

    list.windows(2)
        .map(|win| format!("For want of a {0} the {1} was lost.", win[0], win[1]))
        .chain(std::iter::once(format!(
            "And all for the want of a {}.",
            list[4]
        )))
        .collect::<Vec<_>>()
        .join("\n")
}
