use cpu::Cpu;
use std::env;
mod cpu;
mod opcode;
mod loading;

fn main() {
	let mut args = env::args();
	args.next();
	let path = args.next().unwrap().into_string().unwrap();
	let mut cpu = Cpu::new(Path::new(path));
	cpu.start_program();
}
