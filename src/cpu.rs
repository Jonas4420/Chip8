use crate::bus::Bus;
use crate::error::Error;
use crate::io::IO;

type Result<T> = std::result::Result<T, Error>;

const FONT_SIZE: u16 = 5;
const OPCODE_SIZE: u16 = 2;

macro_rules! nnn {
    ($op: expr) => {
        ((($op[1] & 0xf) as u16) << 8) | ((($op[2] & 0xf) as u16) << 4) | ((($op[3] & 0xf) as u16) << 0)
    };
}

macro_rules! kk {
    ($op: expr) => {
        ((($op[2] & 0xf) as u8) << 4) | (($op[3] & 0xf) as u8)
    };
}

macro_rules! n {
    ($op: expr) => {
        $op[3] as u8
    };
}

macro_rules! x {
    ($op: expr) => {
        $op[1] as usize
    };
}

macro_rules! y {
    ($op: expr) => {
        $op[2] as usize
    };
}

#[derive(Debug, Default)]
pub struct Cpu {
    v: [u8; 0x10],
    i: u16,
    pc: u16,
    sp: u8,
    stack: [u16; 0x10],
    ft: u16,
}

#[derive(Debug)]
enum ProgramCounter {
    Wait,
    Next,
    Skip,
    Jump(u16),
}

impl Cpu {
    pub fn init(&mut self, pc: u16, ft: u16) {
        self.pc = pc;
        self.sp = 0;
        self.ft = ft;
    }

    pub fn cycle(&mut self, bus: &mut Bus, io: &mut IO) -> Result<()> {
        // Fetch
        let hi = bus.ram.read(self.pc)?;
        let lo = bus.ram.read(self.pc.wrapping_add(1))?;

        let opcode = [(hi >> 4) & 0xf, hi & 0xf, (lo >> 4) & 0xf, lo & 0xf];

        // Decode and execute
        let pc = match opcode {
            [0x0, 0x0, 0x0, 0x0] => self.op_nop(bus, io),
            [0x0, 0x0, 0xe, 0x0] => self.op_cls(bus, io),
            [0x0, 0x0, 0xe, 0xe] => self.op_ret(bus, io),
            [0x1, _, _, _] => self.op_jmp(bus, io, nnn!(opcode)),
            [0x2, _, _, _] => self.op_call(bus, io, nnn!(opcode)),
            [0x3, _, _, _] => self.op_sei(bus, io, x!(opcode), kk!(opcode)),
            [0x4, _, _, _] => self.op_snei(bus, io, x!(opcode), kk!(opcode)),
            [0x5, _, _, 0x0] => self.op_se(bus, io, x!(opcode), y!(opcode)),
            [0x6, _, _, _] => self.op_movi(bus, io, x!(opcode), kk!(opcode)),
            [0x7, _, _, _] => self.op_addi(bus, io, x!(opcode), kk!(opcode)),
            [0x8, _, _, 0x0] => self.op_mov(bus, io, x!(opcode), y!(opcode)),
            [0x8, _, _, 0x1] => self.op_or(bus, io, x!(opcode), y!(opcode)),
            [0x8, _, _, 0x2] => self.op_and(bus, io, x!(opcode), y!(opcode)),
            [0x8, _, _, 0x3] => self.op_xor(bus, io, x!(opcode), y!(opcode)),
            [0x8, _, _, 0x4] => self.op_add(bus, io, x!(opcode), y!(opcode)),
            [0x8, _, _, 0x5] => self.op_sub(bus, io, x!(opcode), y!(opcode)),
            [0x8, _, _, 0x6] => self.op_shr(bus, io, x!(opcode), y!(opcode)),
            [0x8, _, _, 0x7] => self.op_subn(bus, io, x!(opcode), y!(opcode)),
            [0x8, _, _, 0xe] => self.op_shl(bus, io, x!(opcode), y!(opcode)),
            [0x9, _, _, 0x0] => self.op_sne(bus, io, x!(opcode), y!(opcode)),
            [0xa, _, _, _] => self.op_lea(bus, io, nnn!(opcode)),
            [0xb, _, _, _] => self.op_jmpshort(bus, io, nnn!(opcode)),
            [0xc, _, _, _] => self.op_rnd(bus, io, x!(opcode), kk!(opcode)),
            [0xd, _, _, _] => self.op_drw(bus, io, x!(opcode), y!(opcode), n!(opcode)),
            [0xe, _, 0x9, 0xe] => self.op_skp(bus, io, x!(opcode)),
            [0xe, _, 0xa, 0x1] => self.op_sknp(bus, io, x!(opcode)),
            [0xf, _, 0x0, 0x7] => self.op_get_dt(bus, io, x!(opcode)),
            [0xf, _, 0x0, 0xa] => self.op_wait(bus, io, x!(opcode)),
            [0xf, _, 0x1, 0x5] => self.op_set_dt(bus, io, x!(opcode)),
            [0xf, _, 0x1, 0x8] => self.op_set_st(bus, io, x!(opcode)),
            [0xf, _, 0x1, 0xe] => self.op_inc(bus, io, x!(opcode)),
            [0xf, _, 0x2, 0x9] => self.op_ldfont(bus, io, x!(opcode)),
            [0xf, _, 0x3, 0x3] => self.op_bcd(bus, io, x!(opcode)),
            [0xf, _, 0x5, 0x5] => self.op_pusha(bus, io, x!(opcode)),
            [0xf, _, 0x6, 0x5] => self.op_popa(bus, io, x!(opcode)),
            _ => Err(Error::UnknownOpcode(opcode)),
        }?;

        self.pc = match pc {
            ProgramCounter::Wait => self.pc,
            ProgramCounter::Next => self.pc.wrapping_add(OPCODE_SIZE),
            ProgramCounter::Skip => self.pc.wrapping_add(2 * OPCODE_SIZE),
            ProgramCounter::Jump(addr) => addr,
        };

        Ok(())
    }

