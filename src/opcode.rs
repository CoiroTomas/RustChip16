use cpu::Cpu;

pub enum Opcode {
    Nop,
}

impl Opcode {
	pub fn execute(&self, cpu: &mut Cpu, byte1: i8, byte2: i8, byte3: i8) {
	    match *self {
		    Nop => {},
		}
	}
}