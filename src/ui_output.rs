use crate::data::{GameLocation, UiOutputInstruction};
use crossterm::{
    cursor::{Hide, MoveTo},
    style::Print,
    terminal::{Clear, ClearType},
    ExecutableCommand, Result,
};
use std::io::stdout;

const PLAYER_CHARACTER: char = '#';
const NPC_CHARACTER: char = '*';

pub fn init(player_location: &GameLocation) -> Result<()> {
    stdout()
        .execute(Hide)?
        .execute(MoveTo(0, 0))?
        .execute(Clear(ClearType::All))?;

    show_location(player_location, PLAYER_CHARACTER)?;

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
        UiOutputInstruction::MovePlayer { from, to } => show_character(&from, to, PLAYER_CHARACTER),
        UiOutputInstruction::MoveNpc { from, to } => show_character(&from, &to, NPC_CHARACTER),
        UiOutputInstruction::RemoveNpc { from } => clear_location(from),
    }
}

fn show_character(from: &GameLocation, to: &GameLocation, character: char) -> Result<()> {
    clear_location(from)?;
    show_location(to, character)
}

fn clear_location(d: &GameLocation) -> Result<()> {
    let (x, y) = d.get_xy();
    stdout().execute(MoveTo(x, y))?.execute(Print(" "))?;
    Ok(())
}

fn show_location(d: &GameLocation, character: char) -> Result<()> {
    let (x, y) = d.get_xy();
    stdout().execute(MoveTo(x, y))?.execute(Print(character))?;
    Ok(())
}
