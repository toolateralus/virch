use crate::{memory::Memory, opcodes::{Opcode, ProgramBuilder}, register::*};
use super::*;

#[test]
fn cpu_initialization() {
	let mut cpu = CPU::new();
	cpu.initialize();
	assert_eq!(cpu.ip, 0);
	assert_eq!(cpu.read_register(RT), 0);
}

#[test]
fn program_builder() {
	let mut builder = ProgramBuilder::new();
	builder.instruction(Opcode::Load);
	builder.u32(6);
	builder.i32(255);
	builder.u8(245);
	
	let mut memory = Memory::new();
	builder.build(&mut memory);

	let memory = memory.memory; // the inner array.
	assert_eq!(memory[0], Opcode::Load as u8, "instruction (load) failed to build.");
	assert_eq!(memory[1..5], [0x06,0x00,0x00,0x00], "u32 failed to build.");
	assert_eq!(memory[5..9], [0xFF,0x00,0x00,0x00], "i32 failed to build.");
	assert_eq!(memory[9], 245, "u8 failed to build.");
}
#[test]
fn multiplication() {
	let mut builder = ProgramBuilder::new();
	builder.instruction(Opcode::Mul);
	
	let mut mem = Memory::new();
	builder.build(&mut mem);
	
	let mut cpu = CPU::new();
	cpu.write_register(A, 250);
	cpu.write_register(B, 300);
	cpu.cycle(&mut mem);
	
	assert_eq!(cpu.read_register(A), 75000);
}
#[test]
fn division() {
	let mut builder = ProgramBuilder::new();
	builder.instruction(Opcode::Div);
	
	let mut mem = Memory::new();
	builder.build(&mut mem);
	
	let mut cpu = CPU::new();
	cpu.write_register(A, 500);
	cpu.write_register(B, 2);
	cpu.cycle(&mut mem);
	
	assert_eq!(cpu.read_register(A), 250);
}
#[test]
fn modulus() {
	let mut builder = ProgramBuilder::new();
	builder.instruction(Opcode::Div);
	
	let mut mem = Memory::new();
	builder.build(&mut mem);
	
	let mut cpu = CPU::new();
	cpu.write_register(A, 5);
	cpu.write_register(B, 3);
	cpu.cycle(&mut mem);
	
	assert_eq!(cpu.read_register(C), 2);
}
#[test]
fn addition() {
	let mut builder = ProgramBuilder::new();
	builder.instruction(Opcode::Add);
	
	let mut mem = Memory::new();
	builder.build(&mut mem);
	
	let mut cpu = CPU::new();
	cpu.write_register(A, 123);
	cpu.write_register(B, 1);
	cpu.cycle(&mut mem);
	
	assert_eq!(cpu.read_register(A), 124);
}
#[test]
fn subtraction() {
	let mut builder = ProgramBuilder::new();
	builder.instruction(Opcode::Sub);
	
	let mut mem = Memory::new();
	builder.build(&mut mem);
	
	let mut cpu = CPU::new();
	cpu.write_register(A, 432);
	cpu.write_register(B, 1);
	cpu.cycle(&mut mem);
	
	assert_eq!(cpu.read_register(A), 431);
}
#[test]
pub fn loading() {
	let mut builder = ProgramBuilder::new();
	builder.instruction(Opcode::Load);
	builder.u32(0);
	builder.i32(123);
	
	let mut mem = Memory::new();
	builder.build(&mut mem);
	
	let mut cpu = CPU::new();
	cpu.cycle(&mut mem);
	
	assert_eq!(cpu.read_register(A), 123);
}
#[test]
fn storing() {
	let mut builder = ProgramBuilder::new();
	builder.instruction(Opcode::Store);
	builder.u32(0);
	builder.i32(100);
	
	let mut mem = Memory::new();
	builder.build(&mut mem);
	
	let mut cpu = CPU::new();
	cpu.write_register(A, 432);
	cpu.cycle(&mut mem);
	
	assert_eq!(mem.read_i32(100), 432);
}
#[test]
fn jumping() {
	let mut builder = ProgramBuilder::new();
	builder.instruction(Opcode::Jump);
	builder.i32(100);
	
	let mut mem = Memory::new();
	builder.build(&mut mem);
	
	let mut cpu = CPU::new();
	cpu.cycle(&mut mem);
	
	assert_eq!(cpu.ip, 100);
}
#[test]
fn compare_equal() {
	let mut builder = ProgramBuilder::new();
	builder.instruction(Opcode::Cmpi);
	
	let mut mem = Memory::new();
	builder.build(&mut mem);
	
	let mut cpu = CPU::new();
	cpu.write_register(A, 123);
	cpu.write_register(B, 123);
	cpu.cycle(&mut mem);
	
	assert_eq!(cpu.read_register(A), 1);
}
#[test]
fn compare_not_equal() {
	let mut builder = ProgramBuilder::new();
	builder.instruction(Opcode::Cmpi);
	
	let mut mem = Memory::new();
	builder.build(&mut mem);
	
	let mut cpu = CPU::new();
	cpu.write_register(A, 123);
	cpu.write_register(B, 321);
	cpu.cycle(&mut mem);
	
	assert_eq!(cpu.read_register(A), 0);
}
#[test]
fn halting() {
	let mut builder = ProgramBuilder::new();
	builder.instruction(Opcode::Halt);
	
	let mut mem = Memory::new();
	builder.build(&mut mem);
	
	let mut cpu = CPU::new();
	assert!(!cpu.cycle(&mut mem));
}
#[test]
fn no_operation() {
	let mut builder = ProgramBuilder::new();
	builder.instruction(Opcode::Nop);
	
	let mut mem = Memory::new();
	builder.build(&mut mem);
	
	let mut cpu = CPU::new();
	assert!(cpu.cycle(&mut mem));
}
#[test]
fn cycle_counting() {
	let mut builder = ProgramBuilder::new();
	builder.instruction(Opcode::Nop);
	builder.instruction(Opcode::Nop);
	builder.instruction(Opcode::Nop);
	
	let mut mem = Memory::new();
	builder.build(&mut mem);
	
	let mut cpu = CPU::new();
	cpu.cycle(&mut mem);
	cpu.cycle(&mut mem);
	cpu.cycle(&mut mem);
	
	assert_eq!(cpu.read_register(RT), 3);
}