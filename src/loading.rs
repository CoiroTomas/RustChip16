use cpu::{Cpu};
use std::io::{File, BytesReader, Seek};
use std::io;

pub fn load_bin(file: &mut File, cpu: &mut Cpu) -> () {
	let mut i: usize = 0;
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
		file.read_u8();
		file.read_u8();

		let rom_size: u32 = match file.read_be_u32() {
			Ok(rom) => rom,
			Err(e) => panic!("{}", e.desc)
		};
		let checksum: u32 = match file.read_be_u32(){
			Ok(sum) => sum,
			Err(e) => panic!("{}", e.desc)
		};
		cpu.pc = match file.read_be_i16() {
			Ok(ip) => ip,
			Err(e) => panic!("{}", e.desc)
		};

		check_rom_size(file, rom_size);
		crc32_checksum(file, checksum);

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

fn check_rom_size(file: &mut File, rom_size: u32) -> () {
	let file_size = match file.stat() {
		Ok(file_stat) => file_stat.size - 0x10,
		Err(e) => panic!("{}", e.desc)
	};
	if rom_size as u64 != file_size {
		panic!("The ROM size is not what the header says");
	}
}

fn crc32_checksum(file: &mut File, checksum: u32) -> () {
	()
}

