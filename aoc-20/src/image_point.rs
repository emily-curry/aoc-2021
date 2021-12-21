use std::cmp::Ordering;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct ImagePoint {
    pub x: i64,
    pub y: i64,
}

impl ImagePoint {
    pub fn new(x: i64, y: i64) -> Self {
        ImagePoint { x, y }
    }

    pub fn get_points_around(&self) -> Vec<ImagePoint> {
        vec![
            ImagePoint::new(self.x - 1, self.y - 1),
            ImagePoint::new(self.x, self.y - 1),
            ImagePoint::new(self.x + 1, self.y - 1),
            ImagePoint::new(self.x - 1, self.y),
            ImagePoint::new(self.x, self.y),
            ImagePoint::new(self.x + 1, self.y),
            ImagePoint::new(self.x - 1, self.y + 1),
            ImagePoint::new(self.x, self.y + 1),
            ImagePoint::new(self.x + 1, self.y + 1),
        ]
    }
}

impl PartialOrd for ImagePoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// ~~Sorts first by y, then by x (lowest first). This will allow us to iterate over a BTreeMap in a natural order.~~
/// This ended up not being relevant, as the solution I went with didn't rely on keeping track of all points, just the important+enabled points.
impl Ord for ImagePoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

#[cfg(test)]
mod tests {
    use crate::image_point::ImagePoint;
    use std::collections::BTreeMap;

    #[test]
    fn compare() {
        assert!(ImagePoint::new(9, 0) < ImagePoint::new(10, 0)); // If y is the same, cmp x.
        assert!(ImagePoint::new(10, 0) < ImagePoint::new(9, 1)); // cmp y first.

        let mut map: BTreeMap<ImagePoint, bool> = BTreeMap::new();
        map.insert(ImagePoint::new(9, 1), true);
        map.insert(ImagePoint::new(11, 0), false);
        map.insert(ImagePoint::new(10, 1), true);
        map.insert(ImagePoint::new(-10, -10), false);

        let mut iter = map.iter();
        assert_eq!(iter.next().unwrap().0, &ImagePoint::new(-10, -10));
        assert_eq!(iter.next().unwrap().0, &ImagePoint::new(11, 0));
        assert_eq!(iter.next().unwrap().0, &ImagePoint::new(9, 1));
        assert_eq!(iter.next().unwrap().0, &ImagePoint::new(10, 1));
    }
}
