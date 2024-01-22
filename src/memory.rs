use std::{fs::File, io::Read};

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

    pub fn from_bytes(file_path:&String) -> Self {
       

        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(err) => {
                panic!("Failed to open file: {}", err);
            }
        };

        let mut file_contents = Vec::new();

        if let Err(err) = file.read_to_end(&mut file_contents) {
            panic!("Failed to read file: {}", err);
        }

        let mut memory = Memory::new();

        if file_contents.len() > MEM_SIZE {
            panic!("program or file is too large to fit in memory.");
        }

        for (i, v) in file_contents.iter().enumerate() {
            memory.write_u8(i, *v as u8);
        }

        memory
    }
}
