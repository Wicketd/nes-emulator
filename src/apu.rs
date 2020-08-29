use crate::bus::{DeviceRead, DeviceWrite};

pub struct Apu {}

impl Apu {
    pub fn new() -> Self {
        Self {}
    }
}

impl DeviceRead for Apu {
    fn device_read(&self, address: u16) -> u8 {
        0
    }
}

impl DeviceWrite for Apu {
    fn device_write(&mut self, address: u16, value: u8) {
    }
}
