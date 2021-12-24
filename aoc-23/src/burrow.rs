use crate::burrow_map::{MAP, MAP_LARGE, MAP_PATHS, MAP_PATHS_LARGE};
use crate::burrow_room::BurrowRoom;
use crate::burrow_state::BurrowState;
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug)]
pub struct Burrow {
    rooms: &'static [BurrowRoom],
    paths: &'static [(BurrowRoom, &'static [BurrowRoom])],
}

impl Burrow {
    const fn new(
        rooms: &'static [BurrowRoom],
        paths: &'static [(BurrowRoom, &'static [BurrowRoom])],
    ) -> Self {
        Burrow { rooms, paths }
    }

    pub const fn from_map() -> Self {
        Burrow::new(&MAP, &MAP_PATHS)
    }

    pub const fn from_map_large() -> Self {
        Burrow::new(&MAP_LARGE, &MAP_PATHS_LARGE)
    }

    pub fn go<'a>(&self, initial: BurrowState, complete: BurrowState) -> Option<BurrowState> {
        let mut states = BinaryHeap::new();
        let mut seen: HashSet<BurrowState> = HashSet::new();
        states.push(initial);

        while let Some(state) = states.pop() {
            if seen.contains(&state) {
                continue;
            }

            if state == complete {
                return Some(state);
            }

            states.extend(self.generate_next_states(&state));
            seen.insert(state);
        }

        None
    }

    /// Using our static list of possible paths, we use the state to filter down to the set that would be allowed by the rules.
    /// Then, we apply each of those paths to the state to get the set of next states.
    fn generate_next_states<'a>(
        &'a self,
        state: &'a BurrowState,
    ) -> impl Iterator<Item = BurrowState> + '_ {
        self.paths
            .iter()
            .flat_map(|path| path.1.iter().map(|to| (path.0, *to)))
            .filter(|(_, b)| b.x != 255 && b.y != 255)
            .map(|path| {
                let from_occupied = state.amphipods.contains_key(&path.0);
                if !from_occupied {
                    return None;
                }
                let to_occupied = state.amphipods.contains_key(&path.1);
                if to_occupied {
                    return None;
                }
                let already_settled_in = state.is_finally_home(&path.0);
                if already_settled_in {
                    return None;
                }
                let is_valid = state.is_path_valid(&state.amphipods[&path.0], &path.0, &path.1);
                if let Some(size) = is_valid {
                    Some((path.0, path.1, size))
                } else {
                    None
                }
            })
            .filter(Option::is_some)
            .map(|path| state.apply_path(path.unwrap()))
    }
}
