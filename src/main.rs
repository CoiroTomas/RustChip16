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
