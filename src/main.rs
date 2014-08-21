mod cpu;
mod opcode;

fn main() {
    let mut my_cpu = cpu::Cpu::new(Path::new("test.willfail",));
	my_cpu.execute(opcode::Nop, 1, 2, 3);
}