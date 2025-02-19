mod crossterm_backend;
mod app;
mod ui;
mod metadata;
mod player;
mod explorer;
use crate::crossterm_backend::run;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    run()?;
    Ok(())
}