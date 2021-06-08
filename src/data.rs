pub enum MovementDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct GameLocation {
    xy: (u16, u16),
}

impl GameLocation {
    pub fn new(x: u16, y: u16) -> GameLocation {
        GameLocation { xy: (x, y) }
    }
    pub fn get_xy(&self) -> (u16, u16) {
        self.xy
    }
}

pub enum UiOutputInstruction<'a> {
    MovePlayer {
        from: GameLocation,
        to: &'a GameLocation,
    },
    MoveNpc {
        from: GameLocation,
        to: GameLocation,
    },
    RemoveNpc {
        from: GameLocation,
    },
}

pub struct GameData {
    bounds_xy: (u16, u16),
    player_loc: GameLocation,
    npc_locs: Vec<GameLocation>,
}

impl GameData {
    pub fn new(bounds_xy: (u16, u16), player_loc: GameLocation) -> GameData {
        GameData {
            bounds_xy,
            player_loc,
            npc_locs: vec![
                GameLocation {
                    xy: (bounds_xy.0, 2),
                },
                GameLocation {
                    xy: (bounds_xy.0, 6),
                },
            ],
        }
    }

    pub fn move_player(&mut self, direction: MovementDirection) -> UiOutputInstruction {
        let from = self.player_loc.clone();
        // If the player tries to move off the board, just ignore it
        if let Some(gl) = apply_direction(direction, &self.player_loc, self.bounds_xy) {
            self.player_loc = gl;
        }
        UiOutputInstruction::MovePlayer {
            from,
            to: &self.player_loc,
        }
    }

    pub fn move_npcs(&mut self) -> Vec<UiOutputInstruction> {
        let (new_locs, instructions): (Vec<Option<GameLocation>>, Vec<UiOutputInstruction>) = self
            .npc_locs
            .iter()
            .map(|n| {
                let from = n.clone();
                let to = apply_direction(MovementDirection::Left, n, self.bounds_xy);
                let inst = match to {
                    Some(to) => UiOutputInstruction::MoveNpc { from, to },
                    None => UiOutputInstruction::RemoveNpc { from },
                };
                (to, inst)
            })
            .unzip();
        self.npc_locs = new_locs.iter().filter_map(|x| *x).collect();
        instructions
    }

    pub fn player_collided(&self) -> bool {
        self.npc_locs.iter().any(|x| *x == self.player_loc)
    }
}

fn apply_direction(
    direction: MovementDirection,
    gl: &GameLocation,
    (x_max, y_max): (u16, u16),
) -> Option<GameLocation> {
    fn safe_inc(v: u16, max: u16) -> Option<u16> {
        match v {
            v if v >= max - 1 => None,
            _ => Some(v + 1),
        }
    }
    fn safe_dec(v: u16) -> Option<u16> {
        match v {
            0 => None,
            _ => Some(v - 1),
        }
    }
    let (x, y) = gl.xy;
    let xy = match direction {
        MovementDirection::Up => (Some(x), safe_dec(y)),
        MovementDirection::Right => (safe_inc(x, x_max), Some(y)),
        MovementDirection::Down => (Some(x), safe_inc(y, y_max)),
        MovementDirection::Left => (safe_dec(x), Some(y)),
    };
    match xy {
        (Some(x), Some(y)) => Some(GameLocation { xy: (x, y) }),
        _ => None,
    }
}
