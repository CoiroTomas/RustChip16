use cpu;

trait Opcode {
	fn execute(cpu: &mut cpu::Cpu, byte1: i8, byte2: i8, byte3: i8) -> ();
}

//The struct will contain its own opcode definitions,
//then a list of Opcode's will serve as the instruction list
struct Nop;
impl Opcode for Nop {
    #[allow(dead_code)]
	fn execute(cpu: &mut cpu::Cpu, byte1: i8, byte2: i8, byte3: i8) -> () {
	    ()
	}
}