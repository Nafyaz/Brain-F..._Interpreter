use crate::memory::Memory;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Program<'a> {
    source: &'a str,
    bracket_map: HashMap<usize, usize>,
    pointer: usize,
    memory: Memory,
    pub execution_count: u32,
}

impl<'a> Program<'a> {
    pub fn new(source: &'a str, memory: Memory) -> Result<Self, String> {
        Ok(Self {
            source,
            bracket_map: Self::build_bracket_map(source)?,
            pointer: 0,
            memory,
            execution_count: 0,
        })
    }

    fn build_bracket_map(source: &str) -> Result<HashMap<usize, usize>, String> {
        let mut stk = Vec::new();
        let mut bracket_map = HashMap::new();

        for (idx, c) in source.chars().enumerate() {
            match c {
                '[' => stk.push(idx),
                ']' => {
                    let open_idx = stk
                        .pop()
                        .ok_or_else(|| format!("Unmatched bracket at position {idx}"))?;

                    bracket_map.insert(open_idx, idx);
                    bracket_map.insert(idx, open_idx);
                }
                _ => {}
            }
        }

        if let Some(lst) = stk.last() {
            return Err(format!("Unmatched Bracket at position {lst}"));
        }

        Ok(bracket_map)
    }

    const fn jump(&mut self, address: usize) {
        self.pointer = address;
    }

    pub fn execute(&mut self) -> Result<bool, String> {
        if self.pointer >= self.source.len() {
            return Err("Executed all instructions.".into());
        }

        let instruction = self.source.as_bytes()[self.pointer] as char;

        match instruction {
            '>' => self.memory.move_right()?,
            '<' => self.memory.move_left()?,
            '+' => self.memory.add(),
            '-' => self.memory.subtract(),
            '.' => self.memory.output(),
            ',' => self.memory.input()?,
            '[' => {
                if self.memory.read() == 0 {
                    self.jump(self.bracket_map[&self.pointer]);
                    self.execution_count += 1;
                    return Ok(false);
                }
            }
            ']' => {
                if self.memory.read() != 0 {
                    self.jump(self.bracket_map[&self.pointer]);
                    self.execution_count += 1;
                    return Ok(false);
                }
            }
            _ => {}
        }

        self.pointer += 1;
        self.execution_count += 1;
        Ok(self.pointer == self.source.len())
    }
}
