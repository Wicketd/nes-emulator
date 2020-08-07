pub struct Clock {
    speed: u32,
}

impl Clock {
    pub fn new(mode: ClockMode) -> Self {
        Self { speed: Self::determine_speed(mode) }
    }

    fn determine_speed(mode: ClockMode) -> u32 {
        match mode {
            ClockMode::Ntsc => 1_789_773,
            ClockMode::Pal => 1_662_607,
            ClockMode::Dendy => 1_773_448,
        }
    }
}

#[derive(Copy, Clone)]
pub enum ClockMode {
    Ntsc,
    Pal,
    Dendy,
}
