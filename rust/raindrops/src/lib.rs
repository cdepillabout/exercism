pub fn raindrops(n: u32) -> String {
    // - has 3 as a factor, add 'Pling' to the result.
    // - has 5 as a factor, add 'Plang' to the result.
    // - has 7 as a factor, add 'Plong' to the result.
    // - _does not_ have any of 3, 5, or 7 as a factor, the result should be the digits of the number.
    let mut res: String = String::new();

    if n % 3 == 0 {
        res += "Pling";
    }
    if n % 5 == 0 {
        res += "Plang";
    }
    if n % 7 == 0 {
        res += "Plong";
    }

    if n % 3 != 0 && n % 5 != 0 && n % 7 != 0 {
        res = n.to_string();
    }

    return res;
}
