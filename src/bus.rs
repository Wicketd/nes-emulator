use crate::apu::Apu;
use crate::ppu::Ppu;
use crate::rom::Rom;

pub struct Bus {
    ppu: Ppu,
    apu: Apu,
    rom: Rom,
}

impl Bus {
    pub fn new(ppu: Ppu, apu: Apu, rom: Rom) -> Self {
        Self { ppu, apu, rom }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.select_device(address).device_read(address)
    }

    fn select_device(&self, address: u16) -> &dyn DeviceRead {
        match address {
            (0x2000..=0x3FFF) => &self.ppu as &dyn DeviceRead,
            (0x4000..=0x401F) => &self.apu as &dyn DeviceRead,
            (0x4200..=0xFFFF) => &self.rom as &dyn DeviceRead,
            _ => unimplemented!("no readable device connected to address `${:04X}`", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.select_device_mut(address).device_write(address, value)
    }

    fn select_device_mut(&mut self, address: u16) -> &mut dyn DeviceWrite {
        match address {
            (0x2000..=0x3FFF) => &mut self.ppu as &mut dyn DeviceWrite,
            (0x4000..=0x401F) => &mut self.apu as &mut dyn DeviceWrite,
            _ => unimplemented!("no writable device connected to address `${:04X}`", address),
        }
    }
}

pub trait DeviceRead {
    fn device_read(&self, address: u16) -> u8;
}

pub trait DeviceWrite {
    fn device_write(&mut self, address: u16, value: u8);
}
