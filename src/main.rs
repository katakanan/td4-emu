extern crate num_derive;

mod emulator;
mod mem;
mod opecode;
mod port;
mod reg;

use emulator::*;

fn main() {
    println!("Hello, world!");

    let emu = Emulator::new();

    println!("{:?}", emu.fetch());
}
