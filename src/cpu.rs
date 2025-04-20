use image;
use std::fs::File;
use opcode::{to_opcode, join_bytes, separate_byte, separate_word};
use opcode;
use piston_window::*;
use std::path::Path;
use loading::{load_bin, load_c16};

enum Flag {
	Carry = 1 << 1,
   	Zero = 1 << 2,
   	Overflow = 1 << 6,
	Negative = 1 << 7,
}

enum Pad {
	Up = 1,
	Down = 2, 
	Left = 4,
	Right = 8,
	Select = 16,
	Start = 32,
	A = 64,
	B = 128,
}

pub struct Chip16Graphics {
	pub state: StateRegister,
	pub palette: [u32; 16],
	pub screen: [u8 ; 76800], //320x240
	size: u32,
}
	
pub struct StateRegister {
	bg: u8,
	pub spritew: u8,
	pub spriteh: u8,
	pub hflip: bool,
	pub vflip: bool,
}

pub struct Memory {
	memory: [i8; 65536],
}
	
pub struct Cpu {
	pub pc: u16,
	pub sp: u16,
	rx: [i16; 16],
	flags: i8,
	pub vblank: bool,
	pub graphics: Chip16Graphics,
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
		join_bytes(ll, hh)
	}
	
	pub fn write_word(&mut self, dir: usize, value: i16) -> () {
		let (hh, ll) = separate_word(value);
		self.memory[dir] = ll;
		self.memory[dir + 1] = hh;
	}
}

impl StateRegister {
	pub fn new() -> StateRegister{
	    StateRegister { bg: 0, spritew: 0, spriteh: 0, hflip: false, vflip: false,}
	}
	
	fn clear(self: &mut StateRegister) {
		self.bg = 0;
	}
}

impl Chip16Graphics {
	pub fn new(multiplier: u32) -> Chip16Graphics {
		Chip16Graphics {
			state: StateRegister::new(),
			palette: [0x000000, 0x000000, 0x888888, 0xBF3932, 0xDE7AAE, 0x4C3D21, 0x905F25, 0xE49452,
				0xEAD979, 0x537A3B, 0xABD54A, 0x252E38, 0x00467F, 0x68ABCC, 0xBCDEE4, 0xFFFFFF],
			screen: [0; 76800],
			size: multiplier,
		}
	}
	
	#[allow(dead_code)]
	pub fn new_test() -> Chip16Graphics {
		Chip16Graphics {
			state: StateRegister::new(),
			palette: [0x000000, 0x000000, 0x888888, 0xBF3932, 0xDE7AAE, 0x4C3D21, 0x905F25, 0xE49452,
				0xEAD979, 0x537A3B, 0xABD54A, 0x252E38, 0x00467F, 0x68ABCC, 0xBCDEE4, 0xFFFFFF],
			screen: [0; 76800],
			size: 1,
		}
	}
	
	fn clear(self: &mut Chip16Graphics) {
	    self.state.clear();
		for pixel in self.screen.iter_mut() {
			*pixel = 0;
		}
	}
	
	pub fn set_bg(&mut self, byte: u8) -> () {
		self.state.bg = byte;
	}

	pub fn drw(&mut self, mem: &mut Memory, spr_x: i16, spr_y: i16, spr_address: i16) -> bool {
		let spritew = self.state.spritew as u32 as i32;
		let spriteh = self.state.spriteh as u32 as i32;
		let spr_x = spr_x as i32;
		let spr_y = spr_y as i32;
		if spr_x > 319  //If nothing is to be drawn, return
			|| spr_y > 239
			|| spritew == 0
			|| spriteh == 0
			|| (spr_x + spritew * 2) < 0
			|| (spr_y + spriteh) < 0
		{
			return false;
		}

		let mut hit: u8 = 0;
		for y in 0i32..spriteh {
			for x in 0i32..spritew {
				let mut x = x;
				let mut y = y;
				
				x = x * 2;
				if (spr_x + x) < 0 //If outside boundaries, continue to next iteration
					|| (spr_x + x) > 319
					|| (spr_y + y) < 0
					|| (spr_y + y) > 239
				{
					continue;
				}

				let pixels = mem.read_byte((y as u16 * spritew as u16
					+ x as u16 / 2
					+ spr_address as u16)
				as usize);
				
				let (hh_pixel, ll_pixel) = separate_byte(pixels);
				let odd_pixel: u8;
				let even_pixel: u8;
				if self.state.hflip { //Flips in case of a horizontal flip
					odd_pixel = hh_pixel as u8;
					even_pixel = ll_pixel as u8;
				} else {
					even_pixel = hh_pixel as u8;
					odd_pixel = ll_pixel as u8;
				}
				
				if self.state.hflip { //When a flip is active, move the byte to the other side
					x = ((spritew - 1) - x / 2) * 2;
				}
				
				if self.state.vflip {
					y = (spriteh - 1) - y;
				}
				
				let x = x as u32;
				let y = y as u32;

				let screen_pos = (320 * (spr_y as u32 + y) + spr_x as u32 + x) as usize;
				if even_pixel != 0 && screen_pos < 76800 {  //If the pixel is transparent, doesn't draw
					hit |= self.screen[screen_pos];
					self.screen[screen_pos] = even_pixel;
				}

				if odd_pixel != 0 && screen_pos + 1 < 76800 {
					hit |= self.screen[screen_pos + 1];
					self.screen[screen_pos + 1] = odd_pixel;
				}
			}
		}
		hit != 0 //If different than zero, put carry
	}

