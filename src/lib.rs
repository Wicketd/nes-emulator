#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate enum_dispatch;
#[macro_use]
extern crate getset;
#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate log;

#[rustfmt::skip]
mod macros;
mod bus;
mod apu;
mod ppu;
mod rom;
mod types;

use types::Result;
use bus::Bus;
use apu::Apu;
use ppu::Ppu;
use rom::Rom;

pub fn run() -> Result {
    // TODO: remove me
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let mut bus = Bus::new(
        Ppu::new(),
        Apu::new(),
        // TODO: remove me
        Rom::from_file(std::path::Path::new("test"))?
    );

    Ok(())
}
