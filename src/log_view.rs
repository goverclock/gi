use color_eyre::eyre::{Ok, Result};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::Stylize,
    style::Style,
    widgets::{Block, Paragraph, Widget},
};
use std::fs;

struct NumberColumnWidget {
    upper_space: u16,
    displayed_start: u16,
    max_line_number: u16,
}

impl NumberColumnWidget {
    fn new(max_line_number: u16) -> Self {
        Self {
            upper_space: 1,
            displayed_start: 1,
            max_line_number,
        }
    }

    fn minimal_width_needed(&self, heigth: u16) -> u16 {
        (self.displayed_start + heigth - self.upper_space - 1)
            .to_string()
            .len() as u16
            + 1
    }

    fn scroll_up(&mut self, lines: u16) {
        if self.displayed_start == 1 {
            return;
        }
        self.displayed_start -= lines;
    }

    fn scroll_down(&mut self, lines: u16) {
        if self.displayed_start == self.max_line_number {
            return;
        }
        self.displayed_start += lines;
    }
}

impl Widget for &NumberColumnWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        if area.width < self.minimal_width_needed(area.height) {
            panic!(
                "fail to render NumberColumnWidget, {} columns needed, got {}",
                self.minimal_width_needed(area.height),
                area.width
            );
        }

        for rows in 0..area.height {
            if rows < self.upper_space {
                continue;
            }
            let displayed_number = self.displayed_start + rows - self.upper_space;
            if displayed_number > self.max_line_number {
                break;
            }
            buf[(area.x, area.y + rows)].set_symbol(&displayed_number.to_string());
            buf.set_string(
                area.x,
                area.y + rows,
                &displayed_number.to_string(),
                Style::default(),
            );
        }
    }
}

struct LogContentWidget {
    log_path: String,
    log_contents: String,
    line_number_start: u16,
}

impl LogContentWidget {
    fn new(log_path: String, log_contents: String) -> Self {
        Self {
            log_path,
            log_contents,
            line_number_start: 1,
        }
    }

    fn scroll_up(&mut self, lines: u16) {
        if self.line_number_start > lines {
            self.line_number_start -= lines;
        }
    }

    fn scroll_down(&mut self, lines: u16) {
        self.line_number_start += lines;
    }
}

impl Widget for &LogContentWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title = self.log_path.clone();
        let p = Paragraph::new(self.log_contents.clone())
            .block(Block::default().title(title.bold().into_centered_line()))
            .scroll((self.line_number_start - 1, 0));
        p.render(area, buf);
    }
}

// contains a number column, and contens of a log
// +--+------------------------+
// | 1| first line of a log    |
// | 2| second line of a log   |
// | 3| third line of a log    |
// |..| more lines...          |
// +--+------------------------+
pub struct LogView {
    number_column_widget: NumberColumnWidget,
    log_content_widget: LogContentWidget,
}

impl LogView {
    pub fn new(log_path: String) -> Self {
        let contents = fs::read_to_string(&log_path).unwrap();
        Self {
            log_content_widget: LogContentWidget::new(log_path, contents),
            number_column_widget: NumberColumnWidget::new(100 /* TODO: lines of the file */),
        }
    }

    pub fn handle_events(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('j') => {
                self.log_content_widget.scroll_down(1);
                self.number_column_widget.scroll_down(1);
            }
            KeyCode::Char('k') => {
                self.log_content_widget.scroll_up(1);
                self.number_column_widget.scroll_up(1);
            }
            // TODO: space and 'b', or C-d and C-u?
            _ => {}
        }

        Ok(())
    }
}

impl Widget for &LogView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let number_column_width = self.number_column_widget.minimal_width_needed(area.height);
        let areas =
            Layout::horizontal([Constraint::Length(number_column_width), Constraint::Fill(1)])
                .split(area);

        self.number_column_widget.render(areas[0], buf);
        self.log_content_widget.render(areas[1], buf);
    }
}
