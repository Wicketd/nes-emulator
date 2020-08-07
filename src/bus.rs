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
        if let Some(_) = address.checked_add(1) {
            let bytes = [self.read(address), self.read(address + 1)];
            Ok(u16::from_le_bytes(bytes))
        } else {
            Err(anyhow!("address out of bounds"))
        }
    }
}
