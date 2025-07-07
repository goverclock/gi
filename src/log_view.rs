use ratatui::{
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use std::{borrow::Cow, fs};

struct LogContents(String);

impl<'a> Into<Text<'a>> for LogContents {
    fn into(self) -> Text<'a> {
        let lines_with_number: Vec<_> = match self.0.into() {
            Cow::Borrowed("") => vec![Line::from("1 ")],
            Cow::Borrowed(s) => s
                .lines()
                .enumerate()
                .map(|(i, l)| Line::from(format!("{:<3}", i + 1) + l))
                .collect(),
            Cow::Owned(s) if s.is_empty() => vec![Line::from("1 ")],
            Cow::Owned(s) => s
                .lines()
                .enumerate()
                .map(|(i, l)| Line::from(format!("{:<3}", i + 1) + l))
                .collect(),
        };
        Text::from(lines_with_number)
    }
}

impl Clone for LogContents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

// the original, raw log view
pub struct LogView {
    log_path: String,
    contents: LogContents,
}

impl LogView {
    pub fn new(log_path: String) -> Self {
        let contents = fs::read_to_string(&log_path).unwrap();
        Self {
            log_path,
            contents: LogContents(contents),
        }
    }
}

impl Widget for &LogView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title = self.log_path.clone();
        let p = Paragraph::new(self.contents.clone())
            .block(Block::default().title(title.bold().into_centered_line()));
        p.render(area, buf);
    }
}
