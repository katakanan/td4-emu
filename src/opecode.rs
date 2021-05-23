use num_derive::FromPrimitive;

#[derive(Debug, FromPrimitive)]
pub enum Opecode {
    ADD2A = 0b0000,
    ADD2B = 0b0101,
    MOV2A = 0b0011,
    MOV2B = 0b0111,
    MOVA2B = 0b0001,
    JMP = 0b1111,
    JNC = 0b1110,
    INA = 0b0010,
    INB = 0b0110,
    OUTB = 0b1001,
    OUTIM = 0b1011,
    NOP,
}
