mod data;
mod ui_input;
mod ui_output;

use crate::data::GameLocation;
use data::{GameData, MovementDirection};
use std::sync::mpsc;
use std::thread;

pub enum GameCommand {
    Quit,
    MovePlayer(MovementDirection),
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut gd = GameData::new((40, 10));
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || ui_input::listen_for_player_input(tx).unwrap());

    ui_output::init()?;
    ui_output::show_player(&GameLocation { xy: (0, 0) }, gd.get_player_loc())?;

    apply_game_commands(rx, &mut gd)?;

    Ok(())
}

fn apply_game_commands(
    rx: mpsc::Receiver<GameCommand>,
    gd: &mut GameData,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    for cmd in rx.iter() {
        match cmd {
            GameCommand::MovePlayer(dir) => {
                let from = gd.get_player_loc().xy;
                gd.move_player(dir);
                ui_output::show_player(&GameLocation { xy: from }, gd.get_player_loc())?;
            }
            GameCommand::Quit => break,
        }
    }

    Ok(())
}
