#[derive(Debug)]
pub struct Ppu {

}

impl Ppu {
    pub fn new() -> Self {
        Self {}
    }

    pub fn bus_read(&self, address: u16) -> u8 {
        0
    }

    pub fn bus_write(&mut self, address: u16, value: u8) {

    }
}
