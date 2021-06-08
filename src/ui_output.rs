use crate::data::{GameLocation, UiOutputInstruction};
use crossterm::{
    cursor::{Hide, MoveTo, RestorePosition, SavePosition},
    style::Print,
    terminal::{Clear, ClearType},
    QueueableCommand, Result,
};
use std::io::{stdout, Write};

const PLAYER_CHARACTER: char = '#';
const NPC_CHARACTER: char = '*';

pub fn init(bounds_xy: (u16, u16), player_location: &GameLocation) -> Result<()> {
    stdout()
        .queue(Hide)?
        .queue(MoveTo(0, 0))?
        .queue(Clear(ClearType::All))?;

    draw_chrome(bounds_xy)?;
    show_location(player_location, PLAYER_CHARACTER)?;

    stdout().flush()?;
    Ok(())
}

pub fn process_instructions(instructions: &[UiOutputInstruction]) -> Result<()> {
    for ins in instructions {
        process_inst(ins)?;
    }
    finally()
}

pub fn process_instruction(instruction: &UiOutputInstruction) -> Result<()> {
    process_inst(instruction)?;
    finally()
}

fn finally() -> Result<()> {
    // Restore the cursor position to set the next write position, such as when the game ends
    stdout().queue(RestorePosition)?.flush()?;
    Ok(())
}

fn process_inst(instruction: &UiOutputInstruction) -> Result<()> {
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
    let (x, y) = apply_offset(d).get_xy();
    stdout().queue(MoveTo(x, y))?.queue(Print(" "))?;
    Ok(())
}

fn show_location(d: &GameLocation, character: char) -> Result<()> {
    let (x, y) = apply_offset(d).get_xy();
    stdout().queue(MoveTo(x, y))?.queue(Print(character))?;
    Ok(())
}

fn apply_offset(gl: &GameLocation) -> GameLocation {
    let (x, y) = gl.get_xy();
    GameLocation::new(x, y + 1)
}

fn draw_chrome((x_bound, y_bound): (u16, u16)) -> Result<()> {
    let mut out = stdout();
    out.queue(MoveTo(0, 0))?
        .queue(Print(str::repeat("-", x_bound.into())))?;
    out.queue(MoveTo(0, y_bound + 1))?
        .queue(Print(str::repeat("-", x_bound.into())))?;
    out.queue(MoveTo(0, y_bound + 3))?.queue(SavePosition)?;
    Ok(())
}
