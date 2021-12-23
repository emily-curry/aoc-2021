use crate::cuboid_cube::CuboidCube;

pub struct CuboidSet {
    cubes: Vec<CuboidCube>,
}

impl CuboidSet {
    pub fn new(cubes: Vec<CuboidCube>) -> Self {
        CuboidSet::assert_non_overlapping(&cubes);
        CuboidSet { cubes }
    }

    pub fn push(&mut self, other: CuboidCube) {
        self.cubes.push(other);
    }

    /// Updates self to contain the set of cuboids that represents all cuboids in the current set **and** the cuboids in `other`.
    pub fn union(&mut self, other: &CuboidCube) {
        self.intersect(other);
        self.push(other.clone());
    }

    /// Updates self to contain the set of cuboids that represents all cuboids in the current set **except** the cuboids in `other`.
    pub fn intersect(&mut self, other: &CuboidCube) {
        self.cubes = self
            .cubes
            .iter()
            .flat_map(|cube| cube.intersect(other))
            .collect()
    }

    pub fn volume(&self) -> usize {
        self.cubes.iter().map(|cube| cube.volume()).sum()
    }

    #[cfg(debug_assertions)]
    fn assert_non_overlapping(cubes: &Vec<CuboidCube>) {
        for lhs in cubes {
            for rhs in cubes {
                if lhs == rhs {
                    continue;
                }
                assert!(
                    !lhs.contains(rhs),
                    "Cubes must not overlap, but the following cubes do!\n{:?}\n{:?}",
                    lhs,
                    rhs
                );
            }
        }
    }

    #[cfg(not(debug_assertions))]
    fn assert_non_overlapping(_cubes: &Vec<CuboidCube>) {}
}

#[cfg(test)]
mod tests {
    use crate::cuboid_cube::CuboidCube;
    use crate::cuboid_set::CuboidSet;

    #[test]
    fn union() {
        let mut original = CuboidSet::new(vec![CuboidCube {
            x: -2..3,
            y: -2..3,
            z: -2..3,
        }]);
        assert_eq!(original.volume(), 125);
        let other_inner = CuboidCube {
            x: -1..1,
            y: -1..1,
            z: -1..1,
        };
        assert_eq!(other_inner.volume(), 8);

        original.union(&other_inner);
        assert_eq!(original.volume(), 125);

        let mut original = CuboidSet::new(vec![CuboidCube {
            x: -2..3,
            y: -2..3,
            z: -2..3,
        }]);
        let other_outer = CuboidCube {
            x: 3..5,
            y: 3..6,
            z: 3..4,
        };
        original.union(&other_outer);
        assert_eq!(original.volume(), 125 + 6);

        let mut original = CuboidSet::new(vec![CuboidCube {
            x: -2..3,
            y: -2..3,
            z: -2..3,
        }]);
        let other_intersect = CuboidCube {
            x: 0..5,
            y: 0..6,
            z: 0..4,
        };
        assert_eq!(other_intersect.volume(), 120);
        original.union(&other_intersect);
        assert_eq!(original.volume(), 125 + 120 - 27); // a 3x3x3 cube overlaps
    }
}
