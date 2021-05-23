use crate::mem::*;
use crate::opecode::*;
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

    pub fn fetch(&self) -> (Opecode, u8) {
        let pc = self.reg.pc;

        let bin = self.prg.mem.get(pc as usize).unwrap();

        let opecode = num_traits::FromPrimitive::from_u8(bin >> 4).unwrap_or_else(|| Opecode::NOP);
        let operand = bin & 0x0F;

        (opecode, operand)
    }
}
