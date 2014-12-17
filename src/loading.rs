use cpu::{Cpu};
use std::io::{File, BytesReader, Seek};
use std::io;

pub fn load_bin(file: &mut File, cpu: &mut Cpu) -> () {
	let mut i: uint = 0;
	for byte in file.bytes() {
		let byte = match byte {
			Ok(number) => number as i8,
			Err(e) => panic!("{}", e.desc),
		};
		cpu.memory.write_byte(i, byte);
		i += 1;
	}
}

pub fn load_c16(file: &mut File, cpu: &mut Cpu) -> () {
	let magic_number = match file.read_be_u64() {
		Ok(number) => number,
		Err(e) => panic!("{}", e.desc)
	};
	if magic_number == 0x43483135 {
		//Do verifications (checksum, size check)
		match file.seek(0x10, io::SeekSet){
			Ok(ok) => ok,
			Err(e) => panic!("{}", e.desc)
		};
		load_bin(file, cpu);
	} else {
		match file.seek(0, io::SeekSet){
			Ok(ok) => ok,
			Err(e) => panic!("{}", e.desc)
		};
		load_bin(file, cpu);
	}
}
