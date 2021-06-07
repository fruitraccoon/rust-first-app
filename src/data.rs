pub enum MovementDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone)]
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
        to: &'a GameLocation,
    },
}

pub struct GameData {
    bounds_xy: (u16, u16),
    player_loc: GameLocation,
    npc_loc: GameLocation,
}

impl GameData {
    pub fn new(bounds_xy: (u16, u16), player_loc: GameLocation) -> GameData {
        GameData {
            bounds_xy,
            player_loc,
            npc_loc: GameLocation {
                xy: (bounds_xy.0, 2),
            },
        }
    }

    pub fn move_player(&mut self, direction: MovementDirection) -> UiOutputInstruction {
        let xy = self.player_loc.xy;
        self.player_loc = apply_direction(direction, &self.player_loc, self.bounds_xy);
        UiOutputInstruction::MovePlayer {
            from: GameLocation { xy },
            to: &self.player_loc,
        }
    }

    pub fn move_npcs(&mut self) -> UiOutputInstruction {
        let xy = self.npc_loc.xy;
        self.npc_loc = apply_direction(MovementDirection::Left, &self.npc_loc, self.bounds_xy);
        UiOutputInstruction::MoveNpc {
            from: GameLocation { xy },
            to: &self.npc_loc,
        }
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
