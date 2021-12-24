use crate::amphipod::Amphipod;
use crate::burrow_room::BurrowRoom;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct BurrowState {
    pub amphipods: BTreeMap<BurrowRoom, Amphipod>,
    pub cost: u32,
    y_max: u8,
}

impl BurrowState {
    pub fn from_input() -> Self {
        let mut amphipods: BTreeMap<BurrowRoom, Amphipod> = BTreeMap::new();
        amphipods.insert(BurrowRoom::new(2, 1), Amphipod::Copper);
        amphipods.insert(BurrowRoom::new(2, 2), Amphipod::Bronze);
        amphipods.insert(BurrowRoom::new(4, 1), Amphipod::Bronze);
        amphipods.insert(BurrowRoom::new(4, 2), Amphipod::Copper);
        amphipods.insert(BurrowRoom::new(6, 1), Amphipod::Amber);
        amphipods.insert(BurrowRoom::new(6, 2), Amphipod::Desert);
        amphipods.insert(BurrowRoom::new(8, 1), Amphipod::Desert);
        amphipods.insert(BurrowRoom::new(8, 2), Amphipod::Amber);
        BurrowState {
            amphipods,
            cost: 0,
            y_max: 2,
        }
    }

    pub fn from_complete() -> Self {
        let mut amphipods: BTreeMap<BurrowRoom, Amphipod> = BTreeMap::new();
        amphipods.insert(BurrowRoom::new(2, 1), Amphipod::Amber);
        amphipods.insert(BurrowRoom::new(2, 2), Amphipod::Amber);
        amphipods.insert(BurrowRoom::new(4, 1), Amphipod::Bronze);
        amphipods.insert(BurrowRoom::new(4, 2), Amphipod::Bronze);
        amphipods.insert(BurrowRoom::new(6, 1), Amphipod::Copper);
        amphipods.insert(BurrowRoom::new(6, 2), Amphipod::Copper);
        amphipods.insert(BurrowRoom::new(8, 1), Amphipod::Desert);
        amphipods.insert(BurrowRoom::new(8, 2), Amphipod::Desert);
        BurrowState {
            amphipods,
            cost: 0,
            y_max: 2,
        }
    }

    pub fn from_input_2() -> Self {
        let mut amphipods: BTreeMap<BurrowRoom, Amphipod> = BTreeMap::new();
        amphipods.insert(BurrowRoom::new(2, 1), Amphipod::Copper);
        amphipods.insert(BurrowRoom::new(2, 2), Amphipod::Desert);
        amphipods.insert(BurrowRoom::new(2, 3), Amphipod::Desert);
        amphipods.insert(BurrowRoom::new(2, 4), Amphipod::Bronze);
        amphipods.insert(BurrowRoom::new(4, 1), Amphipod::Bronze);
        amphipods.insert(BurrowRoom::new(4, 2), Amphipod::Copper);
        amphipods.insert(BurrowRoom::new(4, 3), Amphipod::Bronze);
        amphipods.insert(BurrowRoom::new(4, 4), Amphipod::Copper);
        amphipods.insert(BurrowRoom::new(6, 1), Amphipod::Amber);
        amphipods.insert(BurrowRoom::new(6, 2), Amphipod::Bronze);
        amphipods.insert(BurrowRoom::new(6, 3), Amphipod::Amber);
        amphipods.insert(BurrowRoom::new(6, 4), Amphipod::Desert);
        amphipods.insert(BurrowRoom::new(8, 1), Amphipod::Desert);
        amphipods.insert(BurrowRoom::new(8, 2), Amphipod::Amber);
        amphipods.insert(BurrowRoom::new(8, 3), Amphipod::Copper);
        amphipods.insert(BurrowRoom::new(8, 4), Amphipod::Amber);
        BurrowState {
            amphipods,
            cost: 0,
            y_max: 4,
        }
    }

