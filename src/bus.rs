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

    pub fn read_zp(&self, address: u8) -> u8 {
        self.read(address.into())
    }

    pub fn read_u16(&self, address: Address) -> Result<u16> {
        if address as usize + 1 >= Self::WIDTH {
            Err(anyhow!("reading 2 bytes from `${:04X}` would exceed max bus address `${:04X}`", address, Self::WIDTH - 1))
        } else {
            Ok(u16::from_le_bytes([self.read(address), self.read(address + 1)]))
        }
    }

    pub fn read_zp_u16(&self, address: u8) -> Result<u16> {
        if let Some(_) = address.checked_add(1) {
            Ok(self.read_u16(address.into())?)
        } else {
            Err(anyhow!("writing 2 bytes to `${:04X}` would exceed the maximum zero-page address of `${:04X}`", address, u8::MAX))
        }
    }

    pub fn read_n(&self, address: Address, n: u16) -> Vec<u8> {
        let mut bytes = Vec::new();

        for i in 0..n {
            bytes.push(self.read(address + i));
        }

        bytes
    }

    pub fn write(&mut self, address: Address, value: u8) {
        self.bytes[address as usize] = value;
    }

    pub fn write_zp(&mut self, address: u8, value: u8) {
        self.write(address as Address, value);
    }

    pub fn write_u16(&mut self, address: Address, value: u16) -> Result {
        if address as usize + 1 >= Self::WIDTH {
            Err(anyhow!("writing 2 bytes to `${:04X}` would exceed the maximum bus address of `${:04X}`", address, Self::WIDTH - 1))
        } else {
            let bytes = u16::to_le_bytes(value);
            self.write(address, bytes[0]);
            self.write(address + 1, bytes[1]);
            Ok(())
        }
    }

    pub fn write_zp_u16(&mut self, address: u8, value: u16) -> Result {
        if let Some(_) = address.checked_add(1) {
            let bytes = u16::to_le_bytes(value);
            self.write(address as Address, bytes[0]);
            self.write(address as Address + 1, bytes[1]);
            Ok(())
        } else {
            Err(anyhow!("writing 2 bytes to `${:04X}` would exceed the maximum zero-page address of `${:04X}`", address, u8::MAX))
        }
    }
}
