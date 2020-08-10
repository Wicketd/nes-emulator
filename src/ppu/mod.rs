mod palette;

use crate::types::Result;
use self::palette::Palette;
use piston::{self, PistonWindow};

pub struct Ppu {
    window: PistonWindow,
    palette: Palette,
}

impl Ppu {
    pub fn new(window: PistonWindow) -> Result<Self> {
        Ok(Self {
            window,
            palette: Palette::new()?,
        })
    }

    pub fn start(&mut self) {
        while let Some(e) = self.window.next() {
            self.window.draw_2d(&e, |ctx, gfx, _| {
                piston::clear([0.5, 0.5, 0.5, 1.0], gfx);
                piston::rectangle(
                    [1.0, 0.0, 0.0, 1.0],
                    [0.0, 0.0, 100.0, 100.0],
                    ctx.transform,
                    gfx
                );
            });
        }
    }
}
