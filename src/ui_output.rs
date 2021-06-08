use crate::data::{GameLocation, UiOutputInstruction};
use crossterm::{
    cursor::{Hide, MoveTo},
    style::Print,
    terminal::{Clear, ClearType},
    QueueableCommand, Result,
};
use std::io::{stdout, Write};

const PLAYER_CHARACTER: char = '#';
const NPC_CHARACTER: char = '*';

pub fn init(player_location: &GameLocation) -> Result<()> {
    stdout()
        .queue(Hide)?
        .queue(MoveTo(0, 0))?
        .queue(Clear(ClearType::All))?;

    show_location(player_location, PLAYER_CHARACTER)?;

    stdout().flush()?;
    Ok(())
}

pub fn process_instructions(instructions: &[UiOutputInstruction]) -> Result<()> {
    for ins in instructions {
        process_inst(ins)?;
    }
    stdout().flush()?;
    Ok(())
}

pub fn process_instruction(instruction: &UiOutputInstruction) -> Result<()> {
    process_inst(instruction)?;
    stdout().flush()?;
    Ok(())
}

pub fn process_inst(instruction: &UiOutputInstruction) -> Result<()> {
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
    stdout().queue(MoveTo(x, y))?.queue(Print(" "))?;
    Ok(())
}

fn show_location(d: &GameLocation, character: char) -> Result<()> {
    let (x, y) = d.get_xy();
    stdout().queue(MoveTo(x, y))?.queue(Print(character))?;
    Ok(())
}
