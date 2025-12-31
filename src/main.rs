#![warn(
    clippy::all,
    // clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

mod memory;
mod program;
mod util;

use crate::memory::Memory;
use crate::program::Program;
use std::io;

fn main() {
    let mut source_in = String::new();
    io::stdin().read_line(&mut source_in).unwrap();

    let source = source_in.trim();
    let memory = match Memory::new(128) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error creating memory: {e}");
            return;
        }
    };

    let mut program = match Program::new(&source, memory) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error creating program: {e}");
            return;
        }
    };

    loop {
        match program.execute() {
            Ok(has_ended) => {
                if has_ended {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error executing program: {e}");
                return;
            }
        }
    }

    println!(
        "\n\nProgram ended. Total execution count: {}",
        program.execution_count
    );
}
