use crate::bus::Bus;
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

const OPCODE_SIZE: u16 = 2;

macro_rules! nnn {
    ($op: expr) => {
        (((($op[1] & 0xf) as u16) << 8) | ((($op[2] & 0xf) as u16) << 4) | ((($op[3] & 0xf) as u16) << 0)).into()
    };
}

macro_rules! kk {
    ($op: expr) => {
        (((($op[2] & 0xf) as u8) << 4) | (($op[3] & 0xf) as u8)).into()
    };
}

macro_rules! x {
    ($op: expr) => {
        ($op[1]).into()
    };
}

macro_rules! y {
    ($op: expr) => {
        ($op[2]).into()
    };
}

macro_rules! n {
    ($op: expr) => {
        ($op[3]).into()
    };
}

#[derive(Debug, Default)]
pub struct Cpu {
    v: [u8; 0x10],
    i: u16,
    pc: u16,
    sp: u8,
    stack: [u16; 0x10],
}

#[derive(Debug)]
enum ProgramCounter {
    Wait,
    Next,
    Skip,
    Jump(u16),
}

impl Cpu {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn cycle(&mut self, bus: &mut Bus) -> Result<()> {
        // Fetch
        let hi = bus.ram.read(self.pc)?;
        let lo = bus.ram.read(self.pc + 1)?;

        let opcode = [(hi >> 4) & 0xf, (hi >> 0) & 0xf, (lo >> 4) & 0xf, (lo >> 0) & 0xf];

        // Decode and execute
        let pc = match opcode {
            [0x0, 0x0, 0x0, 0x0] => self.op_nop(bus),
            [0x0, 0x0, 0xe, 0x0] => self.op_cls(bus),
            [0x0, 0x0, 0xe, 0xe] => self.op_ret(bus),
            [0x1, _, _, _] => self.op_jmp(bus, nnn!(opcode)),
            [0x2, _, _, _] => self.op_call(bus, nnn!(opcode)),
            [0x3, _, _, _] => self.op_sei(bus, x!(opcode), kk!(opcode)),
            [0x4, _, _, _] => self.op_snei(bus, x!(opcode), kk!(opcode)),
            [0x5, _, _, 0x0] => self.op_se(bus, x!(opcode), y!(opcode)),
            [0x6, _, _, _] => self.op_movi(bus, x!(opcode), kk!(opcode)),
            [0x7, _, _, _] => self.op_addi(bus, x!(opcode), kk!(opcode)),
            [0x8, _, _, 0x0] => self.op_mov(bus, x!(opcode), y!(opcode)),
            [0x8, _, _, 0x1] => self.op_or(bus, x!(opcode), y!(opcode)),
            [0x8, _, _, 0x2] => self.op_and(bus, x!(opcode), y!(opcode)),
            [0x8, _, _, 0x3] => self.op_xor(bus, x!(opcode), y!(opcode)),
            [0x8, _, _, 0x4] => self.op_add(bus, x!(opcode), y!(opcode)),
            [0x8, _, _, 0x5] => self.op_sub(bus, x!(opcode), y!(opcode)),
            [0x8, _, _, 0x6] => self.op_shr(bus, x!(opcode), y!(opcode)),
            [0x8, _, _, 0x7] => self.op_subn(bus, x!(opcode), y!(opcode)),
            [0x8, _, _, 0xe] => self.op_shl(bus, x!(opcode), y!(opcode)),
            [0x9, _, _, 0x0] => self.op_sne(bus, x!(opcode), y!(opcode)),
            [0xa, _, _, _] => self.op_lea(bus, nnn!(opcode)),
            [0xb, _, _, _] => self.op_jmprel(bus, nnn!(opcode)),
            [0xc, _, _, _] => self.op_rnd(bus, x!(opcode), kk!(opcode)),
            [0xd, _, _, _] => self.op_drw(bus, x!(opcode), y!(opcode), n!(opcode)),
            [0xe, _, 0x9, 0xe] => self.op_skp(bus, x!(opcode)),
            [0xe, _, 0xa, 0x1] => self.op_sknp(bus, x!(opcode)),
            [0xf, _, 0x0, 0x7] => self.op_get_dt(bus, x!(opcode)),
            [0xf, _, 0x0, 0xa] => self.op_wait(bus, x!(opcode)),
            [0xf, _, 0x1, 0x5] => self.op_set_dt(bus, x!(opcode)),
            [0xf, _, 0x1, 0x8] => self.op_set_st(bus, x!(opcode)),
            [0xf, _, 0x1, 0xe] => self.op_inc(bus, x!(opcode)),
            [0xf, _, 0x2, 0x9] => self.op_ldfont(bus, x!(opcode)),
            [0xf, _, 0x3, 0x3] => self.op_bcd(bus, x!(opcode)),
            [0xf, _, 0x5, 0x5] => self.op_pusha(bus, x!(opcode)),
            [0xf, _, 0x6, 0x5] => self.op_popa(bus, x!(opcode)),
            _ => Err(Error::UnknownOpcode(opcode)),
        }?;

