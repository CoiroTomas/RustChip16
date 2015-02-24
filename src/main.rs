extern crate piston;
extern crate graphics;
extern crate gfx;
extern crate gfx_graphics;
extern crate sdl2_window;
extern crate sdl2;
use std::mem::transmute;
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
	let mut cpu = Cpu::new(Path::new(path));
	cpu.start_program();
}
