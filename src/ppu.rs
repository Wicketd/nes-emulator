use crate::bus::{DeviceRead, DeviceWrite};
use crate::constants::*;

pub struct Ppu {
    address_active: u16,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            address_active: 0,
        }
    }

    fn update_address_active(&mut self, value: u8) {
        self.address_active <<= 8;
        self.address_active += value as u16;
    }
}

impl DeviceRead for Ppu {
    fn device_read(&self, address: u16) -> u8 {
        0
    }
}

impl DeviceWrite for Ppu {
    fn device_write(&mut self, address: u16, value: u8) {
        match address {
            PPUADDR => self.update_address_active(value),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ppu() -> Ppu {
        Ppu::new()
    }

    #[test]
    fn set_address_active() {
        let mut ppu = ppu();

        ppu.device_write(PPUADDR, 0x22);
        assert_eq!(ppu.address_active, 0x0022);

        ppu.device_write(PPUADDR, 0x11);
        assert_eq!(ppu.address_active, 0x2211);

        ppu.device_write(PPUADDR, 0x00);
        assert_eq!(ppu.address_active, 0x1100);

        ppu.device_write(PPUADDR, 0x00);
        assert_eq!(ppu.address_active, 0x0000);
    }
}
