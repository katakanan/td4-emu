extern crate num_derive;

mod emulator;
mod mem;
mod opecode;
mod port;
mod reg;

use emulator::*;

fn main() {
    println!("Hello, world!");

    let mut emu = Emulator::new("prg.bin");

    println!("{:?}", emu);

    for _ in 0..10 {
        let (opecode, operand) = emu.fetch_decode();
        println!("{:?} {:?}", opecode, operand);
        let next_pc = emu.exec_mut(&opecode, operand);
        emu.reg.pc = next_pc;
        println!("{:?}", emu);
    }
}
