use color_eyre::{Result, eyre::Ok};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::{Block, Paragraph, Widget},
};

use crate::log_view::LogView;

pub struct App {
    log_view: LogView,
    // TODO: focus dispatcher
}

impl App {
    // accept a single file or directory
    pub fn new(log_path: String) -> Self {
        Self {
            log_view: LogView::new(log_path),
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break Ok(());
                }
                self.log_view.handle_events(key)?;
            }
        }
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let areas = Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(area);

        let explorer_view =
            Paragraph::new("log files...").block(Block::bordered().title("Directory name".bold()));
        explorer_view.render(areas[0], buf);

        self.log_view.render(areas[1], buf);
    }
}
