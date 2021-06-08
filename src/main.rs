mod data;
mod ui_input;
mod ui_output;

use data::{GameData, GameLocation, MovementDirection};
use rand::Rng;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub enum GameCommand {
    Quit,
    MovePlayer(MovementDirection),
    GameTick,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let player_initial_location = GameLocation::new(0, 0);
    let bounds = (40, 10);
    let mut gd = GameData::new(bounds, player_initial_location);
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || ui_input::listen_for_player_input(tx).unwrap());
    thread::spawn(move || listen_for_game_ticks(tx2).unwrap());

    ui_output::init(bounds, &player_initial_location)?;

    apply_game_commands(rx, &mut gd)?;

    Ok(())
}

fn listen_for_game_ticks(
    tx: mpsc::Sender<GameCommand>,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    loop {
        thread::sleep(Duration::from_millis(100));
        tx.send(GameCommand::GameTick)?;
    }
}

fn apply_game_commands(
    rx: mpsc::Receiver<GameCommand>,
    gd: &mut GameData,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    for cmd in rx.iter() {
        match cmd {
            GameCommand::MovePlayer(dir) => {
                let ins = gd.move_player(dir);
                ui_output::process_instruction(&ins)?;
            }
            GameCommand::GameTick => {
                let instructions = gd.move_npcs();
                ui_output::process_instructions(&instructions)?;

                if rand::thread_rng().gen_range(1..=2) == 1 {
                    gd.spawn_npc(&|x| rand::thread_rng().gen_range(0..x));
                }
            }
            GameCommand::Quit => break,
        }

        if gd.player_collided() {
            break;
        }
    }

    Ok(())
}
