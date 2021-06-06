use crate::data::MovementDirection;
use crate::GameCommand;
use crossterm::event::{read, Event, KeyCode};
use std::sync::mpsc;

pub fn listen_for_player_input(
    tx: mpsc::Sender<GameCommand>,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    loop {
        let command = match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Esc => GameCommand::Quit,
                KeyCode::Up => GameCommand::MovePlayer(MovementDirection::Up),
                KeyCode::Right => GameCommand::MovePlayer(MovementDirection::Right),
                KeyCode::Down => GameCommand::MovePlayer(MovementDirection::Down),
                KeyCode::Left => GameCommand::MovePlayer(MovementDirection::Left),
                _ => continue,
            },
            Event::Mouse(_) => continue,
            Event::Resize(_, _) => continue,
        };

        tx.send(command)?;
    }
}
