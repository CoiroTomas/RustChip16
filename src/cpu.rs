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
	
pub struct Cpu {
    pc: i16,
	sp: i16,
	r: [i16, ..16],
	flags: i8,
	file: File,
	vblank: bool,
	graphics: Graphics,
	memory: [i8, ..65536],
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
	    	vblank: false, graphics: Graphics::new(),
			memory: [0, ..65536],};
	    cpu
    }
	
	fn execute(&mut self, opcode: i8, first_byte: i8, second_byte: i8, third_byte: i8) {
	    match opcode {
			0x00 => self.nop(),
			0x01 => self.cls(),
			0x02 => self.vblnk(),
			0x03 => self.bgc(second_byte),
			0x04 => self.spr(second_byte, third_byte),
			0x05 => self.drw(first_byte, second_byte, third_byte),
			0x06 => self.drwsprite(first_byte, second_byte),
			0x07 => self.rnd(first_byte, second_byte, third_byte),
			0x08 => self.flip(third_byte),
			0x09 => self.snd0(),
			0x0A => self.snd1(second_byte, third_byte),
			0x0B => self.snd2(second_byte, third_byte),
			0x0C => self.snd3(second_byte, third_byte),
			0x0D => self.snp(first_byte, second_byte, third_byte),
			0x0E => self.sng(first_byte, second_byte, third_byte),
			0x10 => self.jmp(second_byte, third_byte),
			0x12 => self.jx(first_byte, second_byte, third_byte),
			0x13 => self.jme(first_byte, second_byte, third_byte),
			0x14 => self.call(second_byte, third_byte),
			0x15 => self.ret(),
			0x16 => self.jmp(first_byte),
			0x17 => self.cx(first_byte, second_byte, third_byte),
			0x18 => self.callr(first_byte),
			0x20 => self.ldir(first_byte, second_byte, third_byte),
			0x21 => self.ldisp(second_byte, third_byte),
			0x22 => self.ldm(first_byte, second_byte, third_byte),
			0x23 => self.ldmr(first_byte),
			0x24 => self.mov(first_byte),
			0x30 => self.stm(first_byte, second_byte, third_byte),
			0x31 => self.stmr(first_byte),
			//will continue
		}
	}
}