    pub fn from_complete_2() -> Self {
        let mut r = BurrowState::from_complete();
        r.amphipods.insert(BurrowRoom::new(2, 3), Amphipod::Amber);
        r.amphipods.insert(BurrowRoom::new(2, 4), Amphipod::Amber);
        r.amphipods.insert(BurrowRoom::new(4, 3), Amphipod::Bronze);
        r.amphipods.insert(BurrowRoom::new(4, 4), Amphipod::Bronze);
        r.amphipods.insert(BurrowRoom::new(6, 3), Amphipod::Copper);
        r.amphipods.insert(BurrowRoom::new(6, 4), Amphipod::Copper);
        r.amphipods.insert(BurrowRoom::new(8, 3), Amphipod::Desert);
        r.amphipods.insert(BurrowRoom::new(8, 4), Amphipod::Desert);
        r.y_max = 4;
        r
    }

    pub fn apply_path(&self, path: (BurrowRoom, BurrowRoom, u32)) -> Self {
        let mut next = self.clone();
        let amphi = next
            .amphipods
            .remove(&path.0)
            .expect("Tried to move from a room that didn't have anyone in it!");
        next.cost += path.2 * amphi.cost();
        next.amphipods.insert(path.1, amphi);
        next
    }

    /// Returns whether or not moving `from` a room `to` another room is valid.
    /// If false, it means that the movement would violate the rules.
    pub fn is_path_valid(&self, a: &Amphipod, from: &BurrowRoom, to: &BurrowRoom) -> Option<u32> {
        // Cannot move into a home that does not belong to us.
        let dest_home = to.is_home();
        let is_dest_home_valid = dest_home.is_none() || &dest_home.unwrap() == a;
        if !is_dest_home_valid {
            return None;
        }
        // If moving into a home, we can't block a non-resident.
        if to.y > 0 && to.y < self.y_max {
            for i in (to.y + 1)..=self.y_max {
                let other_occupant = self.amphipods.get(&BurrowRoom::new(to.x, i));
                if other_occupant.is_some() && other_occupant.unwrap() != a {
                    return None;
                }
            }
        }
        // For each step we have to take, we can't cross another amphipod.
        let steps = self.get_steps(from, to);
        for step in &steps {
            if self.amphipods.contains_key(&step) {
                return None;
            }
        }

        Some(steps.len() as u32)
    }

    fn get_steps(&self, from: &BurrowRoom, to: &BurrowRoom) -> Vec<BurrowRoom> {
        let mut result = Vec::new();
        let mut step: BurrowRoom = *from;
        while step != *to {
            // Do we need to move out of the current home?
            if step.y > 0 && step.x != to.x {
                let next = BurrowRoom::new(step.x, step.y - 1);
                step = next;
                result.push(next);
            }
            // Do we need to move across the hallway?
            else if step.y == 0 && step.x != to.x {
                let next_x = if step.x > to.x {
                    step.x - 1
                } else {
                    step.x + 1
                };
                let next = BurrowRoom::new(next_x, step.y);
                step = next;
                result.push(next);
            }
            // Do we need to move into the home?
            else if step.y != to.y && step.x == to.x {
                let next = BurrowRoom::new(step.x, step.y + 1);
                step = next;
                result.push(next);
            } else {
                panic!("What happened? We started at {:?}, got to {:?}, but couldn't figure out how to get to {:?}", from, step, to);
            }
        }

        result
    }

    /// Returns whether or not the current room is occupied by a resident that should not move.
    pub fn is_finally_home(&self, room: &BurrowRoom) -> bool {
        if let Some(a) = self.amphipods.get(room) {
            if let Some(home) = room.is_home() {
                let is_matching_type = a == &home;
                if is_matching_type {
                    // If this is the bottom-most slot, or the bottom-most slot is occupied by a resident that belongs there, then we are finally home.
                    if room.y == self.y_max
                        || self.is_finally_home(&BurrowRoom::new(room.x, room.y + 1))
                    {
                        return true;
                    }
                }
            }
        }
        false
    }
}

impl PartialEq for BurrowState {
    fn eq(&self, other: &Self) -> bool {
        self.amphipods.eq(&other.amphipods)
    }
}

impl Eq for BurrowState {}

impl Hash for BurrowState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (room, amphi) in &self.amphipods {
            room.x.hash(state);
            room.y.hash(state);
            amphi.hash(state);
        }
    }
}

impl PartialOrd for BurrowState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BurrowState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