    fn op_nop(&mut self, _bus: &mut Bus, _io: &mut IO) -> Result<ProgramCounter> {
        Ok(ProgramCounter::Next)
    }

    fn op_cls(&mut self, _bus: &mut Bus, io: &mut IO) -> Result<ProgramCounter> {
        io.screen.clear();
        Ok(ProgramCounter::Next)
    }

    fn op_ret(&mut self, _bus: &mut Bus, _io: &mut IO) -> Result<ProgramCounter> {
        let addr = self.stack_pop()?;
        Ok(ProgramCounter::Jump(addr))
    }

    fn op_jmp(&mut self, _bus: &mut Bus, _io: &mut IO, nnn: u16) -> Result<ProgramCounter> {
        Ok(ProgramCounter::Jump(nnn))
    }

    fn op_call(&mut self, _bus: &mut Bus, _io: &mut IO, nnn: u16) -> Result<ProgramCounter> {
        self.stack_push(self.pc.wrapping_add(OPCODE_SIZE))?;
        Ok(ProgramCounter::Jump(nnn))
    }

    fn op_sei(&mut self, _bus: &mut Bus, _io: &mut IO, x: usize, kk: u8) -> Result<ProgramCounter> {
        Ok(ProgramCounter::skip_if(self.v[x] == kk))
    }

    fn op_snei(&mut self, _bus: &mut Bus, _io: &mut IO, x: usize, kk: u8) -> Result<ProgramCounter> {
        Ok(ProgramCounter::skip_if(self.v[x] != kk))
    }

    fn op_se(&mut self, _bus: &mut Bus, _io: &mut IO, x: usize, y: usize) -> Result<ProgramCounter> {
        Ok(ProgramCounter::skip_if(self.v[x] == self.v[y]))
    }

    fn op_movi(&mut self, _bus: &mut Bus, _io: &mut IO, x: usize, kk: u8) -> Result<ProgramCounter> {
        self.v[x] = kk;
        Ok(ProgramCounter::Next)
    }

    fn op_addi(&mut self, _bus: &mut Bus, _io: &mut IO, x: usize, kk: u8) -> Result<ProgramCounter> {
        self.v[x] = self.v[x].wrapping_add(kk);
        Ok(ProgramCounter::Next)
    }

    fn op_mov(&mut self, _bus: &mut Bus, _io: &mut IO, x: usize, y: usize) -> Result<ProgramCounter> {
        self.v[x] = self.v[y];
        Ok(ProgramCounter::Next)
    }

