use color_eyre::{
    Result,
    eyre::{Ok, eyre},
};
use std::env;

mod app;
mod log_view;

fn parse_cmd_args() -> Result<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(eyre!("bad cmd args, provide log file/directory path"));
    }
    Ok(args[1].clone())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let log_path = parse_cmd_args()?;

    let mut terminal = ratatui::init();
    terminal.clear().unwrap();
    app::App::new(log_path).run(&mut terminal)
}
