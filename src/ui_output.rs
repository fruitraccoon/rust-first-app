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

pub fn process_instructions(instructions: &[UiOutputInstruction]) -> Result<()> {
    for ins in instructions {
        process_instruction(ins)?;
    }
    Ok(())
}

pub fn process_instruction(instruction: &UiOutputInstruction) -> Result<()> {
    match instruction {
        UiOutputInstruction::MovePlayer { from, to } => show_player(&from, to),
        UiOutputInstruction::MoveNpc { from, to } => show_player(&from, to),
    }
}

fn show_player(from: &GameLocation, to: &GameLocation) -> Result<()> {
    clear_location(from)?;
    show_location(to)
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
