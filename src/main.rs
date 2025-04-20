extern crate piston_window;
extern crate image;
mod cpu;
mod opcode;
mod loading;
use piston_window::*;
use std::env;
use std::path::Path;

fn main() {
	let mut args = env::args();
	let (min, _) = args.size_hint();
	if min < 2 {
		println!("No ROM specified");
		return;
	}
	let multiplier: u32;
	args.next();
	let path = args.next().unwrap();
	if let Some(multi) = args.next() {
		//Initialize a multiplier
		multiplier = multi.trim().parse().ok().unwrap();
	} else {
		multiplier = 2;
	}
	let mut window: PistonWindow = WindowSettings::new("RustChip16", [320 * multiplier, 240 * multiplier])
		.exit_on_esc(true)
		.build()
		.unwrap();
	let mut cpu = cpu::Cpu::new(Path::new(&path[..]), multiplier);
	cpu.start_program(&mut window);
}

mod tests {
	#![allow(overflowing_literals, dead_code)]
	use cpu::Cpu;
	use opcode::Opcode;
	
	fn stage_1op_test(op: Opcode, byte1: i8, byte2: i8, byte3: i8) -> Cpu {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(op, byte1, byte2, byte3);
		cpu.start_test(1);
		cpu
	}

	#[test]
	fn new_cpu() -> () {
		let cpu = stage_1op_test(Opcode::Nop, 0, 0, 0);
		assert_eq!(cpu.pc, 4);
	}

	#[test]
	fn jmp() -> () {
		let cpu = stage_1op_test(Opcode::Jmp, 0, 0x10, 0x20);
		assert_eq!(cpu.pc, 0x2010);
	}
	
	#[test]
	fn flip() -> () {
		let cpu0 = stage_1op_test(Opcode::Flip, 0, 0, 0);
		let cpu1 = stage_1op_test(Opcode::Flip, 0, 0, 1);
		let cpu2 = stage_1op_test(Opcode::Flip, 0, 0, 2);
		let cpu3 = stage_1op_test(Opcode::Flip, 0, 0, 3);
		assert_eq!(cpu0.graphics.state.hflip, false);
		assert_eq!(cpu0.graphics.state.vflip, false);
		
		assert_eq!(cpu1.graphics.state.hflip, false);
		assert_eq!(cpu1.graphics.state.vflip, true);
		
		assert_eq!(cpu2.graphics.state.hflip, true);
		assert_eq!(cpu2.graphics.state.vflip, false);
		
		assert_eq!(cpu3.graphics.state.hflip, true);
		assert_eq!(cpu3.graphics.state.vflip, true);
	}
	
	#[test]
	fn flags() -> () {
		let mut cpu = Cpu::new_test();
		cpu.put_carry(true);
		cpu.put_zero(true);
		cpu.put_overflow(true);
		cpu.put_negative(true);
		assert!(cpu.has_carry() && cpu.has_zero() && cpu.has_overflow() && cpu.has_negative());
		cpu.put_carry(false);
		cpu.put_zero(false);
		cpu.put_overflow(false);
		cpu.put_negative(false);
		assert!(!(cpu.has_carry() || cpu.has_zero() || cpu.has_overflow() || cpu.has_negative()));
	}
	
	#[test]
	fn jmc() -> () {
		let mut cpu0 = Cpu::new_test();
		cpu0.put_carry(true);
		cpu0.add_opcode(Opcode::Jmc, 0, 0x10, 0x20);
		cpu0.start_test(1);
		assert_eq!(cpu0.pc, 0x2010);
		
		let mut cpu1 = Cpu::new_test();
		cpu1.put_carry(false);
		cpu1.add_opcode(Opcode::Jmc, 0, 0x10, 0x20);
		cpu1.start_test(1);
		assert_eq!(cpu1.pc, 0x4);
	}
	