	pub fn draw_screen(&mut self, window: &mut PistonWindow, _: &RenderArgs, input: &Event) -> () {
		let mut colours: Vec<[u8;4]> = Vec::with_capacity(16);
		for p in self.palette.iter() {
			let v: [u8; 4] = [ //Transforms the palette into something Piston accepts
				((p & 0xFF0000) >> 16) as u8,
				((p & 0x00FF00) >> 8) as u8,
				(p & 0x0000FF) as u8,
				255,];
			colours.push(v);
		}

		let screen = self.screen.iter();
		let size = self.size as u32;
		
		let mut buffer_image = image::ImageBuffer::new(320 * size, 240 * size);
		
		for (pixel, i) in screen.zip(0..76800u32) {
			let y: u32 = (i / 320) as u32;
			let x: u32 = (i % 320) as u32;
			for j in 0..size {
				for k in 0..size {
					buffer_image.put_pixel(size * x + j, size * y + k,
						image::Rgba(if *pixel == 0 {
							colours[self.state.bg as usize]
						} else {
							colours[*pixel as usize]
					}));
				}
			}
		}
		
		let texture = Texture::from_image(
			&mut window.create_texture_context(),
			&buffer_image,
			&TextureSettings::new(),
		).unwrap();
		
		window.draw_2d(input, |_c, g, _| {
			image(&texture, _c.transform, g);
		});
		
	}
}

impl Cpu {
	pub fn new(file_path: &Path, multiplier: u32) -> Cpu {
		let mut file = match File::open(&file_path) {
		    Ok(file) => file,
   		    Err(e) => panic!("{} {}", e.to_string(), file_path.display()),
		};
		
		let mut cpu = Cpu {pc: 0, sp: 0xFDF0, rx: [0; 16], flags: 0,
			vblank: false, graphics: Chip16Graphics::new(multiplier),
			memory: Memory::new(),
		};
		let ext = file_path.extension().unwrap();
		match ext.to_str() {
			Some("bin") => load_bin(&mut file, &mut cpu),
			Some("c16") => load_c16(&mut file, &mut cpu),
			_ => panic!("The file is not a valid extension"),
		}
		cpu
	}
	
	#[allow(dead_code)]
	pub fn new_test() -> Cpu {// "Virgin" cpu for testing
		Cpu {pc: 0, sp: 0xFDF0, rx: [0; 16], flags: 0,
			vblank: false, graphics: Chip16Graphics::new_test(),
			memory: Memory::new(),
		}
	}
	
	#[allow(dead_code)]
	pub fn add_opcode(&mut self, op: opcode::Opcode, byte1: i8, byte2: i8, byte3: i8) -> () {
		let pc = self.pc as usize; //Ability to add instructions for testing
		self.memory.write_byte(pc, op as i8);
		self.memory.write_byte(pc + 1, byte1);
		self.memory.write_byte(pc + 2, byte2);
		self.memory.write_byte(pc + 3, byte3);
		self.pc = pc as u16 + 4;
	}
	
	#[allow(dead_code)]
	pub fn start_test(&mut self, instructions_to_execute: i8) -> () {
		self.pc = 0; //Ability to specify how many opcodes you want executed, for testing
		for _ in 0..instructions_to_execute {
			self.step()
		}
	}
	
