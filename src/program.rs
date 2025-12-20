use crate::build_bracket_map::build_bracket_map;
use crate::memory::Memory;
use std::collections::HashMap;

#[derive(Debug)]
struct Program<'a> {
    source: &'a str,
    bracket_map: HashMap<usize, usize>,
    pointer: usize,
    memory: Memory,
    execution_count: u8,
}

impl<'a> Program<'a> {
    fn new(source: &'a str, memory: Memory) -> Result<Self, String> {
        Ok(Self {
            source,
            bracket_map: build_bracket_map(source)?,
            pointer: self.initialize_pointer()?,
            memory,
            execution_count: 0,
        })
    }

    // TODO: pointer will be initialized to first executable (non-comment) code.
    fn initialize_pointer(&mut self) -> Result<usize, String> {}

    fn execute(&mut self) -> Result<u8, String> {
        if self.pointer >= self.source.len() {
            return Err("Executed all instructions.".into());
        }

        if !"><+-.,[]".contains(self.source.chars().nth(self.pointer).unwrap()) {
            self.pointer += 1;
            self.execution_count += 1;
            return Ok(self.execution_count);
        }

        Ok(0)
    }
}
