#[allow(dead_code)]
#[repr(u8)]
pub enum Instruction {
    Halt,
    Jump,
    Store,
    Load,
    Add,
    Sub,
    Mul,
    Div,
    Cmpi, // compare integer.
}

impl Instruction {
    pub fn from(value: u8) -> Result<Instruction, ()> {
        match value {
            0 => Ok(Instruction::Halt),
            1 => Ok(Instruction::Jump),
            2 => Ok(Instruction::Store),
            3 => Ok(Instruction::Load),
            4 => Ok(Instruction::Add),
            5 => Ok(Instruction::Sub),
            6 => Ok(Instruction::Mul),
            7 => Ok(Instruction::Div),
            8 => Ok(Instruction::Cmpi),
            _ => Err(()),
        }
    }
}