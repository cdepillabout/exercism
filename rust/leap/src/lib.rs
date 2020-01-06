use num::Zero;
use std::ops::Rem;

fn is_div<A, B>(i: A, j: B) -> bool
where
    A: Rem<B>,
    <A as Rem<B>>::Output: PartialEq + Zero
{
    let rem: <A as Rem<B>>::Output = i % j;
    let zero: <A as Rem<B>>::Output = <A as Rem<B>>::Output::zero();
    rem == zero
}

pub fn is_leap_year(year: u64) -> bool {
    is_div(year, 4) && (!is_div(year, 100) || is_div(year, 400))
}
