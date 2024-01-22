pub const MEM_SIZE : usize = 4096;
pub struct Memory {
    pub memory: [u8; MEM_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            memory: [0; MEM_SIZE],
        }
    }
    pub fn write_i32(&mut self, addr: usize, value: i32) {
        let bytes = value.to_le_bytes();
        self.memory[addr..addr+4].copy_from_slice(&bytes);
    }
    pub fn write_u8(&mut self, addr: usize, value: u8) {
        self.memory[addr] = value;
    }
    pub fn read_i32(&mut self, addr: usize) -> i32 {
        let bytes = &self.memory[addr..addr+4];
        i32::from_le_bytes([bytes[0],bytes[1],bytes[2],bytes[3]])
    }
    pub fn read_u8(&mut self, addr: usize) -> u8 {
        self.memory[addr]
    }
}
