extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate sdl2_window;
use std::cell::RefCell;
use opengl_graphics::OpenGL;
use sdl2_window::Sdl2Window as Window;
use cpu::Cpu;
use std::env;
mod cpu;
mod opcode;
mod loading;

fn main() {
	let mut args = env::args();
	let (min, _) = args.size_hint();
	if min < 2 {
		println!("No ROM specified");
		return;
	}
	args.next();
	let path = args.next().unwrap();
	let window = RefCell::new(Window::new(
		OpenGL::_3_2,
		piston::window::WindowSettings {
			title: "RustChip16".to_string(),
			samples: 0,
			size: [320, 240],
			fullscreen: false,
			exit_on_esc: true,
		}
	));
	let mut cpu = Cpu::new(Path::new(path));
	cpu.start_program(window);
}

mod tests {
	#![allow(dead_code)]
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
		
		if !cpu.check_flags(10) {
			panic!("{}", 10);
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
		cpu.start_test(2);
		
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
		assert!(cpu2.has_carry());
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
}
