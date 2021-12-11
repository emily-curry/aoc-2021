use aoc_core::intmap::{IntMap, IntMapPoint};
use std::collections::HashSet;

pub struct OctoMap {
    map: IntMap,
}

impl OctoMap {
    pub fn new(map: IntMap) -> Self {
        OctoMap { map }
    }

    /// Returns the number of elements in the map.
    pub fn get_size(&self) -> usize {
        self.map.get_width() * self.map.get_height()
    }

    /// Returns the total number of flashes that occurred on this step.
    pub fn step(&mut self) -> u64 {
        self.increment_all();
        self.flash()
    }

    fn increment_all(&mut self) {
        let points: Vec<IntMapPoint> = self.map.iter_points().collect();
        for point in points {
            self.increment_point(point.0, point.1);
        }
    }

    fn increment_point(&mut self, x: usize, y: usize) {
        let current_point = self.map.get_point(x, y);
        self.map.set_point(&(x, y, current_point.2 + 1));
    }

    /// Causes all octopi to flash according to the rules, resets all octopi that flashed, and returns the number of flashes that occurred on this step.
    fn flash(&mut self) -> u64 {
        let octopoints: Vec<(usize, usize)> = self.map.iter_points().map(|p| (p.0, p.1)).collect();
        let mut did_flash: HashSet<(usize, usize)> = HashSet::new();
        let mut total_flashes = 0u64;
        let mut pass_flashes = 0u64;
        // Loop until no flashes occur on a pass.
        loop {
            total_flashes += pass_flashes;
            pass_flashes = 0;
            for octo in &octopoints {
                // If the octopus already flashed in this invocation of flash, continue.
                if did_flash.contains(&octo) {
                    continue;
                }
                let map_point = self.map.get_point(octo.0, octo.1);
                if map_point.2 > 9 {
                    pass_flashes += 1;
                    did_flash.insert(*octo);
                    let adjacent_points = self.map.get_adjacent_points(octo.0, octo.1);
                    for adj in adjacent_points {
                        self.increment_point(adj.0, adj.1);
                    }
                }
            }

            if pass_flashes == 0 {
                break;
            }
        }

        for octo in did_flash {
            self.map.set_point(&(octo.0, octo.1, 0));
        }

        total_flashes
    }
}
