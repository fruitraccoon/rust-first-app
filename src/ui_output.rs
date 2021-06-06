use crate::data::GameLocation;
use crossterm::{
    cursor::{Hide, MoveTo},
    style::Print,
    terminal::{Clear, ClearType},
    ExecutableCommand, Result,
};
use std::io::stdout;

pub fn init() -> Result<()> {
    stdout()
        .execute(Hide)?
        .execute(MoveTo(0, 0))?
        .execute(Clear(ClearType::All))?;
    Ok(())
}

pub fn show_player(from: &GameLocation, to: &GameLocation) -> Result<()> {
    clear_location(from)?;
    show_location(to)?;
    Ok(())
}

fn clear_location(d: &GameLocation) -> Result<()> {
    let (x, y) = d.xy;
    stdout().execute(MoveTo(x, y))?.execute(Print(" "))?;
    Ok(())
}

fn show_location(d: &GameLocation) -> Result<()> {
    let (x, y) = d.xy;
    stdout().execute(MoveTo(x, y))?.execute(Print("&"))?;
    Ok(())
}
