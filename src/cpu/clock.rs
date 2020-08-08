pub struct Clock {
    speed: u32,
    cycles: u64,
}

impl Clock {
    pub fn new(mode: ClockMode) -> Self {
        Self {
            speed: Self::determine_speed(mode),
            cycles: 0,
        }
    }

    pub fn tick(&mut self) {
        self.cycles += 1;
    }

    fn determine_speed(mode: ClockMode) -> u32 {
        match mode {
            ClockMode::Ntsc => 1 / 1_789_773,
            ClockMode::Pal => 1 / 1_662_607,
            ClockMode::Dendy => 1 / 1_773_448,
        }
    }
}

#[derive(Copy, Clone)]
pub enum ClockMode {
    Ntsc,
    Pal,
    Dendy,
}
