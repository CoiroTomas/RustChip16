use std::io::File;

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
	    StateRegister { bg: 0, spritew: 0, spriteh: 0, hflip: false, vflip: false,}
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
}
