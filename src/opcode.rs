extern crate num;
use self::num::integer::Integer;
use cpu::Cpu;
use std::mem;

pub fn to_opcode(v: i8) -> Opcode {
    unsafe { mem::transmute(v) }
}

fn join_bytes(hh: i8, ll: i8) -> i16 {
	((((hh as u8) as u16) << 8) + (ll as u8) as u16) as i16
}

fn separate_word(word: i16) -> (i8, i8) {
	let word = word as u16;
	let hh = (word >> 8) as u8;
	let ll = (word & 0xff) as u8;
	(hh as i8, ll as i8)
}

fn separate_byte(byte: i8) -> (u8, u8) {
	let byte = byte as u8;
	let hh = (byte >> 4) as u8;
	let ll = byte & 0xf;
	(hh, ll)
}

pub enum Opcode {
    Nop = 0,
	Cls,
	Vblnk,
	Bgc,
	Spr,
	Drw,
	Drw2,
	Rnd,
	Flip,
	Snd0,
	Snd1,
	Snd2,
	Snd3,
	Snp,
	Sng,
	Jmp = 0x10,
	Jx = 0x12,
	Jme,
	Call,
	Ret,
	Jmp2,
	Cx,
	Call2,
	Ldi = 0x20,
	Ldi2,
	Ldm,
	Ldm2,
	Mov,
	Stm = 0x30,
	Stm2,
	Addi = 0x40,
	Add,
	Add2,
	Subi = 0x50,
	Sub,
	Sub2,
	Cmpi,
	Cmp,
	Andi = 0x60,
	And,
	Tsti,
	Tst,
	Ori = 0x70,
	Or,
	Or2,
	Xori = 0x80,
	Xor,
	Xor2,
	Muli = 0x90,
	Mul,
	Mul2,
	Divi = 0xA0,
	Div,
	Div2,
	Modi,
	Mod,
	Mod2,
	Remi,
	Rem,
	Rem2,
	Shl = 0xB0,
	Shr,
	Sar,
	Shl2,
	Shr2,
	Sar2,
	Push = 0xC0,
	Pop,
	Pushall,
	Popall,
	Pushf,
	Popf,
	Pal = 0xD0,
	Pal2,
	Noti = 0xE0,
	Not,
	Not2,
	Negi,
	Neg,
	Neg2,
}

impl Opcode {
	pub fn execute(&self, cpu: &mut Cpu, byte1: i8, byte2: i8, byte3: i8) {
	    match *self {
		    Nop => nop(),
			Cls => cls(cpu),
			Vblnk => nop(),
			Bgc => bgc(cpu, byte2),
			Spr => spr(cpu, byte2 as u8, byte3 as u8),
			Drw => nop(),
			Drw2 => nop(),
			Rnd => nop(),
			Flip => flip(cpu, byte3),
			Snd0 => nop(),
			Snd1 => nop(),
			Snd2 => nop(),
			Snd3 => nop(),
			Snp => nop(),
			Sng => nop(),
			Jmp => nop(),
			Jx => nop(),
			Jme => nop(),
			Call => nop(),
			Ret => nop(),
			Jmp2 => nop(),
			Cx => nop(),
			Call2 => nop(),
			Ldi => nop(),
			Ldi2 => nop(),
			Ldm => nop(),
			Ldm2 => nop(),
			Mov => nop(),
			Stm => nop(),
			Stm2 => nop(),
			Addi => nop(),
			Add => nop(),
			Add2 => nop(),
			Subi => nop(),
			Sub => nop(),
			Sub2 => nop(),
			Cmpi => nop(),
			Cmp => nop(),
			Andi => nop(),
			And => nop(),
			Tsti => nop(),
			Tst => nop(),
			Ori => nop(),
			Or => nop(),
			Or2 => nop(),
			Xori => nop(),
			Xor => nop(),
			Xor2 => nop(),
			Muli => nop(),
			Mul => nop(),
			Mul2 => nop(),
			Divi => nop(),
			Div => nop(),
			Div2 => nop(),
			Modi => nop(),
			Mod => nop(),
			Mod2 => nop(),
			Remi => nop(),
			Rem => nop(),
			Rem2 => nop(),
			Shl => nop(),
			Shr => nop(),
			Sar => nop(),
			Shl2 => nop(),
			Shr2 => nop(),
			Sar2 => nop(),
			Push => nop(),
			Pop => nop(),
			Pushall => nop(),
			Popall => nop(),
			Pushf => nop(),
			Popf => nop(),
			Pal => nop(),
			Pal2 => nop(),
			Noti => nop(),
			Not => nop(),
			Not2 => nop(),
			Negi => nop(),
			Neg => nop(),
			Neg2 => nop(),
		}
	}
}

fn nop() -> () {
	()
}

fn cls(cpu: &mut Cpu) -> () {
	cpu.clear_fg_bg();
}

fn bgc(cpu: &mut Cpu, byte: i8) -> () {
	cpu.set_bg(byte as u8);
}

fn spr(cpu: &mut Cpu, ll: u8, hh: u8) -> () {
	cpu.set_spr_wh(ll, hh);
}

fn flip(cpu: &mut Cpu, byte3: i8) -> () {
	cpu.flip(byte3 > 1, byte3.is_odd())
}