    fn op_or(&mut self, _bus: &mut Bus, _io: &mut IO, x: usize, y: usize) -> Result<ProgramCounter> {
        self.v[x] |= self.v[y];
        Ok(ProgramCounter::Next)
    }

    fn op_and(&mut self, _bus: &mut Bus, _io: &mut IO, x: usize, y: usize) -> Result<ProgramCounter> {
        self.v[x] &= self.v[y];
        Ok(ProgramCounter::Next)
    }

    fn op_xor(&mut self, _bus: &mut Bus, _io: &mut IO, x: usize, y: usize) -> Result<ProgramCounter> {
        self.v[x] ^= self.v[y];
        Ok(ProgramCounter::Next)
    }

    fn op_add(&mut self, _bus: &mut Bus, _io: &mut IO, x: usize, y: usize) -> Result<ProgramCounter> {
        let (vx, vf) = self.v[x].overflowing_add(self.v[y]);
        self.v[x] = vx;
        self.v[0xf] = if vf { 0x01 } else { 0x00 };
        Ok(ProgramCounter::Next)
    }

    fn op_sub(&mut self, _bus: &mut Bus, _io: &mut IO, x: usize, y: usize) -> Result<ProgramCounter> {
        let vf = if self.v[x] > self.v[y] { 0x01 } else { 0x00 };
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
        self.v[0xf] = vf;
        Ok(ProgramCounter::Next)
    }

    fn op_shr(&mut self, _bus: &mut Bus, _io: &mut IO, x: usize, _y: usize) -> Result<ProgramCounter> {
        let vf = if self.v[x] & 0x01 != 0 { 0x01 } else { 0x00 };
        self.v[x] >>= 1;
        self.v[0xf] = vf;
        Ok(ProgramCounter::Next)
    }

    fn op_subn(&mut self, _bus: &mut Bus, _io: &mut IO, x: usize, y: usize) -> Result<ProgramCounter> {
        let vf = if self.v[y] > self.v[x] { 0x01 } else { 0x00 };
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
        self.v[0xf] = vf;
        Ok(ProgramCounter::Next)
    }

    fn op_shl(&mut self, _bus: &mut Bus, _io: &mut IO, x: usize, _y: usize) -> Result<ProgramCounter> {
        let vf = if self.v[x] & 0x80 != 0 { 0x01 } else { 0x00 };
        self.v[x] <<= 1;
        self.v[0xf] = vf;
        Ok(ProgramCounter::Next)
    }

    fn op_sne(&mut self, _bus: &mut Bus, _io: &mut IO, x: usize, y: usize) -> Result<ProgramCounter> {
        Ok(ProgramCounter::skip_if(self.v[x] != self.v[y]))
    }

    fn op_lea(&mut self, _bus: &mut Bus, _io: &mut IO, nnn: u16) -> Result<ProgramCounter> {
        self.i = nnn;
        Ok(ProgramCounter::Next)
    }

    fn op_jmpshort(&mut self, _bus: &mut Bus, _io: &mut IO, nnn: u16) -> Result<ProgramCounter> {
        let addr = nnn.wrapping_add(self.v[0] as u16);
        Ok(ProgramCounter::Jump(addr))
    }

    fn op_rnd(&mut self, bus: &mut Bus, _io: &mut IO, x: usize, kk: u8) -> Result<ProgramCounter> {
        self.v[x] = bus.rng.get_byte() & kk;
        Ok(ProgramCounter::Next)
    }

    fn op_drw(&mut self, bus: &mut Bus, io: &mut IO, x: usize, y: usize, n: u8) -> Result<ProgramCounter> {
        let vf = (0..n).try_fold(0x00, |acc, i| {
            let addr = self.i.wrapping_add(i as u16);
            bus.ram.read(addr).map(|byte| {
                if io.screen.draw(self.v[x], self.v[y].wrapping_add(i), byte) {
                    0x01
                } else {
                    acc
                }
            })
        })?;

        self.v[0xf] = vf;

        Ok(ProgramCounter::Next)
    }

