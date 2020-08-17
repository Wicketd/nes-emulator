#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate getset;
#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate log;

#[rustfmt::skip]
mod macros;
mod types;
mod bus;
mod rom;
mod ppu;

use types::Result;
use bus::{Bus, BusDevice};
use ppu::Ppu;

pub fn run() -> Result {
    // TODO: remove me
    std::env::set_var("RUST_LOG", "info");

    pretty_env_logger::init();

    let mut bus = Bus::new();

    let ppu = Ppu::new();
    bus.connect_device(BusDevice::Ppu(ppu), 0x2000, 0x3FFF);

    Ok(())
}
