use std::{error::Error, time::Duration};

mod app;
mod crossterm;
mod git;
mod ui;
mod utils;

const TICK_RATE: Duration = Duration::from_millis(1500);

fn main() -> Result<(), Box<dyn Error>> {
    crate::crossterm::run(TICK_RATE)?;
    Ok(())
}
