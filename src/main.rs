extern crate num_derive;

mod emulator;
mod mem;
mod operand;
mod port;
mod reg;

use emulator::*;

fn main() {
    println!("Hello, world!");

    let emu = Emulator::new();

    println!("{:?}", emu);
}
