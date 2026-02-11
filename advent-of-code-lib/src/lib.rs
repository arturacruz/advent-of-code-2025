use std::fs;

pub enum InputType {
    Input, Test
}

pub fn get_input(ty: InputType) -> String {
    let path = match ty {
        InputType::Test => "assets/test.txt",
        InputType::Input => "assets/input.txt"
    };
    fs::read_to_string(path).expect("Input file not found")
}

pub fn parse_number(s: &str) -> i64 {
    s.parse().expect("Failed to parse into i64.")
}

