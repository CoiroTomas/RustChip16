use cpu::Cpu;
mod cpu;
mod opcode;


fn main() {
    let mut cpu = Cpu::new(Path::new("C:\\tomas\\GIT\\Rust16\\src\\main.rs",));
	cpu.start_program();
}