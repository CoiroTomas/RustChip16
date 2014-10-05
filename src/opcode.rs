extern crate num;
use self::num::integer::Integer;
use std::rand::{task_rng, Rng};
use cpu::{Cpu, join_bytes, separate_byte};
use std::mem;

pub fn to_opcode(v: i8) -> Opcode {
    unsafe { mem::transmute(v) }
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
	Jmc,
	Jx,
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
	And2,
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
			Cls => cpu.clear_fg_bg(),
			Vblnk => vblnk(cpu),
			Bgc => cpu.set_bg(byte2 as u8),
			Spr => cpu.set_spr_wh(byte2 as u8, byte3 as u8),
			Drw => nop(),
			Drw2 => nop(),
			Rnd => rnd(cpu, byte1, join_bytes(byte2, byte3)),
			Flip => flip(cpu, byte3),
			Snd0 => nop(),
			Snd1 => nop(),
			Snd2 => nop(),
			Snd3 => nop(),
			Snp => nop(),
			Sng => nop(),
			Jmp => jmp(cpu, join_bytes(byte2, byte3)),
			Jmc => jmc(cpu, join_bytes(byte2, byte3)),
			Jx => jx(cpu, byte1, join_bytes(byte2, byte3)),
			Jme => jme(cpu, separate_byte(byte1), join_bytes(byte2, byte3)),
			Call => call(cpu, join_bytes(byte2, byte3)),
			Ret => ret(cpu),
			Jmp2 => {
				let rx = cpu.get_rx(byte1);
				jmp(cpu, rx)
			},
			Cx => cx(cpu, byte1, join_bytes(byte2, byte3)),
			Call2 => {
				let rx = cpu.get_rx(byte1);
				call(cpu, rx)
			},
			Ldi => ldi(cpu, byte1, join_bytes(byte2, byte3)),
			Ldi2 => ldisp(cpu, join_bytes(byte2, byte3)),
			Ldm => ldm(cpu, byte1, join_bytes(byte2, byte3)),
			Ldm2 => ldmrx(cpu, separate_byte(byte1)),
			Mov => mov(cpu, separate_byte(byte1)),
			Stm => stm(cpu, byte1, join_bytes(byte2, byte3)),
			Stm2 => {
				let (x, y) = separate_byte(byte1);
				ldmrx(cpu, (y, x));
			},
			Addi => addi(cpu, byte1, join_bytes(byte2, byte3)),
			Add => {
				let (y, x) = separate_byte(byte1);
				add(cpu, (y, x), x);
			},
			Add2 => add(cpu, separate_byte(byte1), byte2),
			Subi => subi(cpu, byte1, join_bytes(byte2, byte3)),
			Sub => {
				let (y, x) = separate_byte(byte1);
				sub(cpu, (y, x), x);
			},
			Sub2 => sub(cpu, separate_byte(byte1), byte2),
			Cmpi => {cmpi(cpu, byte1, join_bytes(byte2, byte3));},
			Cmp => {cmp(cpu, separate_byte(byte1));},
			Andi => andi(cpu, byte1, join_bytes(byte2, byte3)),
			And => {
				let (y, x) = separate_byte(byte1);
				and(cpu, (y, x), x);
			},
			And2 => and(cpu, separate_byte(byte1), byte2),
			Tsti => {tsti(cpu, byte1, join_bytes(byte2, byte3));},
			Tst => {tst(cpu, separate_byte(byte1));},
			Ori => ori(cpu, byte1, join_bytes(byte2, byte3)),
			Or => {
				let (y, x) = separate_byte(byte1);
				or(cpu, (y, x), x);
			},
			Or2 => or(cpu, separate_byte(byte1), byte2),
			Xori => xori(cpu, byte1, join_bytes(byte2, byte3)),
			Xor => {
				let (y, x) = separate_byte(byte1);
				xor(cpu, (y, x), x);
			},
			Xor2 => xor(cpu, separate_byte(byte1), byte2),
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

fn vblnk(cpu: &mut Cpu)-> () {
	if !cpu.vblank {
		cpu.pc = cpu.pc - 4
	}
}

fn flip(cpu: &mut Cpu, byte3: i8) -> () {
	cpu.flip(byte3 > 1, byte3.is_odd())
}

fn rnd(cpu: &mut Cpu, rx: i8, max_rand: i16) -> () {
	cpu.set_rx(rx, task_rng().gen_range(0, max_rand as u16) as i16);
}

fn jmp(cpu: &mut Cpu, new_dir: i16) -> () {
	cpu.pc = new_dir;
}

fn jmc(cpu: &mut Cpu, new_dir: i16) -> () {
	if cpu.has_carry() {
		jmp(cpu, new_dir);
	}
}

fn jx(cpu: &mut Cpu, flag_index: i8, new_dir: i16) -> () {
	if cpu.check_flags(flag_index) {
		jmp(cpu, new_dir);
	}
}

fn jme(cpu: &mut Cpu, (y, x): (i8, i8), new_dir: i16) -> () {
	if cpu.get_rx(x) == cpu.get_rx(y) {
		jmp(cpu, new_dir);
	}
}

fn call(cpu: &mut Cpu, new_dir: i16) -> () {
	let pc = cpu.pc;
	cpu.push_stack(pc);
	cpu.pc = new_dir;
}

fn ret(cpu: &mut Cpu) -> () {
	let pc = cpu.pop_stack();
	cpu.pc = pc;
}

fn cx(cpu: &mut Cpu, flag_index: i8, new_dir: i16) -> () {
	if cpu.check_flags(flag_index) {
		call(cpu, new_dir);
	}
}

fn ldi(cpu: &mut Cpu, rx: i8, value: i16) -> () {
	cpu.set_rx(rx, value);
}

fn ldisp(cpu: &mut Cpu, value: i16) -> () {
	cpu.sp = value;
}

fn ldm(cpu: &mut Cpu, rx: i8, dir: i16) -> () {
	let value = cpu.memory.read_word(dir as uint);
	cpu.set_rx(rx, value);
}

fn ldmrx(cpu: &mut Cpu, (y, x): (i8, i8)) -> () {
	let value = cpu.memory.read_word(cpu.get_rx(y) as uint);
	cpu.set_rx(x, value);
}

fn mov(cpu: &mut Cpu, (y, x): (i8, i8)) -> () {
	let value = cpu.get_rx(y);
	cpu.set_rx(x, value);
}

fn stm(cpu: &mut Cpu, rx: i8, dir: i16) -> () {
	let value = cpu.get_rx(rx);
	cpu.memory.write_word(dir as uint, value);
}

fn sign(number: i16) -> int{
	if number > 0 {
		return 1;
	} else if number < 0 {
		return -1;
	} else {
		return 0;
	}
}

fn change_flags_add(cpu: &mut Cpu, original: i16, value: i16, result: i16) -> () {
	cpu.put_carry((original as u32 + value as u32) > (1 << 15));
	cpu.put_zero(result == 0);
	cpu.put_overflow(sign(original) == sign(value) && sign(result) != sign(original));
	cpu.put_negative(result < 0);
}

fn addi(cpu: &mut Cpu, rx:i8, value: i16) -> () {
	let rx_val = cpu.get_rx(rx);
	let result = rx_val + value;
	change_flags_add(cpu, rx_val, value, result);
	cpu.set_rx(rx, value);
}

fn add(cpu: &mut Cpu, (ry, rx): (i8, i8), rz: i8) -> () {
	let rx_val = cpu.get_rx(rx);
	let ry_val = cpu.get_rx(ry);
	let result = rx_val + ry_val;
	change_flags_add(cpu, rx_val, ry_val, result);
	cpu.set_rx(rz, result)
}

fn change_flags_sub(cpu: &mut Cpu, original: i16, value: i16, result: i16) -> () {
	cpu.put_carry((original as u32 - value as u32) > (1 << 15));
	cpu.put_zero(result == 0);
	cpu.put_overflow(sign(original) == sign(value) && sign(result) != sign(original));
	cpu.put_negative(result < 0);
}

fn subi(cpu: &mut Cpu, rx:i8, value: i16) -> () {
	let result = cmpi(cpu, rx, value);
	cpu.set_rx(rx, result);
}

fn sub(cpu: &mut Cpu, (ry, rx): (i8, i8), rz: i8) -> () {
	let result = cmp(cpu, (ry, rx));
	cpu.set_rx(rz, result);
}

fn cmpi(cpu: &mut Cpu, rx:i8, value: i16) -> i16 {
	let rx_val = cpu.get_rx(rx);
	let result = rx_val - value;
	change_flags_sub(cpu, rx_val, value, result);
	result
}

fn cmp(cpu: &mut Cpu, (ry, rx): (i8, i8)) -> i16 {
	let rx_val = cpu.get_rx(rx);
	let ry_val = cpu.get_rx(ry);
	let result = rx_val + ry_val;
	change_flags_sub(cpu, rx_val, ry_val, result);
	result
}

fn change_flags_bitwise(cpu: &mut Cpu, result: i16) -> () {
	cpu.put_zero(result == 0);
	cpu.put_negative(result < 0);
}

fn andi(cpu: &mut Cpu, rx:i8, value: i16) -> () {
	let result = tsti(cpu, rx, value);
	cpu.set_rx(rx, result);
}

fn and(cpu: &mut Cpu, (ry, rx): (i8, i8), rz: i8) -> () {
	let result = tst(cpu, (ry, rx));
	cpu.set_rx(rz, result);
}

fn tsti(cpu: &mut Cpu, rx:i8, value: i16) -> i16 {
	let rx_val = cpu.get_rx(rx);
	let result = rx_val & value;
	change_flags_bitwise(cpu, result);
	result
}

fn tst(cpu: &mut Cpu, (ry, rx): (i8, i8)) -> i16 {
	let rx_val = cpu.get_rx(rx);
	let ry_val = cpu.get_rx(ry);
	let result = rx_val & ry_val;
	change_flags_bitwise(cpu, result);
	result
}

fn ori(cpu: &mut Cpu, rx:i8, value: i16) -> () {
	let rx_val = cpu.get_rx(rx);
	let result = rx_val | value;
	change_flags_bitwise(cpu, result);
	cpu.set_rx(rx, result);
}

fn or(cpu: &mut Cpu, (ry, rx): (i8, i8), rz: i8) -> () {
	let rx_val = cpu.get_rx(rx);
	let ry_val = cpu.get_rx(ry);
	let result = rx_val | ry_val;
	change_flags_bitwise(cpu, result);
	cpu.set_rx(rz, result);
}

fn xori(cpu: &mut Cpu, rx:i8, value: i16) -> () {
	let rx_val = cpu.get_rx(rx);
	let result = rx_val ^ value;
	change_flags_bitwise(cpu, result);
	cpu.set_rx(rx, result);
}

fn xor(cpu: &mut Cpu, (ry, rx): (i8, i8), rz: i8) -> () {
	let rx_val = cpu.get_rx(rx);
	let ry_val = cpu.get_rx(ry);
	let result = rx_val ^ ry_val;
	change_flags_bitwise(cpu, result);
	cpu.set_rx(rz, result);
}