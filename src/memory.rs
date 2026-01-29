use std::io;
use std::io::Read;

#[derive(Debug)]
pub struct Memory {
    tape: Vec<u8>,
    pointer: usize,
}

impl Memory {
    pub fn new(size: usize) -> Result<Self, String> {
        if size == 0 {
            return Err("Memory size cannot be 0.".into());
        }

        Ok(Self {
            tape: vec![0; size],
            pointer: 0,
        })
    }

    pub fn read(&self) -> u8 {
        self.tape[self.pointer]
    }

    pub fn move_right(&mut self) -> Result<(), String> {
        if self.pointer == self.tape.len() - 1 {
            return Err("Insufficient memory. Cannot move right.".into());
        }

        self.pointer += 1;

        Ok(())
    }

    pub fn move_left(&mut self) -> Result<(), String> {
        if self.pointer == 0 {
            return Err("Insufficient memory. Cannot move left.".into());
        }
        self.pointer -= 1;

        Ok(())
    }

    pub fn add(&mut self) {
        self.tape[self.pointer] = self.tape[self.pointer].wrapping_add(1);
    }

    pub fn subtract(&mut self) {
        self.tape[self.pointer] = self.tape[self.pointer].wrapping_sub(1);
    }

    pub fn output(&mut self) {
        print!("{}", self.tape[self.pointer] as char);
    }

    pub fn input(&mut self) -> Result<(), String> {
        let mut buffer = [0u8; 1];
        io::stdin()
            .read_exact(&mut buffer)
            .map_err(|e| e.to_string())?;
        self.tape[self.pointer] = buffer[0];

        Ok(())
    }
}
