use std::io::File;

enum Flags {
    Carry = 2,
   	Zero = 4,
   	Overflow = 64,
	Negative = 128,
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
	state: StateRegister,
	vblank: bool,
	memory: [i8, ..65536],
}
	
impl Cpu {
    pub fn new(file_path: Path) -> Cpu {
    	let file = match File::open(&file_path) {
   		    Err(why) => fail!("{} {}",why.desc, file_path.display()),
		    Ok(file) => file,
	    };
        let cpu = Cpu {pc: 0, sp: 0, r: [0, ..16], flags: 0, file: file,
	    	state : StateRegister {bg: 0, spritew: 0, spriteh: 0, hflip: false, vflip: false,},
	    	vblank: false, memory: [0, ..65536],};
	    cpu
    }
}