	#[test]
	fn jx_flags() -> () {
		let mut cpu = Cpu::new_test();
		cpu.put_zero(true);
		if !cpu.check_flags(0) {
			panic!("{}", 0);
		}
		
		cpu.put_zero(false);
		if !cpu.check_flags(1) {
			panic!("{}", 1);
		}
		
		cpu.put_negative(true);
		if !cpu.check_flags(2) {
			panic!("{}", 2);
		}
		
		cpu.put_negative(false);
		if !cpu.check_flags(3) {
			panic!("{}", 3);
		}
		
		cpu.put_zero(false);
		cpu.put_negative(false);
		if !cpu.check_flags(4) {
			panic!("{}", 4);
		}
		
		cpu.put_overflow(true);
		if !cpu.check_flags(5) {
			panic!("{}", 5);
		}
		
		cpu.put_overflow(false);
		if !cpu.check_flags(6) {
			panic!("{}", 6);
		}
		
		cpu.put_carry(false);
		cpu.put_zero(false);
		if !cpu.check_flags(7) {
			panic!("{}", 7);
		}
		
		if !cpu.check_flags(8) {
			panic!("{}", 8);
		}
		
		cpu.put_carry(true);
		if !cpu.check_flags(9) {
			panic!("{}", 9);
		}
		
		cpu.put_carry(true);
		if !cpu.check_flags(10) {
			panic!("{}", 10);
		}
		
		cpu.put_negative(true);
		cpu.put_overflow(true);
		cpu.put_zero(false);
		if !cpu.check_flags(11) {
			panic!("{}", 11);
		}
		
		cpu.put_negative(false);
		cpu.put_overflow(false);
		cpu.put_zero(false);
		if !cpu.check_flags(11) {
			panic!("{}", 11);
		}
		
		cpu.put_negative(true);
		cpu.put_overflow(true);
		cpu.put_zero(true);
		if cpu.check_flags(11) {
			panic!("{}", 11);
		}
		
		cpu.put_negative(true);
		cpu.put_overflow(true);
		if !cpu.check_flags(12) {
			panic!("{}", 12);
		}
		
		cpu.put_negative(false);
		cpu.put_overflow(false);
		if !cpu.check_flags(12) {
			panic!("{}", 12);
		}
		
		cpu.put_negative(false);
		cpu.put_overflow(true);
		if cpu.check_flags(12) {
			panic!("{}", 12);
		}
		
		cpu.put_overflow(true);
		cpu.put_negative(false);
		if !cpu.check_flags(13) {
			panic!("{}", 13);
		}
		
		cpu.put_overflow(true);
		cpu.put_negative(true);
		if cpu.check_flags(13) {
			panic!("{}", 13);
		}
		
		cpu.put_overflow(true);
		cpu.put_negative(false);
		cpu.put_zero(false);
		if !cpu.check_flags(14) {
			panic!("{}", 14);
		}
		
		cpu.put_overflow(true);
		cpu.put_negative(true);
		cpu.put_zero(false);
		if cpu.check_flags(14) {
			panic!("{}", 14);
		}
		
		cpu.put_overflow(false);
		cpu.put_negative(false);
		cpu.put_zero(true);
		if !cpu.check_flags(14) {
			panic!("{}", 14);
		}
	}
	
	#[test]
	fn jme() -> () {
		let mut cpu = Cpu::new_test();
		cpu.set_rx(5, 300);
		cpu.set_rx(6, 300);
		cpu.add_opcode(Opcode::Jme, 0x56, 0x10, 0x20);
		cpu.start_test(1);
		assert_eq!(cpu.pc, 0x2010);

		cpu.pc = 0;
		cpu.set_rx(5, 300);
		cpu.set_rx(6, 299);
		cpu.add_opcode(Opcode::Jme, 0x56, 0x10, 0x20);
		cpu.start_test(1);
		assert_eq!(cpu.pc, 0x4);
	}
	
