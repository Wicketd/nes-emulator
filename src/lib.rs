#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate getset;

mod types;
mod bus;
mod cpu;
mod ppu;

use types::Result;
use bus::Bus;
use cpu::Cpu;
use ppu::Ppu;
use piston::{PistonWindow, WindowSettings};

const WINDOW_TITLE: &str = "NES emulator";
const WINDOW_SIZE: (u32, u32) = (1024, 768);

pub fn run() -> Result {
    let bus = Bus::new();
    let mut cpu = Cpu::new(bus)?;
    cpu.start()?;

    let window = create_window()?;
    let mut ppu = Ppu::new(window)?;
    ppu.start();

    Ok(())
}

fn create_window() -> Result<PistonWindow> {
    let window = WindowSettings::new(WINDOW_TITLE, WINDOW_SIZE)
        .exit_on_esc(cfg!(debug_assertions))
        .build()
        .unwrap();

    Ok(window)
}
