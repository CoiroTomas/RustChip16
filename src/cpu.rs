use std::io::File;
use opcode::Opcode;

//branching condition functions
fn zero(flag_byte: i8) -> bool {
	(flag_byte & Zero as i8) == Zero as i8
}

fn non_zero(flag_byte: i8) -> bool {
	!zero(flag_byte)
}

fn negative(flag_byte: i8) -> bool {
	(flag_byte & Negative as i8) == Negative as i8
}

fn non_negative(flag_byte: i8) -> bool {
	!negative(flag_byte)
}

fn positive(flag_byte: i8) -> bool {
	non_zero(flag_byte) && non_negative(flag_byte)
}

fn overflow(flag_byte: i8) -> bool {
	(flag_byte & Overflow as i8) == Overflow as i8
}

fn non_overflow(flag_byte: i8) -> bool {
	!overflow(flag_byte)
}

fn above(flag_byte: i8) -> bool {
	above_equal(flag_byte) && non_zero(flag_byte)
}

fn above_equal(flag_byte: i8) -> bool {
	!below(flag_byte) 
}

fn below(flag_byte: i8) -> bool {
	(flag_byte & Carry as i8) == Carry as i8
}

fn below_equal(flag_byte: i8) -> bool {
	below(flag_byte) && zero(flag_byte)
}

fn signed_greater_than(flag_byte: i8) -> bool {
	signed_greater_than_equal(flag_byte) && non_zero(flag_byte)
}

fn signed_greater_than_equal(flag_byte: i8) -> bool {
	((flag_byte & Overflow as i8) == Overflow as i8)
	== ((flag_byte & Negative as i8) == Negative as i8)
}

fn signed_less_than(flag_byte: i8) -> bool {
	!signed_greater_than_equal(flag_byte)
}

fn signed_less_than_equal(flag_byte: i8) -> bool {
	signed_less_than(flag_byte) && zero(flag_byte)
}
//branching functions done

enum Flags {
    Carry = 2,
   	Zero = 4,
   	Overflow = 64,
	Negative = 128,
}

struct Graphics {
    state: StateRegister,
	palette: [u8, ..16],
	screen: [[u8, ..320], ..240],
}
	
struct StateRegister {
    bg: u8,
	fg: u8,
	spritew: u8,
	spriteh: u8,
	hflip: bool,
	vflip: bool,
}

struct Memory {
    memory: [i8, ..65536],
}
	
pub struct Cpu {
    pc: i16,
	sp: i16,
	r: [i16, ..16],
	flags: i8,
	file: File,
	vblank: bool,
	graphics: Graphics,
	memory: Memory,
}

impl Memory {
    pub fn new() -> Memory {
	    Memory { memory: [0, ..65536] }
	}
}

impl StateRegister {
    pub fn new() -> StateRegister{
	    StateRegister { bg: 0, fg: 0, spritew: 0, spriteh: 0, hflip: false, vflip: false,}
	}
	
	fn clear(self: &mut StateRegister) {
		self.fg = 0;
		self.bg = 0;
	}
	
	fn set_spr(self: &mut StateRegister, ll: i8, hh: i8) {
		self.spritew = ll as u8;
		self.spriteh = hh as u8;
	}
}

impl Graphics {
    pub fn new() -> Graphics {
	    Graphics {
		    state: StateRegister::new(),
		    palette: [0x000000, 0x000000, 0x888888, 0xBF3932, 0xDE7AAE, 0x4C3D21, 0x905F25, 0xE49452,
		        0xEAD979, 0x537A3B, 0xABD54A, 0x252E38, 0x00467F, 0x68ABCC, 0xBCDEE4, 0xFFFFFF],
		    screen: [[0, ..320], ..240],
		}
	}
	
	fn clear(self: &mut Graphics) {
	    self.state.clear();
	}
	
	fn spr(self: &mut Graphics, ll: i8, hh: i8) {
		self.state.set_spr(ll, hh);
	}
}
	
impl Cpu {
    pub fn new(file_path: Path) -> Cpu {
    	let file = match File::open(&file_path) {
   		    Err(why) => fail!("{} {}",why.desc, file_path.display()),
		    Ok(file) => file,
	    };
        let cpu = Cpu {pc: 0, sp: 0, r: [0, ..16], flags: 0, file: file,
	    	vblank: false, graphics: Graphics::new(), memory: Memory::new(),};
	    cpu
    }
	
	pub fn start_program(&mut self) -> () {
	    //TODO initialize the necessary variables and then start the execution loop
	}
}