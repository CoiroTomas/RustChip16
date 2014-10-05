use std::io::File;
use opcode::{Opcode, to_opcode};

pub fn join_bytes(hh: i8, ll: i8) -> i16 {
	((((hh as u8) as u16) << 8) + (ll as u8) as u16) as i16
}

pub fn separate_word(word: i16) -> (i8, i8) {
	let word = word as u16;
	let hh = (word >> 8) as u8;
	let ll = (word & 0xff) as u8;
	(hh as i8, ll as i8)
}

pub fn separate_byte(byte: i8) -> (i8, i8) {
	let byte = byte as u8;
	let hh = (byte >> 4) as u8;
	let ll = byte & 0xf;
	(hh as i8, ll as i8)
}

enum Flags {
    Carry = 1 << 1,
   	Zero = 1 << 2,
   	Overflow = 1 << 6,
	Negative = 1 << 7,
}

struct Graphics {
    state: StateRegister,
	palette: [u64, ..16],
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
    pub pc: i16,
	pub sp: i16,
	rx: [i16, ..16],
	flags: i8,
	file: File,
	pub vblank: bool,
	graphics: Graphics,
	pub memory: Memory,
}

impl Memory {
    pub fn new() -> Memory {
	    Memory { memory: [0, ..65536] }
	}
	
	pub fn read_byte(self, dir: uint) -> i8 {
		self.memory[dir]
	}
	
	pub fn write_byte(&mut self, dir: uint, value: i8) -> () {
		self.memory[dir] = value;
	}
	
	pub fn read_word(self, dir: uint) -> i16 {
		let ll = self.memory[dir];
		let hh = self.memory[dir + 1];
		join_bytes(hh, ll)
	}
	
	pub fn write_word(&mut self, dir: uint, value: i16) -> () {
		let (hh, ll) = separate_word(value);
		self.memory[dir] = ll;
		self.memory[dir + 1] = hh;
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
	
	pub fn set_bg(&mut self, byte: u8) -> () {
		self.state.bg = byte;
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
        let cpu = Cpu {pc: 0, sp: 0, rx: [0, ..16], flags: 0, file: file,
	    	vblank: false, graphics: Graphics::new(), memory: Memory::new(),};
	    cpu
    }
	
	pub fn clear_fg_bg(&mut self) {
		self.graphics.clear()
	}
	
	pub fn set_bg(&mut self, byte: u8) -> () {
		self.graphics.set_bg(byte);
	}
	
	pub fn set_spr_wh(&mut self, ll: u8, hh: u8) -> () {
		self.graphics.state.spritew = ll;
		self.graphics.state.spriteh = hh;
	}
	
	pub fn get_rx(&mut self, rx: i8) -> i16 {
		self.rx[rx as uint]
	}
	
	pub fn set_rx(&mut self, rx: i8, value: i16) -> () {
		self.rx[rx as uint] = value;
	}
	
	pub fn pop_stack(&mut self) -> i16 {
		let word = self.memory.read_word(self.sp as uint);
		self.sp = self.sp - 2;
		word
	}
	
	pub fn push_stack(&mut self, word: i16) -> () {
		self.memory.write_word(self.sp as uint, word);
		self.sp = self.sp + 2;
	}
	
	pub fn flip(&mut self, hor: bool, ver: bool) -> () {
		self.graphics.state.hflip = hor;
		self.graphics.state.vflip = ver;
	}
	
	pub fn clear_flags(&mut self) -> () {
		self.flags = 0;
	}
	
	pub fn has_carry(&self) -> bool {
		(Carry as i8 & self.flags) != 0
	}
	
	pub fn has_zero(&self) -> bool {
		(Zero as i8 & self.flags) != 0
	}
	
	pub fn has_overflow(&self) -> bool {
		(Overflow as i8 & self.flags) != 0
	}
	
	pub fn has_negative(&self) -> bool {
		(Negative as i8 & self.flags) != 0
	}
	
	pub fn put_carry(&mut self, new_state: bool) -> () {
		if new_state {
			self.flags = Carry as i8 | self.flags
		} else {
			self.flags = Carry as i8 ^ self.flags
		}
	}
	
	pub fn put_zero(&mut self, new_state: bool) -> () {
		if new_state {
			self.flags = Zero as i8 | self.flags
		} else {
			self.flags = Zero as i8 ^ self.flags
		}
	}
	
	pub fn put_overflow(&mut self, new_state: bool) -> () {
		if new_state {
			self.flags = Overflow as i8 | self.flags
		} else {
			self.flags = Overflow as i8 ^ self.flags
		}
	}
	
	pub fn put_negative(&mut self, new_state: bool) -> () {
		if new_state {
			self.flags = Negative as i8 | self.flags
		} else {
			self.flags = Negative as i8 ^ self.flags
		}
	}
	
	pub fn check_flags(&self, index: i8) -> bool {
		match index {
			0 => self.has_zero(),
			1 => !self.has_zero(),
			2 => self.has_negative(),
			3 => !self.has_negative(),
			4 => !self.has_negative() && !self.has_zero(),
			5 => self.has_overflow(),
			6 => !self.has_overflow(),
			7 => !self.has_carry() && !self.has_zero(),
			8 => !self.has_carry(),
			9 => self.has_carry(),
			0xA => self.has_carry() || self.has_zero(),
			0xB => (self.has_overflow() == self.has_negative()) && !self.has_zero(),
			0xC => (self.has_overflow() == self.has_negative()),
			0xD => (self.has_overflow() != self.has_negative()),
			0xE => (self.has_overflow() != self.has_negative()) || !self.has_zero(),
			_ => fail!(),
		}
	}
	
	pub fn step(&mut self) -> () {
		let opcode = to_opcode(self.memory.read_byte(self.pc as uint));
		let byte1 = self.memory.read_byte((self.pc + 1) as uint);
		let byte2 = self.memory.read_byte((self.pc + 2) as uint);
		let byte3 = self.memory.read_byte((self.pc + 3) as uint);
		self.pc = self.pc + 4;
		opcode.execute(self, byte1, byte2, byte3);
	}
	
	pub fn start_program(&mut self) -> () {
	    //TODO initialize the necessary variables and then start the execution loop
	}
}