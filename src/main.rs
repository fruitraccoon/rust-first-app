mod data {
    pub enum MovementDirection {
        Up,
        Right,
        Down,
        Left,
    }

    pub struct GameLocation {
        pub xy: (u16, u16),
    }

    pub struct GameData {
        bounds_xy: (u16, u16),
        player_loc: GameLocation,
    }

    impl GameData {
        pub fn new(bounds_xy: (u16, u16)) -> GameData {
            GameData {
                bounds_xy,
                player_loc: GameLocation { xy: (0, 0) },
            }
        }

        pub fn get_player_loc(&self) -> &GameLocation {
            &self.player_loc
        }

        pub fn move_player(&mut self, direction: MovementDirection) {
            self.player_loc = apply_direction(direction, &self.player_loc, self.bounds_xy);
        }
    }

    fn apply_direction(
        direction: MovementDirection,
        gl: &GameLocation,
        (x_max, y_max): (u16, u16),
    ) -> GameLocation {
        fn safe_inc(v: u16, max: u16) -> u16 {
            match v {
                v if v >= max => max,
                _ => v + 1,
            }
        }
        fn safe_dec(v: u16) -> u16 {
            match v {
                0 => 0,
                _ => v - 1,
            }
        }
        let (x, y) = gl.xy;
        let xy = match direction {
            MovementDirection::Up => (x, safe_dec(y)),
            MovementDirection::Right => (safe_inc(x, x_max), y),
            MovementDirection::Down => (x, safe_inc(y, y_max)),
            MovementDirection::Left => (safe_dec(x), y),
        };
        GameLocation { xy }
    }
}

mod ui_input {
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
}

mod ui_output {
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
}

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
