#![warn(
    clippy::all,
    // clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

mod memory;
mod program;
mod text_to_bf;

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    for c in input.chars() {
        let n = c as u32;
        let sq_low = n.isqrt();
        let sq_high = n / sq_low;
        let rem = n % (sq_low * sq_high);

        let answer = "+".repeat(sq_low as usize)
            + "[>"
            + "+".repeat(sq_high as usize).as_str()
            + "<-]>"
            + "+".repeat(rem as usize).as_str()
            + ".[-]<";
        println!("{}", answer);
    }
}