    fn op_skp(&mut self, _bus: &mut Bus, io: &mut IO, x: usize) -> Result<ProgramCounter> {
        if (self.v[x] as usize) < io.pad.len() {
            Ok(ProgramCounter::skip_if(io.pad[self.v[x] as usize]))
        } else {
            Err(Error::PadAddressOutOfRange(self.v[x]))
        }
    }

    fn op_sknp(&mut self, _bus: &mut Bus, io: &mut IO, x: usize) -> Result<ProgramCounter> {
        if (self.v[x] as usize) < io.pad.len() {
            Ok(ProgramCounter::skip_if(!io.pad[self.v[x] as usize]))
        } else {
            Err(Error::PadAddressOutOfRange(self.v[x]))
        }
    }

    fn op_get_dt(&mut self, bus: &mut Bus, _io: &mut IO, x: usize) -> Result<ProgramCounter> {
        self.v[x] = bus.dt.get();
        Ok(ProgramCounter::Next)
    }

    fn op_wait(&mut self, _bus: &mut Bus, io: &mut IO, x: usize) -> Result<ProgramCounter> {
        let any_pressed = io
            .pad
            .iter()
            .position(|&key| key)
            .map(|idx| {
                self.v[x] = idx as u8;
            })
            .is_some();

        Ok(ProgramCounter::wait_if(!any_pressed))
    }

    fn op_set_dt(&mut self, bus: &mut Bus, _io: &mut IO, x: usize) -> Result<ProgramCounter> {
        bus.dt.set(self.v[x]);
        Ok(ProgramCounter::Next)
    }

    fn op_set_st(&mut self, bus: &mut Bus, _io: &mut IO, x: usize) -> Result<ProgramCounter> {
        bus.st.set(self.v[x]);
        Ok(ProgramCounter::Next)
    }

    fn op_inc(&mut self, _bus: &mut Bus, _io: &mut IO, x: usize) -> Result<ProgramCounter> {
        self.i = self.i.wrapping_add(self.v[x] as u16);
        Ok(ProgramCounter::Next)
    }

    fn op_ldfont(&mut self, _bus: &mut Bus, _io: &mut IO, x: usize) -> Result<ProgramCounter> {
        self.i = self.ft.wrapping_add((self.v[x] as u16) * FONT_SIZE);
        Ok(ProgramCounter::Next)
    }

    fn op_bcd(&mut self, bus: &mut Bus, _io: &mut IO, x: usize) -> Result<ProgramCounter> {
        let digits = [(self.v[x] / 100) % 10, (self.v[x] / 10) % 10, self.v[x] % 10];

        for (i, digit) in digits.iter().copied().enumerate() {
            bus.ram.write(self.i.wrapping_add(i as u16), digit)?;
        }

        Ok(ProgramCounter::Next)
    }

    fn op_pusha(&mut self, bus: &mut Bus, _io: &mut IO, x: usize) -> Result<ProgramCounter> {
        for i in 0..=x {
            bus.ram.write(self.i.wrapping_add(i as u16), self.v[i])?;
        }

        Ok(ProgramCounter::Next)
    }

    fn op_popa(&mut self, bus: &mut Bus, _io: &mut IO, x: usize) -> Result<ProgramCounter> {
        for i in 0..=x {
            self.v[i] = bus.ram.read(self.i.wrapping_add(i as u16))?;
        }

        Ok(ProgramCounter::Next)
    }

    fn stack_push(&mut self, addr: u16) -> Result<()> {
        if (self.sp as usize) < self.stack.len() {
            self.stack[self.sp as usize] = addr;
            self.sp += 1;
            Ok(())
        } else {
            Err(Error::StackOverflow)
        }
    }

    fn stack_pop(&mut self) -> Result<u16> {
        if self.sp > 0 {
            self.sp -= 1;
            Ok(self.stack[self.sp as usize])
        } else {
            Err(Error::StackOverflow)
        }
    }
}

impl ProgramCounter {
    pub fn skip_if(cond: bool) -> Self {
        if cond {
            Self::Skip
        } else {
            Self::Next
        }
    }

    pub fn wait_if(cond: bool) -> Self {
        if cond {
            Self::Wait
        } else {
            Self::Next
        }
    }
}
