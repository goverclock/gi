use color_eyre::Result;
use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{self, Event};
use ratatui::text::{Line, Span};
use ratatui::{DefaultTerminal, Frame, widgets::Paragraph};
use std::env;
use std::fs;

struct App {
    file_paths: Vec<String>,
    contents: String,
}

impl App {
    fn new(file_paths: Vec<String>) -> Self {
        Self {
            file_paths,
            contents: String::from("FUCK"),
        }
    }

    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let contents = fs::read_to_string(&self.file_paths[1]).expect("should be a file");
        self.contents = contents;
        loop {
            terminal.draw(|frame| self.render(frame))?;
            if matches!(event::read()?, Event::Key(_)) {
                break Ok(());
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let p = Paragraph::new(self.contents.clone());

        frame.render_widget(p, frame.area());
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Ok(());
    }

    color_eyre::install()?;
    let mut terminal = ratatui::init();
    terminal.clear().unwrap();
    App::new(args).run(&mut terminal)
}
