use std::collections::HashMap;

#[derive(Debug)]
struct Program {
    code: Vec<char>,
    left_bracket_map: HashMap<char, char>,
    right_bracket_map: HashMap<char, char>,
}

impl Program {}
