use std::ops::RangeInclusive;

const X_MIN: i32 = 117;
const X_MAX: i32 = 164;
const Y_MIN: i32 = -140;
const Y_MAX: i32 = -89;

fn main() {
    let launcher = ProbeLauncher::new(X_MIN..=X_MAX, Y_MIN..=Y_MAX);
    let probes = launcher.find_probe_vectors();
    let highest = probes
        .iter()
        .fold(0, |acc, val| if val.max_y > acc { val.max_y } else { acc });
    println!("Highest possible probe height: {:?}", highest);
    let count = probes.len();
    println!("Number of unique initial probe vectors: {:?}", count);
}

#[derive(Debug, Copy, Clone)]
struct ProbeState {
    x: i32,
    y: i32,
    velocity_x: i32,
    velocity_y: i32,
    max_y: i32,
}

impl ProbeState {
    pub fn new(velocity_x: i32, velocity_y: i32) -> Self {
        ProbeState {
            velocity_x,
            velocity_y,
            x: 0,
            y: 0,
            max_y: 0,
        }
    }
}

struct ProbeLauncher {
    x_bound: RangeInclusive<i32>,
    y_bound: RangeInclusive<i32>,
}

impl ProbeLauncher {
    pub fn new(x_bound: RangeInclusive<i32>, y_bound: RangeInclusive<i32>) -> Self {
        ProbeLauncher { x_bound, y_bound }
    }

    pub fn find_probe_vectors(&self) -> Vec<ProbeState> {
        let mut probes: Vec<ProbeState> = Vec::new();
        let min_vel_x = self.get_lowest_velocity_x();
        let max_vel_x = self.get_highest_velocity_x();
        let min_vel_y = self.get_lowest_velocity_y();
        let max_vel_y = self.get_highest_velocity_y();

        for velocity_y in min_vel_y..=max_vel_y {
            for velocity_x in min_vel_x..=max_vel_x {
                let probe = self.fire_probe(ProbeState::new(velocity_x, velocity_y));
                if let Some(p) = probe {
                    probes.push(p);
                }
            }
        }

        probes
    }

    pub fn fire_probe(&self, mut state: ProbeState) -> Option<ProbeState> {
        loop {
            // Update state values.
            state.x += state.velocity_x;
            state.y += state.velocity_y;
            if state.y > state.max_y {
                state.max_y = state.y;
            }
            state.velocity_x = match state.velocity_x {
                i32::MIN..=-1 => state.velocity_x + 1,
                0 => 0,
                1..=i32::MAX => state.velocity_x - 1,
            };
            state.velocity_y -= 1;

            // If in bounds, we've hit the target. Return.
            if self.x_bound.contains(&state.x) && self.y_bound.contains(&state.y) {
                return Some(state);
            }
            // If we've gone past, we've missed.
            if state.x > X_MAX || state.y < Y_MIN {
                break;
            }
        }

        None
    }

    /// Returns the first value of x that could possibly hit. Triangle numbers are back again!
    fn get_lowest_velocity_x(&self) -> i32 {
        let mut velocity_x = 1;
        while (1..=velocity_x).fold(0, |acc, val| acc + val) < *self.x_bound.start() {
            velocity_x += 1;
        }
        velocity_x
    }

    /// Returns the last value of x that could possibly hit.
    fn get_highest_velocity_x(&self) -> i32 {
        let mut highest_velocity_x = 1;
        let mut misses = 0;
        loop {
            let mut velocity_x = highest_velocity_x;
            let mut x = 0;
            let mut did_hit = false;
            while &x <= self.x_bound.end() {
                if self.x_bound.contains(&x) {
                    did_hit = true;
                }
                if velocity_x <= 0 {
                    break;
                }
                x += velocity_x;
                velocity_x -= 1;
            }
            highest_velocity_x += 1;
            if did_hit {
                misses = 0;
            } else {
                misses += 1;
            }
            if misses > X_MAX - X_MIN {
                break;
            }
        }
        highest_velocity_x
    }

    /// Returns the first value of y that could possibly hit.
    fn get_lowest_velocity_y(&self) -> i32 {
        Y_MIN
    }

    /// Returns the last value of y that could possibly hit. There is going to eventually be a value of y that skips over the entire y range.
    fn get_highest_velocity_y(&self) -> i32 {
        let mut highest_velocity_y = 1;
        let mut misses = 0;
        loop {
            let mut velocity_y = highest_velocity_y;
            let mut y = 0;
            let mut did_hit = false;
            while &y >= self.y_bound.start() {
                if self.y_bound.contains(&y) {
                    did_hit = true;
                }
                y += velocity_y;
                velocity_y -= 1;
            }
            highest_velocity_y += 1;
            if did_hit {
                misses = 0;
            } else {
                misses += 1;
            }
            if misses > ((Y_MIN - Y_MAX) * -1) {
                break;
            }
        }
        highest_velocity_y
    }
}
