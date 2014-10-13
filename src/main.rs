use cpu::Cpu;
mod cpu;
mod opcode;


fn main() {
    let mut my_cpu = Cpu::new(Path::new("C:\\tomas\\GIT\\Rust16\\src\\main.rs",));
	my_cpu.start_program();
	my_cpu.step();
}