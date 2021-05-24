#[derive(Debug, Default)]
pub struct Reg {
    pub a: u8,
    pub b: u8,
    pub flag: bool,
    pub pc: u8,
}
