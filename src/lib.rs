#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate bitflags;

mod types;
mod bus;
mod cpu;

use types::Result;
use bus::Bus;
use cpu::Cpu;

pub fn run() -> Result {
    let bus = Bus::new();
    let mut cpu = Cpu::new(bus)?;
    cpu.start()?;

    Ok(())
}
