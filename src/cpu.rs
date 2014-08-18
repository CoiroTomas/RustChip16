use std::io::File;

struct Cpu {
    pc: i16,
	sp: i16,
	r0: i16,
	r1: i16,
	r2: i16,
	r3: i16,
	r4: i16,
	r5: i16,
	r6: i16,
	r7: i16,
	r8: i16,
	r9: i16,
	ra: i16,
	rb: i16,
	rc: i16,
	rd: i16,
	re: i16,
	rf: i16,
	flags: i8,
	file: File,
}

impl Cpu {
    fn new(file_path: std::path::Path) -> Cpu {
		let file = match File::open(&file_path) {
		    Err(why) => fail!("{} {}",why.desc, file_path.display()),
			Ok(file) => file,
		};
	    let cpu = Cpu {pc: 0, sp: 0, r0: 0, r1: 0, r2: 0, r3: 0, r4: 0, r5: 0, r6: 0, r7: 0,
		    r8: 0, r9: 0, ra: 0, rb: 0, rc: 0, rd: 0, re: 0, rf: 0, flags: 0, file: file};
		cpu
	}
}