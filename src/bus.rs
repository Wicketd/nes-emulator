use crate::types::Result;
use crate::ppu::Ppu;

#[derive(Error, Debug)]
pub enum BusError {
    #[error("no device found at address `${0:04X}`")]
    DeviceNotFound(u16),
}

pub struct Bus {
    connections: Vec<DeviceConnection>,
}

struct DeviceConnection {
    device: BusDevice,
    address_start: u16,
    address_end: u16,
}

#[derive(Debug)]
pub enum BusDevice {
    Ppu(Ppu),
}

impl Bus {
    pub fn new() -> Self {
        Self { connections: vec![] }
    }

    pub fn connect_device(&mut self, device: BusDevice, address_start: u16, address_end: u16) {
        info!("connecting device {:?} to address range `${:04X}..=${:04X}", device, address_start, address_end);
        self.connections.push(DeviceConnection { device, address_start, address_end });
    }

    pub fn read(&self, address: u16) -> Result<u8> {
        let result = match self.get_device(address)? {
            BusDevice::Ppu(device) => device.bus_read(address),
        };

        Ok(result)
    }

    fn get_device(&self, address: u16) -> Result<&BusDevice> {
        self.connections.iter().find_map(|connection| {
            if address >= connection.address_start && address <= connection.address_end {
                Some(&connection.device)
            } else {
                None
            }
        }).ok_or_else(|| BusError::DeviceNotFound(address).into())
    }

    pub fn write(&self, address: u16, value: u8) -> Result {
        match self.get_device_mut(address)? {
            BusDevice::Ppu(device) => device.bus_write(address, value),
        }

        Ok(())
    }

    fn get_device_mut(&mut self, address: u16) -> Result<&mut BusDevice> {
        self.connections.iter_mut().find_map(|connection| {
            if address >= connection.address_start && address <= connection.address_end {
                Some(&mut connection.device)
            } else {
                None
            }
        }).ok_or_else(|| BusError::DeviceNotFound(address).into())
    }
}