        self.pc = match pc {
            ProgramCounter::Wait => self.pc,
            ProgramCounter::Next => self.pc + OPCODE_SIZE,
            ProgramCounter::Skip => self.pc + (2 * OPCODE_SIZE),
            ProgramCounter::Jump(addr) => addr,
        };

        Ok(())
    }

    fn op_nop(&mut self, bus: &mut Bus) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_cls(&mut self, bus: &mut Bus) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_ret(&mut self, bus: &mut Bus) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_jmp(&mut self, bus: &mut Bus, nnn: u16) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_call(&mut self, bus: &mut Bus, nnn: u16) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_sei(&mut self, bus: &mut Bus, x: usize, kk: u8) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_snei(&mut self, bus: &mut Bus, x: usize, kk: u8) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_se(&mut self, bus: &mut Bus, x: usize, y: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_movi(&mut self, bus: &mut Bus, x: usize, kk: u8) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_addi(&mut self, bus: &mut Bus, x: usize, kk: u8) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_mov(&mut self, bus: &mut Bus, x: usize, y: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_or(&mut self, bus: &mut Bus, x: usize, y: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_and(&mut self, bus: &mut Bus, x: usize, y: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_xor(&mut self, bus: &mut Bus, x: usize, y: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_add(&mut self, bus: &mut Bus, x: usize, y: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_sub(&mut self, bus: &mut Bus, x: usize, y: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_shr(&mut self, bus: &mut Bus, x: usize, y: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_subn(&mut self, bus: &mut Bus, x: usize, y: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_shl(&mut self, bus: &mut Bus, x: usize, y: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_sne(&mut self, bus: &mut Bus, x: usize, y: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_lea(&mut self, bus: &mut Bus, nnn: u16) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_jmprel(&mut self, bus: &mut Bus, nnn: u16) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_rnd(&mut self, bus: &mut Bus, x: usize, kk: u8) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_drw(&mut self, bus: &mut Bus, x: usize, y: usize, n: u8) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_skp(&mut self, bus: &mut Bus, x: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_sknp(&mut self, bus: &mut Bus, x: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_get_dt(&mut self, bus: &mut Bus, x: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_wait(&mut self, bus: &mut Bus, x: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_set_dt(&mut self, bus: &mut Bus, x: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_set_st(&mut self, bus: &mut Bus, x: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_inc(&mut self, bus: &mut Bus, x: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_ldfont(&mut self, bus: &mut Bus, x: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_bcd(&mut self, bus: &mut Bus, x: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_pusha(&mut self, bus: &mut Bus, x: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }

    fn op_popa(&mut self, bus: &mut Bus, x: usize) -> Result<ProgramCounter> {
        // TODO
        unimplemented!()
    }
}
