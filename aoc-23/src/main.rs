mod amphipod;
mod burrow;
mod burrow_map;
mod burrow_room;
mod burrow_state;

use crate::burrow::Burrow;
use crate::burrow_state::BurrowState;
use std::time::SystemTime;

fn main() {
    let burrow = Burrow::from_map();
    let start = SystemTime::now();
    let result = burrow.go(BurrowState::from_input(), BurrowState::from_complete());
    println!(
        "Best way to sort all amphipods costs {} energy, found in {}s.",
        result.unwrap().cost,
        start.elapsed().unwrap().as_secs_f32()
    );

    let burrow = Burrow::from_map_large();
    let start = SystemTime::now();
    let result = burrow.go(BurrowState::from_input_2(), BurrowState::from_complete_2());
    println!(
        "Best way to really sort all amphipods costs {} energy, found in {}s.",
        result.unwrap().cost,
        start.elapsed().unwrap().as_secs_f32()
    );
}
