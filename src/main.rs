use cpu::Cpu;
mod cpu;
mod opcode;


fn main() {
    let mut my_cpu = Cpu::new(Path::new("test.willfail",));
	my_cpu.start_program();
	my_cpu.step();
}