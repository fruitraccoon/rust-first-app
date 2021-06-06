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
