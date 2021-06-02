mod data {
    pub enum MovementDirection {
        Up,
        Right,
        Down,
        Left,
    }

    pub enum GameCommand {
        Quit,
        MovePlayer(MovementDirection),
    }

    struct GameLocation {
        xy: (u16, u16),
    }

    impl GameLocation {
        fn apply(&mut self, direction: MovementDirection, (x_max, y_max): (u16, u16)) {
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
            let (x, y) = self.xy;
            self.xy = match direction {
                MovementDirection::Up => (x, safe_dec(y)),
                MovementDirection::Right => (safe_inc(x, x_max), y),
                MovementDirection::Down => (x, safe_inc(y, y_max)),
                MovementDirection::Left => (safe_dec(x), y),
            };
        }
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

        pub fn get_player_loc(&self) -> (u16, u16) {
            self.player_loc.xy
        }

        pub fn move_player(&mut self, direction: MovementDirection) {
            self.player_loc.apply(direction, self.bounds_xy);
        }
    }
}

use crossterm::{
    cursor::{Hide, MoveTo},
    event::{read, Event, KeyCode},
    style::Print,
    terminal::{Clear, ClearType},
    ExecutableCommand, Result,
};
use data::{GameCommand, GameData, MovementDirection};
use std::io::{stdout, Stdout};
use std::sync::mpsc;
use std::thread;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut out = stdout();
    let mut gd = GameData::new((40, 10));
    let (tx, rx) = mpsc::channel();

    hide_cursor(&mut out)?;
    clear_screen(&mut out)?;
    show_location(&mut out, &gd)?;

    thread::spawn(move || listen_for_player_input(tx).unwrap());

    apply_player_commands(rx, &mut out, &mut gd)?;

    Ok(())
}

fn listen_for_player_input(
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

fn apply_player_commands(
    rx: mpsc::Receiver<GameCommand>,
    out: &mut Stdout,
    gd: &mut GameData,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    loop {
        let received = rx.recv()?;
        match received {
            GameCommand::MovePlayer(dir) => gd.move_player(dir),
            GameCommand::Quit => break,
        }
        clear_screen(out)?;
        show_location(out, gd)?;
    }

    Ok(())
}

fn hide_cursor(out: &mut Stdout) -> Result<()> {
    out.execute(Hide)?;
    Ok(())
}

fn clear_screen(out: &mut Stdout) -> Result<()> {
    out.execute(MoveTo(0, 0))?.execute(Clear(ClearType::All))?;
    Ok(())
}

fn show_location(out: &mut Stdout, d: &GameData) -> Result<()> {
    let (player_x, player_y) = d.get_player_loc();
    out.execute(MoveTo(player_x, player_y))?
        .execute(Print("🛸"))?;
    Ok(())
}
