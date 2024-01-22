use memory::Memory;

use crate::cpu::CPU;

mod opcodes;
mod cpu;
mod memory;
mod register;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a file path as a command line argument.");
    }
    let file_path = &args[1];
    let mut memory = Memory::from_bytes(file_path);
    let mut cpu = CPU::new();
    cpu.run(&mut memory);
    println!("Program exited with code: {}", cpu.registers[0]);
}

