use crate::types::Result;

pub struct Bus {
    // TODO: replace with devices
    bytes: [u8; Self::LENGTH],
}

impl Bus {
    const LENGTH: usize = u16::MAX as usize + 1;

    pub fn new() -> Self {
        Self { bytes: [0; Self::LENGTH] }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.bytes[address as usize]
    }

    pub fn read_u16(&self, address: u16) -> Result<u16> {
        if address.checked_add(1).is_some() {
            let bytes = [self.read(address), self.read(address + 1)];
            Ok(u16::from_le_bytes(bytes))
        } else {
            Err(anyhow!("address out of bounds"))
        }
    }

    pub fn read_n(&self, address: u16, n: u16) -> Result<Vec<u8>> {
        if address.checked_add(n).is_some() {
            let mut bytes = vec![];

            for i in 0..n {
                bytes.push(self.read(address + i));
            }

            Ok(bytes)
        } else {
            Err(anyhow!("address + n out of bounds"))
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.bytes[address as usize] = value;
    }

    pub fn write_u16(&mut self, address: u16, value: u16) -> Result {
        if address.checked_add(1).is_some() {
            let bytes = value.to_le_bytes();
            self.write(address, bytes[0]);
            self.write(address + 1, bytes[1]);
            Ok(())
        } else {
            Err(anyhow!("address out of bounds"))
        }
    }
}
