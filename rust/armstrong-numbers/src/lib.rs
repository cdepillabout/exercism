pub fn is_armstrong_number(num: u32) -> bool {
    // unimplemented!("true if {} is an armstrong number", num)

    let num_str: String = num.to_string();
    let len = num_str.len();
    let res: u32 = num_str.chars()
        .map(|c| c.to_digit(10).unwrap().pow(len as u32))
        .sum();

    res == num
}
