use aoc_core::intmap::IntMapPoint;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct RiskMapPoint {
    pub point: IntMapPoint,
    pub risk: u64,
}

impl RiskMapPoint {
    pub fn new(point: IntMapPoint, risk: u64) -> Self {
        RiskMapPoint { point, risk }
    }
}

impl Eq for RiskMapPoint {}

impl PartialEq<Self> for RiskMapPoint {
    fn eq(&self, other: &Self) -> bool {
        other.point == self.point
    }
}

impl PartialOrd for RiskMapPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RiskMapPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .risk
            .cmp(&self.risk)
            .then(other.point.0.cmp(&self.point.0))
            .then(other.point.1.cmp(&self.point.1))
    }
}
