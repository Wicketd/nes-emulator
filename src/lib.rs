#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate getset;

mod types;
mod bus;
mod cpu;
mod ui;

use types::Result;
use bus::Bus;
use cpu::Cpu;
use ui::RuntimeUi;
use tui::backend::CrosstermBackend;
use std::io;

pub fn run() -> Result {
    let mut _ui = {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        RuntimeUi::new(backend)?
    };
    // ui.connect()?;

    let bus = Bus::new();
    let mut cpu = Cpu::new(bus)?;
    cpu.start()?;

    Ok(())
}
