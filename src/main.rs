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
	use cpu::Cpu;
	use opcode::Opcode;
	
	fn stage_1op_test(op: Opcode, byte1: i8, byte2: i8, byte3: i8, steps: i8) -> Cpu {
		let mut cpu = Cpu::new_test();
		cpu.add_opcode(op, byte1, byte2, byte3);
		cpu.start_test(steps);
		cpu
	}

	#[test]
	fn new_cpu() -> () {
		let cpu = stage_1op_test(Opcode::Nop, 0, 0, 0, 1);
		assert_eq!(cpu.pc, 4);
	}

	#[test]
	fn jmp() -> () {
		let cpu = stage_1op_test(Opcode::Jmp, 0, 0x10, 0x20, 1);
		assert_eq!(cpu.pc, 0x2010);
	}
	
	#[test]
	fn flip() -> () {
		let cpu0 = stage_1op_test(Opcode::Flip, 0, 0, 0, 1);
		let cpu1 = stage_1op_test(Opcode::Flip, 0, 0, 1, 1);
		let cpu2 = stage_1op_test(Opcode::Flip, 0, 0, 2, 1);
		let cpu3 = stage_1op_test(Opcode::Flip, 0, 0, 3, 1);
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
}
