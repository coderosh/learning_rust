pub fn get_op_precedence(char: &str) -> i8 {
    match char {
        "+" | "-" => 1,
        "*" | "/" => 2,
        "^" => 3,
        _ => 0,
    }
}
