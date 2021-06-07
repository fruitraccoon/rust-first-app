use crate::data::{GameLocation, UiOutputInstruction};
use crossterm::{
    cursor::{Hide, MoveTo},
    style::Print,
    terminal::{Clear, ClearType},
    ExecutableCommand, Result,
};
use std::io::stdout;

pub fn init(player_location: &GameLocation) -> Result<()> {
    stdout()
        .execute(Hide)?
        .execute(MoveTo(0, 0))?
        .execute(Clear(ClearType::All))?;

    show_location(player_location)?;

    Ok(())
}

pub fn process_instruction(i: UiOutputInstruction) -> Result<()> {
    match i {
        UiOutputInstruction::MovePlayer { from, to } => show_player(&from, to),
        UiOutputInstruction::MoveNpc { from, to } => show_player(&from, to),
    }
}

fn show_player(from: &GameLocation, to: &GameLocation) -> Result<()> {
    clear_location(from)?;
    show_location(to)?;
    Ok(())
}

fn clear_location(d: &GameLocation) -> Result<()> {
    let (x, y) = d.get_xy();
    stdout().execute(MoveTo(x, y))?.execute(Print(" "))?;
    Ok(())
}

fn show_location(d: &GameLocation) -> Result<()> {
    let (x, y) = d.get_xy();
    stdout().execute(MoveTo(x, y))?.execute(Print("&"))?;
    Ok(())
}
