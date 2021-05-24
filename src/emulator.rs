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

    pub fn fetch_decode(&self) -> (Opecode, u8) {
        let pc = self.reg.pc;

        let bin = self.prg.mem.get(pc as usize).unwrap();

        let opecode = num_traits::FromPrimitive::from_u8(bin >> 4).unwrap_or_else(|| Opecode::NOP);
        let operand = bin & 0x0F;

        (opecode, operand)
    }

    pub fn exec_mut(&mut self, opecode: &Opecode, operand: u8) -> u8 {
        match opecode {
            Opecode::ADD2A => {
                let tmp = self.reg.a + operand;
                self.reg.flag = (0x10 & tmp) != 0;
                self.reg.a = 0x0F & tmp;
            }
            Opecode::ADD2B => {
                let tmp = self.reg.b + operand;
                self.reg.flag = (0x10 & tmp) != 1;
                self.reg.a = 0x0F & tmp;
            }
            Opecode::MOV2A => {
                self.reg.a = operand;
                self.reg.flag = false;
            }
            Opecode::MOV2B => {
                self.reg.b = operand;
                self.reg.flag = false;
            }
            Opecode::MOVA2B => {
                self.reg.b = self.reg.a;
                self.reg.flag = false;
            }
            Opecode::INA => {
                self.reg.a = self.port.input;
                self.reg.flag = false;
            }
            Opecode::INB => {
                self.reg.b = self.port.input;
                self.reg.flag = false;
            }
            Opecode::OUTB => {
                self.port.output = self.reg.b;
                self.reg.flag = false;
            }
            Opecode::OUTIM => {
                self.port.output = operand;
                self.reg.flag = false;
            }
            Opecode::JMP => {
                self.reg.flag = false;
                return operand;
            }
            Opecode::JNC => {
                if self.reg.flag {
                    self.reg.flag = false;
                    return operand;
                }
            }
            Opecode::NOP => {
                //NOP
                self.reg.flag = false;
            }
        }

        self.reg.pc + 1
    }
}
