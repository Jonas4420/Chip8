use crate::error::Error;

const MEMORY_SIZE: usize = 0x1000;

#[derive(Debug)]
pub struct Ram {
    memory: [u8; MEMORY_SIZE],
}

impl Ram {
    pub fn read(&mut self, addr: u16) -> Result<u8, Error> {
        if (addr as usize) < self.memory.len() {
            Ok(self.memory[addr as usize])
        } else {
            Err(Error::InvalidAddress(addr))
        }
    }

    pub fn write(&mut self, addr: u16, byte: u8) -> Result<(), Error> {
        if (addr as usize) < self.memory.len() {
            self.memory[addr as usize] = byte;
            Ok(())
        } else {
            Err(Error::InvalidAddress(addr))
        }
    }
}

impl Default for Ram {
    fn default() -> Self {
        Self {
            memory: [Default::default(); MEMORY_SIZE],
        }
    }
}