	#[test]
	fn call() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Call, 0, 0x10, 0x20);
		cpu.pc = 0x2010;
		cpu.add_opcode(Opcode::Ret, 0, 0, 0);
		cpu.start_test(1);
		assert_eq!(cpu.pc, 0x2010);
		assert_eq!(cpu.sp, 0xFDF2);
		assert_eq!(cpu.memory.read_word(0xFDF0), 0x4);
		
		cpu.step();
		
		assert_eq!(cpu.pc, 0x4);
	}
	
	#[test]
	fn ldi() -> () {
		let mut cpu = stage_1op_test(Opcode::Ldi, 5, 0x10, 0x20);
		assert_eq!(cpu.get_rx(5), 0x2010);
	}
	
	#[test]
	fn ldisp() -> () {
		let cpu = stage_1op_test(Opcode::Ldi2, 0, 0x10, 0x20);
		assert_eq!(cpu.sp, 0x2010);
	}
	
	#[test]
	fn ldm() -> () {
		let mut cpu = Cpu::new_test();
		cpu.memory.write_word(0x20, 10000);
		cpu.add_opcode(Opcode::Ldm, 5, 0x20, 00);
		cpu.start_test(1);
		
		assert_eq!(10000, cpu.get_rx(5));
	}
	
	#[test]
	fn ldm2() -> () {
		let mut cpu = Cpu::new_test();
		cpu.memory.write_word(0x20, 10000);
		cpu.set_rx(4, 0x20); 
		cpu.add_opcode(Opcode::Ldm2, 0x45, 0, 00);
		cpu.start_test(1);
		
		assert_eq!(10000, cpu.get_rx(5));
	}
	
	#[test]
	fn mov() -> () {
		let mut cpu = Cpu::new_test();
		cpu.set_rx(4, 0x20); 
		cpu.add_opcode(Opcode::Mov, 0x45, 0, 00);
		cpu.start_test(1);
		
		assert_eq!(0x20, cpu.get_rx(5));
	}
	
	#[test]
	fn stm() -> () {
		let mut cpu = Cpu::new_test();
		cpu.set_rx(4, 0x1050);
		cpu.add_opcode(Opcode::Stm, 4, 0x10, 0x20);
		cpu.start_test(1);
		
		assert_eq!(cpu.memory.read_word(0x2010), 0x1050);
	}
	
	#[test]
	fn stm2() -> () {
		let mut cpu = Cpu::new_test();
		cpu.set_rx(4, 0x1050);
		cpu.set_rx(5, 0x20);
		cpu.add_opcode(Opcode::Stm2, 0x54, 0, 0);
		cpu.start_test(1);
		
		assert_eq!(cpu.memory.read_word(0x20), 0x1050);
	}
	
	#[test]
	fn addi() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Addi, 5, 0x05, 0x20);
		cpu.add_opcode(Opcode::Addi, 5, 0xFB, 0xDF); //This is negative 0x2005
		cpu.start_test(1);
		assert_eq!(cpu.get_rx(5), 0x2005);
		cpu.step();
		assert_eq!(cpu.get_rx(5), 0);
		assert!(cpu.has_zero());
	}
	
	#[test]
	fn add() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Add, 0x65, 0, 0);
		cpu.set_rx(5, 300);
		cpu.set_rx(6, 300);
		cpu.start_test(1);
		assert_eq!(cpu.get_rx(5), 600);
		assert!(!cpu.has_zero() && !cpu.has_negative() && !cpu.has_overflow() && !cpu.has_carry());
	}
	
	#[test]
	fn add2() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Add2, 0x65, 7, 0);
		cpu.set_rx(5, 300);
		cpu.set_rx(6, 300);
		cpu.start_test(1);
		assert_eq!(cpu.get_rx(7), 600);
		assert!(!cpu.has_zero() && !cpu.has_negative() && !cpu.has_overflow() && !cpu.has_carry());
	}
	
	#[test]
	fn add_flags() -> () {
		let mut cpu = Cpu::new_test();
		cpu.set_rx(5, -30000);
		cpu.add_opcode(Opcode::Addi, 5, 0xD0, 0x8A);
		cpu.start_test(1);
		assert_eq!(cpu.get_rx(5), 5536);
		assert!(cpu.has_carry());
		assert!(cpu.has_overflow());
		
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Addi, 5, 0xD0, 0x8A);
		cpu.start_test(1);
		assert_eq!(cpu.get_rx(5), -30000);
		assert!(cpu.has_negative());
	}
	
	#[test]
	fn subi() -> () {
		let mut cpu = stage_1op_test(Opcode::Subi, 5, 0x10, 0x20);
		assert_eq!(cpu.get_rx(5), -0x2010);
		assert!(cpu.has_negative());
		assert!(cpu.has_carry());
		assert!(!cpu.has_overflow());
		assert!(!cpu.has_zero());
	}
	
	#[test]
	fn sub() -> () {
		let mut cpu = Cpu::new_test();
		cpu.set_rx(5, 0x2222);
		cpu.set_rx(6, 0x1111);
		cpu.add_opcode(Opcode::Sub, 0x65, 0, 0);
		cpu.start_test(1);
		assert_eq!(cpu.get_rx(5), 0x1111);
	}
	
	#[test]
	fn cmpi_flags() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Cmpi, 5, 0x10, 0x7F);
		cpu.set_rx(5, -30000);
		cpu.start_test(1);
		assert!(!cpu.has_negative());
		assert!(!cpu.has_carry());
		assert!(cpu.has_overflow());
		assert!(!cpu.has_zero());
	}
	
	#[test]
	fn andi() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Andi, 15, 0b101, 0b101);
		cpu.set_rx(15, 0b011000000110);
		cpu.start_test(1);
		assert_eq!(cpu.get_rx(15), 0b010000000100);
		assert!(!cpu.has_negative() && !cpu.has_zero());
	}
	
	#[test]
	fn and() -> () {
		let mut cpu1 = Cpu::new_test();
		cpu1.add_opcode(Opcode::And, 0x65, 0, 0);
		cpu1.set_rx(5, 1);
		cpu1.set_rx(6, 2);
		cpu1.start_test(1);
		assert_eq!(cpu1.get_rx(5), 0);
		assert!(!cpu1.has_negative() && cpu1.has_zero());
		
		let mut cpu2 = Cpu::new_test();
		cpu2.add_opcode(Opcode::And2, 0x65, 7, 0);
		cpu2.set_rx(5, 0xFFFF);
		cpu2.set_rx(6, 0xF00F);
		cpu2.start_test(1);
		assert_eq!(cpu2.get_rx(7), 0xF00F);
		assert!(cpu2.has_negative() && !cpu2.has_zero());
	}
	
	#[test]
	fn ori() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Ori, 15, 0xAB, 0x00);
		cpu.set_rx(15, 0xAB00);
		cpu.start_test(1);
		assert_eq!(cpu.get_rx(15), 0xABAB);
		assert!(cpu.has_negative() && !cpu.has_zero());
	}
	
	#[test]
	fn or() -> () {
		let mut cpu1 = Cpu::new_test();
		cpu1.add_opcode(Opcode::Or, 0x65, 0, 0);
		cpu1.set_rx(5, 1);
		cpu1.set_rx(6, 2);
		cpu1.start_test(1);
		assert_eq!(cpu1.get_rx(5), 3);
		assert!(!cpu1.has_negative() && !cpu1.has_zero());
		
		let mut cpu2 = Cpu::new_test();
		cpu2.add_opcode(Opcode::Or2, 0x65, 7, 0);
		cpu2.set_rx(5, 0x0FF0);
		cpu2.set_rx(6, 0xF00F);
		cpu2.start_test(1);
		assert_eq!(cpu2.get_rx(7), 0xFFFF);
		assert!(cpu2.has_negative() && !cpu2.has_zero());
	}
	
	#[test]
	fn xori() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Xori, 15, 0xAB, 0x01);
		cpu.set_rx(15, 0xAB00);
		cpu.start_test(1);
		assert_eq!(cpu.get_rx(15), 0xAAAB);
		assert!(cpu.has_negative() && !cpu.has_zero());
	}
	
	#[test]
	fn xor() -> () {
		let mut cpu1 = Cpu::new_test();
		cpu1.add_opcode(Opcode::Xor, 0x65, 0, 0);
		cpu1.set_rx(5, 2);
		cpu1.set_rx(6, 2);
		cpu1.start_test(1);
		assert_eq!(cpu1.get_rx(5), 0);
		assert!(!cpu1.has_negative() && cpu1.has_zero());
		
		let mut cpu2 = Cpu::new_test();
		cpu2.add_opcode(Opcode::Xor2, 0x65, 7, 0);
		cpu2.set_rx(5, 0x0FFF);
		cpu2.set_rx(6, 0xF00F);
		cpu2.start_test(1);
		assert_eq!(cpu2.get_rx(7), 0xFFF0);
		assert!(cpu2.has_negative() && !cpu2.has_zero());
	}
	
	#[test]
	fn muli() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Muli, 5, 6, 0);
		cpu.set_rx(5, 10);
		cpu.start_test(1);
		
		assert_eq!(cpu.get_rx(5), 60);
		assert!(!cpu.has_carry() && !cpu.has_negative() && !cpu.has_zero());
	}
	
	#[test]
	fn mul() -> () {
		let mut cpu1 = Cpu::new_test();
		cpu1.add_opcode(Opcode::Mul, 0x65, 0, 0);
		cpu1.set_rx(5, 250);
		cpu1.set_rx(6, 0);
		cpu1.start_test(1);
		assert_eq!(cpu1.get_rx(5), 0);
		assert!(!cpu1.has_negative() && cpu1.has_zero() && !cpu1.has_carry());
		
		let mut cpu2 = Cpu::new_test();
		cpu2.add_opcode(Opcode::Mul2, 0x65, 7, 0);
		cpu2.set_rx(5, 250);
		cpu2.set_rx(6, 250);
		cpu2.start_test(1);
		assert_eq!(cpu2.get_rx(7), -3036);
		assert!(cpu2.has_negative());
		assert!(!cpu2.has_carry());
		assert!(!cpu2.has_zero());
	}
	
	#[test]
	fn divi() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Divi, 5, 6, 0);
		cpu.set_rx(5, 61);
		cpu.start_test(1);
		
		assert_eq!(cpu.get_rx(5), 10);
		assert!(cpu.has_carry() && !cpu.has_negative() && !cpu.has_zero());
	}
	
	#[test]
	fn div() -> () {
		let mut cpu1 = Cpu::new_test();
		cpu1.add_opcode(Opcode::Div, 0x65, 0, 0);
		cpu1.set_rx(5, 250);
		cpu1.set_rx(6, 260);
		cpu1.start_test(1);
		assert_eq!(cpu1.get_rx(5), 0);
		assert!(!cpu1.has_negative() && cpu1.has_zero() && cpu1.has_carry());
		
		let mut cpu2 = Cpu::new_test();
		cpu2.add_opcode(Opcode::Div2, 0x65, 7, 0);
		cpu2.set_rx(5, -250);
		cpu2.set_rx(6, 250);
		cpu2.start_test(1);
		assert_eq!(cpu2.get_rx(7), -1);
		assert!(cpu2.has_negative());
		assert!(!cpu2.has_carry());
		assert!(!cpu2.has_zero());
	}
	
	#[test]
	fn modi() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Modi, 5, 6, 0);
		cpu.set_rx(5, 61);
		cpu.start_test(1);
		
		assert_eq!(cpu.get_rx(5), 1);
		assert!(!cpu.has_negative() && !cpu.has_zero());
	}
	
	#[test]
	fn mod1() -> () {
		let mut cpu1 = Cpu::new_test();
		cpu1.add_opcode(Opcode::Mod, 0x65, 0, 0);
		cpu1.set_rx(5, 250);
		cpu1.set_rx(6, 260);
		cpu1.start_test(1);
		assert_eq!(cpu1.get_rx(5), 250);
		assert!(!cpu1.has_negative() && !cpu1.has_zero());
		
		let mut cpu2 = Cpu::new_test();
		cpu2.add_opcode(Opcode::Mod2, 0x65, 7, 0);
		cpu2.set_rx(5, -240);
		cpu2.set_rx(6, 250);
		cpu2.start_test(1);
		assert_eq!(cpu2.get_rx(7), 10);
		assert!(!cpu2.has_negative());
		assert!(!cpu2.has_zero());
	}
	
	#[test]
	fn remi() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Remi, 5, 6, 0);
		cpu.set_rx(5, 61);
		cpu.start_test(1);
		
		assert_eq!(cpu.get_rx(5), 1);
		assert!(!cpu.has_negative() && !cpu.has_zero());
	}
	
	#[test]
	fn rem() -> () {
		let mut cpu1 = Cpu::new_test();
		cpu1.add_opcode(Opcode::Rem, 0x65, 0, 0);
		cpu1.set_rx(5, 260);
		cpu1.set_rx(6, 260);
		cpu1.start_test(1);
		assert_eq!(cpu1.get_rx(5), 0);
		assert!(!cpu1.has_negative() && cpu1.has_zero());
		
		let mut cpu2 = Cpu::new_test();
		cpu2.add_opcode(Opcode::Rem2, 0x65, 7, 0);
		cpu2.set_rx(5, -240);
		cpu2.set_rx(6, 250);
		cpu2.start_test(1);
		assert_eq!(cpu2.get_rx(7), -240);
		assert!(cpu2.has_negative());
		assert!(!cpu2.has_zero());
	}
	
	#[test]
	fn shl() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Shl, 5, 3, 0);
		cpu.set_rx(5, 1);
		cpu.start_test(1);
		assert_eq!(cpu.get_rx(5), 1<<3);
		assert!(!cpu.has_zero());
		assert!(!cpu.has_negative());
	}
	
	#[test]
	fn shr() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Shr, 5, 3, 0);
		cpu.set_rx(5, 0xF000);
		cpu.start_test(1);
		assert_eq!(cpu.get_rx(5), (0xF000u16 >> 3) as i16);
		assert!(!cpu.has_zero());
		assert!(!cpu.has_negative());
	}
	
	#[test]
	fn sar() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Sar, 5, 4, 0);
		cpu.set_rx(5, 0xF000);
		cpu.start_test(1);
		assert_eq!(cpu.get_rx(5), 0xFF00);
		assert!(!cpu.has_zero());
		assert!(cpu.has_negative());
	}
	
	#[test]
	fn shl2() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Shl2, 0x65, 0, 0);
		cpu.set_rx(5, 1);
		cpu.set_rx(6, 3);
		cpu.start_test(1);
		assert_eq!(cpu.get_rx(5), 1<<3);
		assert!(!cpu.has_zero());
		assert!(!cpu.has_negative());
	}
	
	#[test]
	fn shr2() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Shr2, 0x65, 0, 0);
		cpu.set_rx(5, 0xF000);
		cpu.set_rx(6, 4);
		cpu.start_test(1);
		assert_eq!(cpu.get_rx(5),0x0F00);
		assert!(!cpu.has_zero());
		assert!(!cpu.has_negative());
	}
	
	#[test]
	fn sar2() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Sar2, 0x65, 0, 0);
		cpu.set_rx(5, 0xF000);
		cpu.set_rx(6, 4);
		cpu.start_test(1);
		assert_eq!(cpu.get_rx(5),0xFF00);
		assert!(!cpu.has_zero());
		assert!(cpu.has_negative());
	}
	
	#[test]
	fn push_pop_stack() -> () {
		let mut cpu = Cpu::new_test();
		let sp = cpu.sp;
		assert_eq!(sp, 0xFDF0);
		cpu.add_opcode(Opcode::Push, 0x4, 0, 0);
		cpu.add_opcode(Opcode::Pop, 0x5, 0, 0);
		cpu.set_rx(4, 1000);
		cpu.start_test(1);
		
		assert_eq!(cpu.sp, 0xFDF2);
		assert_eq!(cpu.memory.read_word(sp as u16 as usize), 1000);
		
		cpu.step();
		
		assert_eq!(cpu.get_rx(5), 1000);
	}
	
	#[test]
	fn pushall_popall() -> () {
		let mut cpu = Cpu::new_test();
		let sp = cpu.sp;
		assert_eq!(sp, 0xFDF0);
		cpu.add_opcode(Opcode::Pushall, 0, 0, 0);
		cpu.add_opcode(Opcode::Popall, 0, 0, 0);
		
		cpu.set_rx(0, 1);
		cpu.set_rx(1, 2);
		cpu.set_rx(2, 3);
		cpu.set_rx(3, 4);
		cpu.set_rx(4, 5);
		cpu.set_rx(5, 6);
		cpu.set_rx(6, 7);
		cpu.set_rx(7, 8);
		cpu.set_rx(8, 9);
		cpu.set_rx(9, 10);
		cpu.set_rx(10, 11);
		cpu.set_rx(11, 12);
		cpu.set_rx(12, 13);
		cpu.set_rx(13, 14);
		cpu.set_rx(14, 15);
		cpu.set_rx(15, 16);
		
		cpu.start_test(1);
		
		assert_eq!(cpu.sp, 0xFDF0 + 32);
		assert_eq!(cpu.memory.read_word(sp as usize), 1);
		assert_eq!(cpu.memory.read_word(0xFDF0 + 30), 16);
		
		cpu.set_rx(0, 10);
		cpu.set_rx(1, 20);
		cpu.set_rx(2, 30);
		cpu.set_rx(3, 40);
		cpu.set_rx(4, 50);
		cpu.set_rx(5, 60);
		cpu.set_rx(6, 70);
		cpu.set_rx(7, 80);
		cpu.set_rx(8, 90);
		cpu.set_rx(9, 100);
		cpu.set_rx(10, 110);
		cpu.set_rx(11, 120);
		cpu.set_rx(12, 130);
		cpu.set_rx(13, 140);
		cpu.set_rx(14, 150);
		cpu.set_rx(15, 160);
		
		cpu.step();
		assert_eq!(cpu.sp, 0xFDF0);
		
		assert_eq!(cpu.get_rx(0), 1);
		assert_eq!(cpu.get_rx(1), 2);
		assert_eq!(cpu.get_rx(2), 3);
		assert_eq!(cpu.get_rx(3), 4);
		assert_eq!(cpu.get_rx(4), 5);
		assert_eq!(cpu.get_rx(5), 6);
		assert_eq!(cpu.get_rx(6), 7);
		assert_eq!(cpu.get_rx(7), 8);
		assert_eq!(cpu.get_rx(8), 9);
		assert_eq!(cpu.get_rx(9), 10);
		assert_eq!(cpu.get_rx(10), 11);
		assert_eq!(cpu.get_rx(11), 12);
		assert_eq!(cpu.get_rx(12), 13);
		assert_eq!(cpu.get_rx(13), 14);
		assert_eq!(cpu.get_rx(14), 15);
		assert_eq!(cpu.get_rx(15), 16);
	}
	
	#[test]
	fn pushf_popf() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Pushf, 0, 0, 0);
		cpu.add_opcode(Opcode::Popf, 0, 0, 0);
		
		cpu.put_carry(true);
		
		cpu.start_test(1);
		assert_eq!(cpu.sp, 0xFDF2);
		
		cpu.put_carry(false);
		cpu.step();
		
		assert!(cpu.has_carry());
		assert_eq!(cpu.sp, 0xFDF0);
	}
	
	#[test]
	fn palette() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Pal, 0, 0x00, 0x0D);
		cpu.memory.write_byte(0xD00, 0xFF);//first color
		cpu.memory.write_byte(0xD01, 0xFF);
		cpu.memory.write_byte(0xD02, 0xFF);
		
		cpu.memory.write_byte(0xD03, 0xEE);//second color
		cpu.memory.write_byte(0xD04, 0xEE);
		cpu.memory.write_byte(0xD05, 0xEE);
		
		cpu.memory.write_byte(0xD06, 0xDD);//third color
		cpu.memory.write_byte(0xD07, 0xDD);
		cpu.memory.write_byte(0xD08, 0xDD);
		
		cpu.memory.write_byte(0xD09, 0xCC);//fourth color
		cpu.memory.write_byte(0xD0A, 0xCC);
		cpu.memory.write_byte(0xD0B, 0xCC);
		
		cpu.memory.write_byte(0xD0C, 0xBB);//fifth color
		cpu.memory.write_byte(0xD0D, 0xBB);
		cpu.memory.write_byte(0xD0E, 0xBB);
		
		cpu.memory.write_byte(0xD00 + 45, 0x11);//sixteenth color
		cpu.memory.write_byte(0xD00 + 46, 0x11);
		cpu.memory.write_byte(0xD00 + 47, 0x11);
		
		assert_eq!(cpu.graphics.palette[0], 0);
		assert_eq!(cpu.graphics.palette[1], 0);
		assert_eq!(cpu.graphics.palette[2], 0x888888u32);
		assert_eq!(cpu.graphics.palette[15], 0xFFFFFFu32);
		
		cpu.start_test(1);
		
		assert_eq!(cpu.graphics.palette[0], 0xFFFFFFu32);
		assert_eq!(cpu.graphics.palette[1], 0xEEEEEEu32);
		assert_eq!(cpu.graphics.palette[2], 0xDDDDDDu32);
		assert_eq!(cpu.graphics.palette[3], 0xCCCCCCu32);
		assert_eq!(cpu.graphics.palette[4], 0xBBBBBBu32);
		assert_eq!(cpu.graphics.palette[15], 0x111111u32);
	}
	
	#[test]
	fn noti() -> () {
		let mut cpu = stage_1op_test(Opcode::Noti, 5, 6, 0);
		assert_eq!(cpu.get_rx(5), !6);
	}
	
	#[test]
	fn negi() -> () {
		let mut cpu = stage_1op_test(Opcode::Negi, 5, 6, 0);
		assert_eq!(cpu.get_rx(5), -6);
	}
	
	#[test]
	fn not() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Not, 5, 0, 0);
		
		cpu.set_rx(5, 10);
		cpu.start_test(1);
		
		assert_eq!(cpu.get_rx(5), !10);
		assert!(!cpu.has_zero() && cpu.has_negative());
		
		cpu.set_rx(5, 0);
		cpu.start_test(1);
		
		assert_eq!(cpu.get_rx(5), !0);
		assert!(!cpu.has_zero() && cpu.has_negative());
		
		
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Not2, 0x05, 0, 0);
		
		cpu.set_rx(0, 10);
		cpu.start_test(1);
		
		assert_eq!(cpu.get_rx(5), !10);
		assert!(!cpu.has_zero() && cpu.has_negative());
		
		cpu.set_rx(0, !0);
		cpu.start_test(1);
		
		assert_eq!(cpu.get_rx(5), 0);
		assert!(cpu.has_zero() && !cpu.has_negative());
	}
	
	#[test]
	fn neg() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Neg, 5, 0, 0);
		
		cpu.set_rx(5, 10);
		cpu.start_test(1);
		
		assert_eq!(cpu.get_rx(5), -10);
		assert!(!cpu.has_zero() && cpu.has_negative());
		
		cpu.set_rx(5, 0);
		cpu.start_test(1);
		
		assert_eq!(cpu.get_rx(5), -0);
		assert!(cpu.has_zero() && !cpu.has_negative());
		
		
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Neg2, 0x05, 0, 0);
		
		cpu.set_rx(0, 10);
		cpu.start_test(1);
		
		assert_eq!(cpu.get_rx(5), -10);
		assert!(!cpu.has_zero() && cpu.has_negative());
		
		cpu.set_rx(0, -0);
		cpu.start_test(1);
		
		assert_eq!(cpu.get_rx(5), 0);
		assert!(cpu.has_zero() && !cpu.has_negative());
	}
	
	#[test]
	fn drw_noflip() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Spr, 0, 5, 10);
		cpu.add_opcode(Opcode::Drw, 0x65, 0x55, 0x55);
		cpu.set_rx(5, 10);
		cpu.set_rx(6, 20);
		cpu.flip(false, false);
		
		cpu.memory.write_byte(0x5555, 0xFE);
		cpu.memory.write_byte(0x5555 + 1, 0xDC);
		cpu.memory.write_byte(0x5555 + 2, 0xBA);
		cpu.memory.write_byte(0x5555 + 5, 0x98);
		cpu.memory.write_byte(0x5555 + 5 + 1, 0x76);
		cpu.memory.write_byte(0x5555 + 5 * 2, 0x54);
		
		cpu.start_test(2);
		
		assert_eq!(cpu.graphics.state.spritew, 5);
		assert_eq!(cpu.graphics.state.spriteh, 10);
		
		let screen = cpu.graphics.screen; //320x240
		
		assert_eq!(screen[320 * 20 + 10], 0xF);
		assert_eq!(screen[320 * 20 + 11], 0xE);
		assert_eq!(screen[320 * 20 + 12], 0xD);
		assert_eq!(screen[320 * 20 + 13], 0xC);
		assert_eq!(screen[320 * 20 + 14], 0xB);
		assert_eq!(screen[320 * 20 + 15], 0xA);
		
		assert_eq!(screen[320 * 21 + 10], 0x9);
		assert_eq!(screen[320 * 21 + 11], 0x8);
		assert_eq!(screen[320 * 21 + 12], 0x7);
		assert_eq!(screen[320 * 21 + 13], 0x6);
		
		assert_eq!(screen[320 * 22 + 10], 0x5);
		assert_eq!(screen[320 * 22 + 11], 0x4);
	}
	
	#[test]
	fn drw_noflipv_fliph() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Spr, 0, 5, 10);
		cpu.add_opcode(Opcode::Drw, 0x65, 0x55, 0x55);
		cpu.set_rx(5, 10);
		cpu.set_rx(6, 20);
		cpu.flip(true, false);
		
		cpu.memory.write_byte(0x5555, 0xFE);
		cpu.memory.write_byte(0x5555 + 1, 0xDC);
		cpu.memory.write_byte(0x5555 + 2, 0xBA);
		cpu.memory.write_byte(0x5555 + 5, 0x98);
		cpu.memory.write_byte(0x5555 + 5 + 1, 0x76);
		cpu.memory.write_byte(0x5555 + 5 * 2, 0x54);
		
		cpu.start_test(2);
		
		assert_eq!(cpu.graphics.state.spritew, 5);
		assert_eq!(cpu.graphics.state.spriteh, 10);
		
		let screen = cpu.graphics.screen; //320x240
		
		assert_eq!(screen[320 * 20 + 19], 0xF);
		assert_eq!(screen[320 * 20 + 18], 0xE);
		assert_eq!(screen[320 * 20 + 17], 0xD);
		assert_eq!(screen[320 * 20 + 16], 0xC);
		assert_eq!(screen[320 * 20 + 15], 0xB);
		assert_eq!(screen[320 * 20 + 14], 0xA);

		assert_eq!(screen[320 * 21 + 19], 0x9);
		assert_eq!(screen[320 * 21 + 18], 0x8);
		assert_eq!(screen[320 * 21 + 17], 0x7);
		assert_eq!(screen[320 * 21 + 16], 0x6);
		
		assert_eq!(screen[320 * 22 + 19], 0x5);
		assert_eq!(screen[320 * 22 + 18], 0x4);
	}
	
	#[test]
	fn drw_nofliph_flipv() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Spr, 0, 5, 10);
		cpu.add_opcode(Opcode::Drw, 0x65, 0x55, 0x55);
		cpu.set_rx(5, 10);
		cpu.set_rx(6, 20);
		cpu.flip(false, true);
		
		cpu.memory.write_byte(0x5555, 0xFE);
		cpu.memory.write_byte(0x5555 + 1, 0xDC);
		cpu.memory.write_byte(0x5555 + 2, 0xBA);
		cpu.memory.write_byte(0x5555 + 5, 0x98);
		cpu.memory.write_byte(0x5555 + 5 + 1, 0x76);
		cpu.memory.write_byte(0x5555 + 5 * 2, 0x54);
		
		cpu.start_test(2);
		
		assert_eq!(cpu.graphics.state.spritew, 5);
		assert_eq!(cpu.graphics.state.spriteh, 10);
		
		let screen = cpu.graphics.screen; //320x240
		
		assert_eq!(screen[320 * 29 + 10], 0xF);
		assert_eq!(screen[320 * 29 + 11], 0xE);
		assert_eq!(screen[320 * 29 + 12], 0xD);
		assert_eq!(screen[320 * 29 + 13], 0xC);
		assert_eq!(screen[320 * 29 + 14], 0xB);
		assert_eq!(screen[320 * 29 + 15], 0xA);

		assert_eq!(screen[320 * 28 + 10], 0x9);
		assert_eq!(screen[320 * 28 + 11], 0x8);
		assert_eq!(screen[320 * 28 + 12], 0x7);
		assert_eq!(screen[320 * 28 + 13], 0x6);
		
		assert_eq!(screen[320 * 27 + 10], 0x5);
		assert_eq!(screen[320 * 27 + 11], 0x4);
	}
	
	#[test]
	fn drw_flip() -> () {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(Opcode::Spr, 0, 5, 10);
		cpu.add_opcode(Opcode::Drw, 0x65, 0x55, 0x55);
		cpu.set_rx(5, 10);
		cpu.set_rx(6, 20);
		cpu.flip(true, true);
		
		cpu.memory.write_byte(0x5555, 0xFE);
		cpu.memory.write_byte(0x5555 + 1, 0xDC);
		cpu.memory.write_byte(0x5555 + 2, 0xBA);
		cpu.memory.write_byte(0x5555 + 5, 0x98);
		cpu.memory.write_byte(0x5555 + 5 + 1, 0x76);
		cpu.memory.write_byte(0x5555 + 5 * 2, 0x54);
		
		cpu.start_test(2);
		
		assert_eq!(cpu.graphics.state.spritew, 5);
		assert_eq!(cpu.graphics.state.spriteh, 10);
		
		let screen = cpu.graphics.screen; //320x240
		
		assert_eq!(screen[320 * 29 + 19], 0xF);
		assert_eq!(screen[320 * 29 + 18], 0xE);
		assert_eq!(screen[320 * 29 + 17], 0xD);
		assert_eq!(screen[320 * 29 + 16], 0xC);
		assert_eq!(screen[320 * 29 + 15], 0xB);
		assert_eq!(screen[320 * 29 + 14], 0xA);

		assert_eq!(screen[320 * 28 + 19], 0x9);
		assert_eq!(screen[320 * 28 + 18], 0x8);
		assert_eq!(screen[320 * 28 + 17], 0x7);
		assert_eq!(screen[320 * 28 + 16], 0x6);
		
		assert_eq!(screen[320 * 27 + 19], 0x5);
		assert_eq!(screen[320 * 27 + 18], 0x4);
	}
}
