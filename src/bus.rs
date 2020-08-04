use crate::types::{Result, Address};

pub struct Bus {
    bytes: [u8; Self::WIDTH],
}

impl Bus {
    const WIDTH: usize = 0x10000;

    pub fn new() -> Self {
        Self { bytes: [0; Self::WIDTH] }
    }

    // TODO: error handling
    pub fn read(&self, address: Address) -> u8 {
        self.bytes[address as usize]
    }

    pub fn read_u16(&self, address: Address) -> Result<u16> {
        if address as usize + 1 >= Self::WIDTH {
            Err(anyhow!("reading 2 bytes from `${:04X}` would exceed max bus address `${:04X}`", address, Self::WIDTH - 1))
        } else {
            Ok(u16::from_le_bytes([self.read(address), self.read(address + 1)]))
        }
    }

    pub fn write(&mut self, address: Address, value: u8) {
        self.bytes[address as usize] = value;
    }

    pub fn write_u16(&mut self, address: Address, value: u16) -> Result {
        if address as usize + 1 >= Self::WIDTH {
            Err(anyhow!("writing 2 bytes to `${:04X}` would exceed max bus address `${:04X}`", address, Self::WIDTH - 1))
        } else {
            let bytes = u16::to_le_bytes(value);
            self.write(address, bytes[0]);
            self.write(address + 1, bytes[1]);
            Ok(())
        }
    }
}
