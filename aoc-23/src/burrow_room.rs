use crate::amphipod::Amphipod;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub struct BurrowRoom {
    pub x: u8,
    pub y: u8,
}

impl BurrowRoom {
    pub const fn new(x: u8, y: u8) -> Self {
        BurrowRoom { x, y }
    }

    pub const fn is_home(&self) -> Option<Amphipod> {
        if self.y == 0 {
            return None;
        }
        match self {
            BurrowRoom { x: 2, y: _ } => Some(Amphipod::Amber),
            BurrowRoom { x: 4, y: _ } => Some(Amphipod::Bronze),
            BurrowRoom { x: 6, y: _ } => Some(Amphipod::Copper),
            BurrowRoom { x: 8, y: _ } => Some(Amphipod::Desert),
            _ => None,
        }
    }

    pub const fn is_outside_home(&self) -> bool {
        if self.y != 0 {
            false
        } else {
            self.x == 2 || self.x == 4 || self.x == 6 || self.x == 8
        }
    }
}
