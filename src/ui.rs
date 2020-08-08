use tui::{
    Frame,
    Terminal,
    backend::Backend,
    layout::{Layout, Constraint, Direction, Rect},
    widgets::{Block, Borders, Gauge},
    style::{Style, Color, Modifier},
};
use crate::types::Result;

pub struct RuntimeUi<B: Backend> {
    terminal: Terminal<B>,
}

impl<B: Backend> RuntimeUi<B> {
    pub fn new(backend: B) -> Result<Self> {
        Ok(Self { terminal: Terminal::new(backend)? })
    }

    pub fn connect(&mut self) -> Result {
        Ok(self.terminal.clear()?)
    }

    pub fn render(&mut self) -> Result {
        Ok(self.terminal.draw(|f| Self::draw(f))?)
    }

    fn draw(f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(f.size());

        Self::draw_gauges_sync(f, chunks[0]);

        let block = Block::default()
            .title("Clock synchronization")
            .borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
    }

    fn draw_gauges_sync(f: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .constraints([Constraint::Length(2), Constraint::Length(1), Constraint::Length(2), Constraint::Min(0)].as_ref())
            .margin(2)
            .split(area);

        let gauge = Gauge::default()
            .block(Block::default().title("PPU frame buffer"))
            .gauge_style(
                Style::default()
                    .fg(Color::Green)
                    .bg(Color::Gray)
                    .add_modifier(Modifier::BOLD)
            )
            .label("116 / 120")
            .ratio(116.0 / 120.0);
        f.render_widget(gauge, chunks[0]);

        let gauge = Gauge::default()
            .block(Block::default().title("APU sample buffer"))
            .gauge_style(
                Style::default()
                .fg(Color::Yellow)
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD)
            )
            .label("281 / 256")
            .ratio((281.0 - 256.0) / 256.0);
        f.render_widget(gauge, chunks[2]);
    }
}
