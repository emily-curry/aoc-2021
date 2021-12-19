/// A static list of the 24 possible rotations.
/// The way I am thinking about this is that there are 4 possible rotations around an axis.
/// If we always apply those transformations to the "primary" axis, there are 6 possible ways that
/// we can rotate the scanner first (rotate so that A, B, or C becomes the primary axis, then flip or not)
/// so that those 4 possible rotations produce a unique net rotation.
pub const ROTATIONS: [Rotation; 24] = [
    // No initial transform
    Rotation::new(S::A(false), S::B(false), S::C(false)),
    Rotation::new(S::A(false), S::C(true), S::B(false)),
    Rotation::new(S::A(false), S::B(true), S::C(true)),
    Rotation::new(S::A(false), S::C(false), S::B(true)),
    // Flip primary (flip = rotate 180deg along whatever is now the C axis)
    Rotation::new(S::A(true), S::B(true), S::C(false)),
    Rotation::new(S::A(true), S::C(false), S::B(false)),
    Rotation::new(S::A(true), S::B(false), S::C(true)),
    Rotation::new(S::A(true), S::C(true), S::B(true)),
    // Rotate B to primary
    Rotation::new(S::B(false), S::C(false), S::A(false)),
    Rotation::new(S::B(false), S::A(true), S::C(false)),
    Rotation::new(S::B(false), S::C(true), S::A(true)),
    Rotation::new(S::B(false), S::A(false), S::C(true)),
    // Rotate B to primary, then flip
    Rotation::new(S::B(true), S::C(true), S::A(false)),
    Rotation::new(S::B(true), S::A(false), S::C(false)),
    Rotation::new(S::B(true), S::C(false), S::A(true)),
    Rotation::new(S::B(true), S::A(true), S::C(true)),
    // Rotate C to primary
    Rotation::new(S::C(false), S::A(false), S::B(false)),
    Rotation::new(S::C(false), S::B(true), S::A(false)),
    Rotation::new(S::C(false), S::A(true), S::B(true)),
    Rotation::new(S::C(false), S::B(false), S::A(true)),
    // Rotate C to primary, then flip
    Rotation::new(S::C(true), S::A(true), S::B(false)),
    Rotation::new(S::C(true), S::B(false), S::A(false)),
    Rotation::new(S::C(true), S::A(false), S::B(true)),
    Rotation::new(S::C(true), S::B(true), S::A(true)),
];

pub struct Rotation {
    pub a: RotationAxis,
    pub b: RotationAxis,
    pub c: RotationAxis,
}

impl Rotation {
    pub const fn new(a: RotationAxis, b: RotationAxis, c: RotationAxis) -> Self {
        Rotation { a, b, c }
    }
}

pub enum RotationAxis {
    A(bool),
    B(bool),
    C(bool),
}

type S = RotationAxis;

#[cfg(test)]
mod tests {
    use crate::rotation::{RotationAxis, ROTATIONS};

    /// Validation that the approach in ROTATIONS is correct.
    #[test]
    fn rotation_validate() {
        let initial = (1, 2, 3);
        let mut set: Vec<(i32, i32, i32)> = Vec::new();
        for rotation in ROTATIONS {
            let a = rotate(&initial, &rotation.a);
            let b = rotate(&initial, &rotation.b);
            let c = rotate(&initial, &rotation.c);
            let next = (a, b, c);
            set.push(next);
        }
        assert_eq!(set.len(), 24); // assert no duplicates
        for (a, b, c) in set {
            assert_ne!(a.abs(), b.abs());
            assert_ne!(a.abs(), c.abs());
            assert_ne!(b.abs(), c.abs());
        }
    }

    fn rotate(initial: &(i32, i32, i32), rotate: &RotationAxis) -> i32 {
        match rotate {
            RotationAxis::A(flip) => {
                if *flip {
                    initial.0 * -1
                } else {
                    initial.0
                }
            }
            RotationAxis::B(flip) => {
                if *flip {
                    initial.1 * -1
                } else {
                    initial.1
                }
            }
            RotationAxis::C(flip) => {
                if *flip {
                    initial.2 * -1
                } else {
                    initial.2
                }
            }
        }
    }
}
