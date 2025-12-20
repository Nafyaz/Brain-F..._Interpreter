#[derive(Debug)]
pub struct Memory {
    memory: Vec<u8>,
    pointer: usize,
}

impl Memory {
    fn new(size: usize) -> Result<Self, String> {
        if size == 0 {
            return Err("Memory size cannot be 0.".into());
        }

        Ok(Self {
            memory: vec![0; size],
            pointer: 0,
        })
    }

    fn move_right(&mut self) -> Result<(), String> {
        if self.pointer == self.memory.len() - 1 {
            return Err("Insufficient memory. Cannot move right.".into());
        }

        self.pointer += 1;

        Ok(())
    }

    fn move_left(&mut self) -> Result<(), String> {
        if self.pointer == 0 {
            return Err("Insufficient memory. Cannot move left.".into());
        }
        self.pointer -= 1;

        Ok(())
    }

    fn jump(&mut self, address: usize) -> Result<(), String> {
        if address >= self.memory.len() {
            return Err("Insufficient memory. Address out of bounds".into());
        }

        self.pointer = address;

        Ok(())
    }
}
