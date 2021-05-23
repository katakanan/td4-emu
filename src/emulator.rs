use crate::mem::*;
use crate::operand::*;
use crate::port::*;
use crate::reg::*;

#[derive(Debug)]
pub struct Emulator {
    pub prg: Mem,
    pub reg: Reg,
    pub port: Port,
}

impl Emulator {
    pub fn new() -> Emulator {
        let prg = Mem::load_new("prg.bin");

        let reg = Reg::default();
        let port = Port::default();

        Emulator { prg, reg, port }
    }
}
