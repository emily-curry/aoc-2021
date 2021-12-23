use std::ops::Range;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CuboidCube {
    pub x: Range<i32>,
    pub y: Range<i32>,
    pub z: Range<i32>,
}

impl CuboidCube {
    pub fn new(x: Range<i32>, y: Range<i32>, z: Range<i32>) -> Self {
        CuboidCube { x, y, z }
    }

    /// Returns the ranges of points that do occur in `self` but do not occur in `other`.
    pub fn intersect(&self, other: &CuboidCube) -> impl Iterator<Item = Self> {
        let x_min = CuboidCube {
            x: self.x.start..(other.x.start.min(self.x.end)),
            y: self.y.clone(),
            z: self.z.clone(),
        };
        let x_max = CuboidCube {
            x: (other.x.end.max(self.x.start))..self.x.end,
            y: self.y.clone(),
            z: self.z.clone(),
        };
        let y_min = CuboidCube {
            x: (x_min.x.end.max(self.x.start))..(x_max.x.start.min(self.x.end)),
            y: self.y.start..(other.y.start.min(self.y.end)),
            z: self.z.clone(),
        };
        let y_max = CuboidCube {
            x: (x_min.x.end.max(self.x.start))..(x_max.x.start.min(self.x.end)),
            y: (other.y.end.max(self.y.start))..self.y.end,
            z: self.z.clone(),
        };
        let z_min = CuboidCube {
            x: (x_min.x.end.max(self.x.start))..(x_max.x.start.min(self.x.end)),
            y: (y_min.y.end.max(self.y.start))..(y_max.y.start.min(self.y.end)),
            z: self.z.start..(other.z.start.min(self.z.end)),
        };
        let z_max = CuboidCube {
            x: (x_min.x.end.max(self.x.start))..(x_max.x.start.min(self.x.end)),
            y: (y_min.y.end.max(self.y.start))..(y_max.y.start.min(self.y.end)),
            z: (other.z.end.max(self.z.start))..self.z.end,
        };
        [x_min, x_max, y_min, y_max, z_min, z_max]
            .into_iter()
            .filter(|cube| cube.volume() > 0)
    }

    #[cfg(debug_assertions)]
    pub fn contains(&self, other: &CuboidCube) -> bool {
        (self.x.contains(&other.x.start) || self.x.contains(&(other.x.end - 1)))
            && (self.y.contains(&other.y.start) || self.y.contains(&(other.y.end - 1)))
            && (self.z.contains(&other.z.start) || self.z.contains(&(other.z.end - 1)))
    }

    pub fn volume(&self) -> usize {
        let w = self.x.len();
        let h = self.y.len();
        let d = self.z.len();
        w * h * d
    }
}

#[cfg(test)]
mod tests {
    use crate::cuboid_cube::CuboidCube;
    use crate::cuboid_set::CuboidSet;

    #[test]
    fn intersect() {
        let original = CuboidCube {
            x: -2..3,
            y: -2..3,
            z: -2..3,
        };
        assert_eq!(original.volume(), 125);
        let diff = CuboidCube {
            x: -1..1,
            y: -1..1,
            z: -1..1,
        };
        assert_eq!(diff.volume(), 8);

        let intersection = CuboidSet::new(original.intersect(&diff).collect());
        assert_eq!(intersection.volume(), 125 - 8);

        let diff_2 = CuboidCube {
            x: 2..18,
            y: -4..44,
            z: -1..1,
        };
        let intersection_2 = CuboidSet::new(original.intersect(&diff_2).collect());
        // (5 * 5 * 4) + (1 * 5 * 1) + (1 * 5 * 2)
        assert_eq!(intersection_2.volume(), 115);

        let other_intersect = CuboidCube {
            x: 0..5,
            y: 0..6,
            z: 0..4,
        };
        let intersection_3 = CuboidSet::new(other_intersect.intersect(&original).collect());
        assert_eq!(intersection_3.volume(), 120 - 27);
    }
}
