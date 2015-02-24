use std::old_io::{File, Open, Read};
use std::cell::RefCell;
use std::mem::transmute;
use opcode::{to_opcode};
use opcode;
use piston;
use piston::event::{
	events,
	RenderArgs,
	RenderEvent,
	UpdateArgs,
	UpdateEvent,
};
use sdl2_window::Sdl2Window as Window;
use opengl_graphics::{
	Gl,
	OpenGL,
};
use graphics;
use loading::{load_bin, load_c16};

pub fn join_bytes(ll: i8, hh: i8) -> i16 {
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

enum Flag {
	Carry = 1 << 1,
   	Zero = 1 << 2,
   	Overflow = 1 << 6,
	Negative = 1 << 7,
}

struct Graphics {
	state: StateRegister,
	palette: [u32; 16], //capacity == 16
	screen: [u8 ; 76800], //capacity == 320x240
	size: i32,
	gl: Gl,
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
	memory: [i8; 65536], //capacity == 65536
}
	
pub struct Cpu {
	pub pc: u16,
	pub sp: i16,
	rx: [i16; 16], //capacity == 16
	flags: i8,
	pub vblank: bool,
	graphics: Graphics,
	pub memory: Memory,
}

impl Memory {
	pub fn new() -> Memory {
	    Memory { memory: [0; 65536] }
	}
	
	pub fn read_byte(&mut self, dir: usize) -> i8 {
		self.memory[dir]
	}
	
	pub fn write_byte(&mut self, dir: usize, value: i8) -> () {
		self.memory[dir] = value;
	}
	
	pub fn read_word(&mut self, dir: usize) -> i16 {
		let ll = self.memory[dir];
		let hh = self.memory[dir + 1];
		join_bytes(hh, ll)
	}
	
	pub fn write_word(&mut self, dir: usize, value: i16) -> () {
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
}

impl Graphics {
	pub fn new() -> Graphics {
		Graphics {
			state: StateRegister::new(),
			palette: [0x000000, 0x000000, 0x888888, 0xBF3932, 0xDE7AAE, 0x4C3D21, 0x905F25, 0xE49452,
				0xEAD979, 0x537A3B, 0xABD54A, 0x252E38, 0x00467F, 0x68ABCC, 0xBCDEE4, 0xFFFFFF],
			screen: [0; 76800],
			size: 1,
			gl: Gl::new(OpenGL::_3_2),
		}
	}
	
	fn clear(self: &mut Graphics) {
	    self.state.clear();
	}
	
	pub fn set_bg(&mut self, byte: u8) -> () {
		self.state.bg = byte;
	}

	pub fn drw(&mut self, mem: &mut Memory, spr_x: i16, spr_y: i16, spr_address: i16) -> bool {
		if spr_x > 319
			|| spr_y > 239
			|| self.state.spritew == 0
			|| self.state.spriteh == 0
			|| (spr_x + (self.state.spritew * 2) as i16) < 0
			|| (spr_y + self.state.spriteh as i16) < 0
		{
			return false;
		}

		let x_start: i16;
		let  x_end: i16;
		if self.state.hflip {
			x_start = (self.state.spritew  - 1) as i16;
			x_end = -2;
		} else {
			x_start = 0;
			x_end = self.state.spritew as i16;
		}

		let y_start: i16;
		let y_end: i16;
		if self.state.vflip {
			y_start = (self.state.spriteh - 1) as i16;
			y_end = -1;
		} else {
			y_start = 0;
			y_end = self.state.spriteh as i16;
		}

		let mut hit: u64 = 0;
		let mut j: u16 = 0;
		let mut i: u16 = 0;
		for y in y_start..y_end {
			i = 0;
			for x in x_start..x_end {
				if (i + x as u16) < 0
					|| (i + x as u16) > 319
					|| (j + y as u16) < 0
					|| (j + y as u16) > 239
				{
					continue;
				}

				let pixels = mem.read_byte((y as u16 * self.state.spritew as u16 + x as u16) as usize);
				let (hh_pixel, ll_pixel) = separate_byte(pixels);
				let odd_pixel: u8;
				let even_pixel: u8;
				if self.state.hflip {
					odd_pixel = ll_pixel as u8;
					even_pixel = hh_pixel as u8;
				} else {
					even_pixel = ll_pixel as u8;
					odd_pixel = hh_pixel as u8;
				}

				if (even_pixel != 0) {
					hit |= self.screen[(320*(spr_y as u16 + j) + spr_x as u16 + i) as usize] as u64;
					self.screen[(320*(spr_y as u16 + j) + spr_x as u16 + i*2) as usize] = even_pixel;
				}

				if (odd_pixel != 0) {
					hit |= self.screen[(320*(spr_y as u16 + j) + spr_x as u16 + i + 1) as usize] as u64;
					self.screen[(320*(spr_y as u16 + j) + spr_x as u16 + i*2 + 1) as usize] = odd_pixel;
				}
				i += 2;
			}
			j += 1;
		}

		hit != 0
	}

	pub fn draw_screen(&mut self, _: &mut Window, args: &RenderArgs) -> () {
		let context = &graphics::Context::abs(args.width as f64, args.height as f64);
		let mut colours: Vec<[f32;4]> = Vec::with_capacity(16);
		for p in self.palette.iter() {
			let mut v: Vec<f32> = vec![];
			let v: [f32; 4] = [
				(((p &0xFF0000)>>4) as f32) / 255.0 as f32,
				(((p & 0xFF00) >> 2) as f32) / 255.0,
				((p & 0xFF) as f32) / 255.0,
				1.0,];
			colours.push(v);
		}

		let screen = self.screen.iter();
		let mut i: u32 = 0;
		self.gl.draw([0, 0, args.width as i32, args.height as i32], |_, gl| {
			graphics::clear(colours[0], gl);
			for pixel in screen {
				let y: f64 = ((i / 320) as f64 / args.height as f64);
				let x: f64 = ((i & 320) as f64 / args.width as f64);
				graphics::rectangle(
					colours[*pixel as usize],
					graphics::rectangle::square(x, y, 1.0/255.0),
					context,
					gl);
			}
		})
	}
}

impl Cpu {
	pub fn new(file_path: Path) -> Cpu {
		let mut file = match File::open_mode(&file_path, Open, Read) {
		    Ok(file) => file,
   		    Err(e) => panic!("{} {}", e.desc, file_path.display()),
		};
		
		let mut cpu = Cpu {pc: 0, sp: 0, rx: [0; 16], flags: 0,
			vblank: false, graphics: Graphics::new(),
			memory: Memory::new(),
		};
		if file_path.extension_str() == Some("bin") {
			load_bin(&mut file, &mut cpu)
		} else if file_path.extension_str() == Some("c16") {
			load_c16(&mut file, &mut cpu)
		} else {
			panic!("The file is not a valid extension")
		}
		cpu
	}
	
	pub fn load_pal(&mut self, dir: i16) -> () {
		for i in 0..15 {
			let dir = dir as usize;
			let high: u32 = (self.memory.read_byte(dir + (i * 3)) as i32 as u32) << 16;
			let middle: u32 = (self.memory.read_byte(dir + (i * 3) + 1) as i32 as u32) << 8;
			let low: u32 = self.memory.read_byte(dir + (i * 3) + 2) as i32 as u32;
			self.graphics.palette[i] = high + middle + low;
		}
	}

	pub fn drw(&mut self, sprite_x: i16, sprite_y: i16, sprite_address: i16) -> () {
		let carry: bool;
		{
			let ref mut memory = self.memory;
			carry = self.graphics.drw(memory, sprite_x, sprite_y, sprite_address);
		}
		self.put_carry(carry);
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
		self.rx[rx as usize]
	}
	
	pub fn set_rx(&mut self, rx: i8, value: i16) -> () {
		self.rx[rx as usize] = value;
	}
	
	pub fn pop_stack(&mut self) -> i16 {
		self.sp = self.sp - 2;
		let word = self.memory.read_word(self.sp as usize);
		word
	}
	
	pub fn push_stack(&mut self, word: i16) -> () {
		self.memory.write_word(self.sp as usize, word);
		self.sp = self.sp + 2;
	}
	
	pub fn pushall(&mut self) -> () {
		let vec = self.rx;
		for rx in vec.iter() {
			self.push_stack(*rx);
		}
	}
	
	pub fn popall(&mut self) -> () {
		for i in range(15i8, 1i8) {
			let val = self.pop_stack();
			self.set_rx(i, val);
		}
	}
	
	pub fn pushf(&mut self) -> () {
		let value = ((self.flags as u8) as u16) as i16;
		self.push_stack(value);
	}
	
	pub fn popf(&mut self) -> () {
		let value = self.pop_stack();
		self.flags = ((value as u16) as u8) as i8;
	}
	
	pub fn flip(&mut self, hor: bool, ver: bool) -> () {
		self.graphics.state.hflip = hor;
		self.graphics.state.vflip = ver;
	}
	
	pub fn has_carry(&self) -> bool {
		(Flag::Carry as i8 & self.flags) != 0
	}
	
	pub fn has_zero(&self) -> bool {
		(Flag::Zero as i8 & self.flags) != 0
	}
	
	pub fn has_overflow(&self) -> bool {
		(Flag::Overflow as i8 & self.flags) != 0
	}
	
	pub fn has_negative(&self) -> bool {
		(Flag::Negative as i8 & self.flags) != 0
	}
	
	pub fn put_carry(&mut self, new_state: bool) -> () {
		if new_state {
			self.flags = Flag::Carry as i8 | self.flags
		} else {
			self.flags = Flag::Carry as i8 ^ self.flags
		}
	}
	
	pub fn put_zero(&mut self, new_state: bool) -> () {
		if new_state {
			self.flags = Flag::Zero as i8 | self.flags
		} else {
			self.flags = Flag::Zero as i8 ^ self.flags
		}
	}
	
	pub fn put_overflow(&mut self, new_state: bool) -> () {
		if new_state {
			self.flags = Flag::Overflow as i8 | self.flags
		} else {
			self.flags = Flag::Overflow as i8 ^ self.flags
		}
	}
	
	pub fn put_negative(&mut self, new_state: bool) -> () {
		if new_state {
			self.flags = Flag::Negative as i8 | self.flags
		} else {
			self.flags = Flag::Negative as i8 ^ self.flags
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
			_ => panic!("Failed to find flag: {}", index),
		}
	}
	
	pub fn step(&mut self) -> () {
		if self.pc >= 0xFDF0 {
			panic!("The program accessed the stack as instructions");
		}
		let op_n = self.memory.read_byte(self.pc as usize);
		let op: opcode::Opcode = to_opcode(op_n);
		let byte1 = self.memory.read_byte((self.pc + 1) as usize);
		let byte2 = self.memory.read_byte((self.pc + 2) as usize);
		let byte3 = self.memory.read_byte((self.pc + 3) as usize);
		self.pc = self.pc + 4;
		op.execute(self, byte1, byte2, byte3);
	}

	pub fn start_program(&mut self, window: RefCell<Window>) -> () {		
		let mut update_delta: f64 = 0.0;
		let mut render_delta: f64 = 0.0;
		for e in events(&window) {
			if let Some(u) = e.update_args() {
				let mut dt = u.dt;
				render_delta += dt;
				while(dt > 0.001) {
					dt -= 0.001;
					update_delta += 0.001;
					self.vblank = update_delta > (1.0 / 60.0);
					self.step();
					if self.vblank {
						self.vblank = false;
						update_delta -= (1.0 / 60.0);
					};
				}
			}

			if let Some(r) = e.render_args() {
				if render_delta > (1.0 / 60.0) {
					render_delta -= (1.0 / 60.0);
					self.graphics.draw_screen(&mut window.borrow_mut(), &r);
				}
			}
		}
	}
}
