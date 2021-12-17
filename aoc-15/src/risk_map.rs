use crate::risk_map_point::RiskMapPoint;
use aoc_core::intmap::{IntMap, IntMapPoint};
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug)]
pub struct RiskMap {
    pub risks: IntMap,
}

impl RiskMap {
    pub fn new(risks: IntMap) -> Self {
        RiskMap { risks }
    }

    pub fn get_path(&self) -> Option<u64> {
        let mut distances = self.make_distances();
        let mut visited: HashSet<IntMapPoint> = HashSet::new();
        let mut queue: BinaryHeap<RiskMapPoint> = BinaryHeap::new();
        let first_point = self.risks.get_point(0, 0);
        let first = RiskMapPoint::new(first_point, distances[&first_point]);
        queue.push(first);

        while let Some(risk_point) = queue.pop() {
            if risk_point.point.0 == self.risks.get_width() - 1
                && risk_point.point.1 == self.risks.get_height() - 1
            {
                return Some(risk_point.risk);
            }

            if visited.contains(&risk_point.point) || risk_point.risk > distances[&risk_point.point]
            {
                continue;
            }

            let adj_points_vec = self
                .risks
                .get_adjacent_points_cardinal(risk_point.point.0, risk_point.point.1);
            let adj_points = adj_points_vec.iter().filter(|x| !visited.contains(x));
            for adj in adj_points {
                let next = RiskMapPoint::new(*adj, risk_point.risk + adj.2 as u64);
                if next.risk < distances[&next.point] {
                    distances.insert(next.point, next.risk);
                    queue.push(next);
                }
            }

            visited.insert(risk_point.point);
        }

        None
    }

    fn make_distances(&self) -> HashMap<IntMapPoint, u64> {
        let mut distances = HashMap::new();
        for risk in self.risks.iter_points() {
            distances.insert(risk, u64::MAX);
        }
        distances.insert(self.risks.get_point(0, 0), 0);
        distances
    }
}
