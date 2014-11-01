use cpu::Cpu;
use std::os;
mod cpu;
mod opcode;


fn main() {
    let mut cpu = Cpu::new(Path::new(os::args()[1].as_slice()));
	cpu.start_program();
}