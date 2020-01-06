pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    } else {
        u64::pow(2, s - 1)
    }
}

pub fn total() -> u64 {
    // let orig_res = u64::pow(2, 64) - 1;
    // println!("{} {} {} {} {} {} {} {}", square(1), square(2), square(3), square(4), square(5), square(6), square(1) + square(2) + square(3) + square(4) + square(5) + square(6), u64::pow(2, 6) - 1);
    let res = (1u32..65).map(square).sum();

    // println!("{} {}", res, orig_res);

    res
}
