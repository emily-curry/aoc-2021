use crate::burrow_room::BurrowRoom;

//    01234567890
//   #############
// 0 #...........#
// 1 ###C#B#A#D###
// 2   #B#C#D#A#
//     #########
/// We can statically define the set of possible points.
pub const MAP: [BurrowRoom; 19] = [
    BurrowRoom::new(0, 0),
    BurrowRoom::new(1, 0),
    BurrowRoom::new(2, 0),
    BurrowRoom::new(2, 1),
    BurrowRoom::new(2, 2),
    BurrowRoom::new(3, 0),
    BurrowRoom::new(4, 0),
    BurrowRoom::new(4, 1),
    BurrowRoom::new(4, 2),
    BurrowRoom::new(5, 0),
    BurrowRoom::new(6, 0),
    BurrowRoom::new(6, 1),
    BurrowRoom::new(6, 2),
    BurrowRoom::new(7, 0),
    BurrowRoom::new(8, 0),
    BurrowRoom::new(8, 1),
    BurrowRoom::new(8, 2),
    BurrowRoom::new(9, 0),
    BurrowRoom::new(10, 0),
];

const fn targ(room: BurrowRoom) -> [BurrowRoom; 14] {
    let mut targets: [BurrowRoom; 14] = [BurrowRoom::new(255, 255); 14];
    let mut i = 0usize;
    let mut k = 0usize;
    while k < MAP.len() {
        let target = &MAP[k];
        if target.x != room.x && target.y != room.y && !target.is_outside_home() {
            targets[i] = BurrowRoom::new(target.x, target.y);
            i += 1;
        }
        k += 1;
    }
    targets
}

/// Similar to `MAP`, we can statically define the set of all possible from->to paths.
pub const MAP_PATHS: [(BurrowRoom, &'static [BurrowRoom]); 15] = [
    (BurrowRoom::new(0, 0), &targ(BurrowRoom::new(0, 0))),
    (BurrowRoom::new(1, 0), &targ(BurrowRoom::new(1, 0))),
    (BurrowRoom::new(2, 1), &targ(BurrowRoom::new(2, 1))),
    (BurrowRoom::new(2, 2), &targ(BurrowRoom::new(2, 2))),
    (BurrowRoom::new(3, 0), &targ(BurrowRoom::new(3, 0))),
    (BurrowRoom::new(4, 1), &targ(BurrowRoom::new(4, 1))),
    (BurrowRoom::new(4, 2), &targ(BurrowRoom::new(4, 2))),
    (BurrowRoom::new(5, 0), &targ(BurrowRoom::new(5, 0))),
    (BurrowRoom::new(6, 1), &targ(BurrowRoom::new(6, 1))),
    (BurrowRoom::new(6, 2), &targ(BurrowRoom::new(6, 2))),
    (BurrowRoom::new(7, 0), &targ(BurrowRoom::new(7, 0))),
    (BurrowRoom::new(8, 1), &targ(BurrowRoom::new(8, 1))),
    (BurrowRoom::new(8, 2), &targ(BurrowRoom::new(8, 2))),
    (BurrowRoom::new(9, 0), &targ(BurrowRoom::new(9, 0))),
    (BurrowRoom::new(10, 0), &targ(BurrowRoom::new(10, 0))),
];

pub const MAP_LARGE: [BurrowRoom; 27] = [
    BurrowRoom::new(0, 0),
    BurrowRoom::new(1, 0),
    BurrowRoom::new(2, 0),
    BurrowRoom::new(2, 1),
    BurrowRoom::new(2, 2),
    BurrowRoom::new(2, 3),
    BurrowRoom::new(2, 4),
    BurrowRoom::new(3, 0),
    BurrowRoom::new(4, 0),
    BurrowRoom::new(4, 1),
    BurrowRoom::new(4, 2),
    BurrowRoom::new(4, 3),
    BurrowRoom::new(4, 4),
    BurrowRoom::new(5, 0),
    BurrowRoom::new(6, 0),
    BurrowRoom::new(6, 1),
    BurrowRoom::new(6, 2),
    BurrowRoom::new(6, 3),
    BurrowRoom::new(6, 4),
    BurrowRoom::new(7, 0),
    BurrowRoom::new(8, 0),
    BurrowRoom::new(8, 1),
    BurrowRoom::new(8, 2),
    BurrowRoom::new(8, 3),
    BurrowRoom::new(8, 4),
    BurrowRoom::new(9, 0),
    BurrowRoom::new(10, 0),
];

const fn targ_lg(room: BurrowRoom) -> [BurrowRoom; 22] {
    let mut targets: [BurrowRoom; 22] = [BurrowRoom::new(255, 255); 22];
    let mut i = 0usize;
    let mut k = 0usize;
    while k < MAP_LARGE.len() {
        let target = &MAP_LARGE[k];
        if target.x != room.x && target.y != room.y && !target.is_outside_home() {
            targets[i] = BurrowRoom::new(target.x, target.y);
            i += 1;
        }
        k += 1;
    }
    targets
}

pub const MAP_PATHS_LARGE: [(BurrowRoom, &'static [BurrowRoom]); 23] = [
    (BurrowRoom::new(0, 0), &targ_lg(BurrowRoom::new(0, 0))),
    (BurrowRoom::new(1, 0), &targ_lg(BurrowRoom::new(1, 0))),
    (BurrowRoom::new(2, 1), &targ_lg(BurrowRoom::new(2, 1))),
    (BurrowRoom::new(2, 2), &targ_lg(BurrowRoom::new(2, 2))),
    (BurrowRoom::new(2, 3), &targ_lg(BurrowRoom::new(2, 3))),
    (BurrowRoom::new(2, 4), &targ_lg(BurrowRoom::new(2, 4))),
    (BurrowRoom::new(3, 0), &targ_lg(BurrowRoom::new(3, 0))),
    (BurrowRoom::new(4, 1), &targ_lg(BurrowRoom::new(4, 1))),
    (BurrowRoom::new(4, 2), &targ_lg(BurrowRoom::new(4, 2))),
    (BurrowRoom::new(4, 3), &targ_lg(BurrowRoom::new(4, 3))),
    (BurrowRoom::new(4, 4), &targ_lg(BurrowRoom::new(4, 4))),
    (BurrowRoom::new(5, 0), &targ_lg(BurrowRoom::new(5, 0))),
    (BurrowRoom::new(6, 1), &targ_lg(BurrowRoom::new(6, 1))),
    (BurrowRoom::new(6, 2), &targ_lg(BurrowRoom::new(6, 2))),
    (BurrowRoom::new(6, 3), &targ_lg(BurrowRoom::new(6, 3))),
    (BurrowRoom::new(6, 4), &targ_lg(BurrowRoom::new(6, 4))),
    (BurrowRoom::new(7, 0), &targ_lg(BurrowRoom::new(7, 0))),
    (BurrowRoom::new(8, 1), &targ_lg(BurrowRoom::new(8, 1))),
    (BurrowRoom::new(8, 2), &targ_lg(BurrowRoom::new(8, 2))),
    (BurrowRoom::new(8, 3), &targ_lg(BurrowRoom::new(8, 3))),
    (BurrowRoom::new(8, 4), &targ_lg(BurrowRoom::new(8, 4))),
    (BurrowRoom::new(9, 0), &targ_lg(BurrowRoom::new(9, 0))),
    (BurrowRoom::new(10, 0), &targ_lg(BurrowRoom::new(10, 0))),
];