	pub fn load_pal(&mut self, dir: i16) -> () {
		let dir = dir as u16 as usize;
		for i in 0..16 {
			let high: u32 = (self.memory.read_byte(dir + (i * 3)) as u8 as u32) << 16;
			let middle: u32 = (self.memory.read_byte(dir + (i * 3) + 1) as u8 as u32) << 8;
			let low: u32 = self.memory.read_byte(dir + (i * 3) + 2) as u8 as u32;
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
		self.rx[rx as u8 as usize]
	}
	
	pub fn set_rx(&mut self, rx: i8, value: i16) -> () {
		self.rx[rx as u8 as usize] = value;
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
		for i in 0..16i8 { //This syntax doesn't allow descending ranges
			let val = self.pop_stack();
			self.set_rx(15 - i, val);
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
			self.flags = !(Flag::Carry as i8) & self.flags
		}
	}
	
	pub fn put_zero(&mut self, new_state: bool) -> () {
		if new_state {
			self.flags = Flag::Zero as i8 | self.flags
		} else {
			self.flags = !(Flag::Zero as i8) & self.flags
		}
	}
	
	pub fn put_overflow(&mut self, new_state: bool) -> () {
		if new_state {
			self.flags = Flag::Overflow as i8 | self.flags
		} else {
			self.flags = !(Flag::Overflow as i8) & self.flags
		}
	}
	
	pub fn put_negative(&mut self, new_state: bool) -> () {
		if new_state {
			self.flags = Flag::Negative as i8 | self.flags
		} else {
			self.flags = !(Flag::Negative as i8) & self.flags
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
			0xC => self.has_overflow() == self.has_negative(),
			0xD => self.has_overflow() != self.has_negative(),
			0xE => (self.has_overflow() != self.has_negative()) || self.has_zero(),
			_ => panic!("Failed to find flag: {}", index),
		}
	}
	
	pub fn step(&mut self) -> () {
		if self.pc >= 0xFFFC {
			panic!("The instruction pointer has run out of memory to read");
		}
		let pc = self.pc as usize;
		let op = self.memory.read_byte(pc);

		let op: opcode::Opcode = to_opcode(op);
		let byte1 = self.memory.read_byte(pc + 1);
		let byte2 = self.memory.read_byte(pc + 2);
		let byte3 = self.memory.read_byte(pc + 3);
		self.pc = self.pc + 4;
		op.execute(self, byte1, byte2, byte3);
	}

	pub fn start_program(&mut self, mut window: &mut PistonWindow) -> () {
		let mut vblank_dt: u64 = 0;
		let mut controller1: u16 = 0;
		let mut controller2: u16 = 0;
		while let Some(e) = window.next() {
			
			if let Some(u) = e.update_args() {
				if vblank_dt < 16666 {
					for _ in 0..(u.dt * 1000000.0) as u64 {
						self.step();
						self.vblank = false;
					}
					vblank_dt += (u.dt * 1000000.0) as u64;
				}
			}
			
			if let Some(r) = e.render_args() {
				if vblank_dt >= 16666 {
					self.graphics.draw_screen(&mut window, &r, &e);
					self.vblank = true;
					vblank_dt -= 16666;
					self.memory.write_word(0xFFF0, controller1 as i16);
					self.memory.write_word(0xFFF2, controller2 as i16);
				}
			}

			if let Some(Button::Keyboard(key)) = e.press_args() {
				match key {
					Key::NumPad7 => controller1 |= Pad::A as u16,//A1
					Key::NumPad9 => controller1 |= Pad::B as u16,//B1
					Key::Right => controller1 |= Pad::Right as u16,//Right1
					Key::Up => controller1 |= Pad::Up as u16,//Up1
					Key::Down => controller1 |= Pad::Down as u16,//Down1
					Key::Left => controller1 |= Pad::Left as u16,//Left1
					Key::RShift => controller1 |= Pad::Select as u16,//Select1
					Key::Return => controller1 |= Pad::Start as u16,//Start1
				
					Key::H => controller2 |= Pad::A as u16,//A2
					Key::J => controller2 |= Pad::B as u16,//B2
					Key::D => controller2 |= Pad::Right as u16,//Right2
					Key::W => controller2 |= Pad::Up as u16,//Up2
					Key::S => controller2 |= Pad::Down as u16,//Down2
					Key::A => controller2 |= Pad::Left as u16,//Left2
					Key::LCtrl => controller2 |= Pad::Select as u16,//Select2
					Key::Space => controller2 |= Pad::Start as u16,//Start2
				
					_ => {},
				}
			}
			
			if let Some(Button::Keyboard(key)) = e.release_args() {
				match key {
					Key::NumPad7 => controller1 &= !(Pad::A as u16),//A1
					Key::NumPad9 => controller1 &= !(Pad::B as u16),//B1
					Key::Right => controller1 &= !(Pad::Right as u16),//Right1
					Key::Up => controller1 &= !(Pad::Up as u16),//Up1
					Key::Down => controller1 &= !(Pad::Down as u16),//Down1
					Key::Left => controller1 &= !(Pad::Left as u16),//Left1
					Key::RShift => controller1 &= !(Pad::Select as u16),//Select1
					Key::Return => controller1 &= !(Pad::Start as u16),//Start1
				
					Key::H => controller2 &= !(Pad::A as u16),//A2
					Key::J => controller2 &= !(Pad::B as u16),//B2
					Key::D => controller2 &= !(Pad::Right as u16),//Right2
					Key::W => controller2 &= !(Pad::Up as u16),//Up2
					Key::S => controller2 &= !(Pad::Down as u16),//Down2
					Key::A => controller2 &= !(Pad::Left as u16),//Left2
					Key::LCtrl => controller2 &= !(Pad::Select as u16),//Select2
					Key::Space => controller2 &= !(Pad::Start as u16),//Start2
				
					_ => {},
				}
			}
		}
	}
}
