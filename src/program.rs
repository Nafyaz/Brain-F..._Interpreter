use crate::build_bracket_map::build_bracket_map;
use std::collections::HashMap;

#[derive(Debug)]
struct Program<'a> {
    code: &'a str,
    bracket_map: HashMap<usize, usize>,
}

impl<'a> Program<'a> {
    fn new(code: &'a str) -> Result<Self, String> {
        Ok(Self {
            code,
            bracket_map: build_bracket_map(code)?,
        })
    }
}
