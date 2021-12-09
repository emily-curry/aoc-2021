use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::new("aoc-07/input.txt");
    let swarm = CrabSwarm::new(
        input
            .as_string()
            .split(',')
            .map(|x| CrabSub::new(x.parse().expect(format!("Could not parse {}", x).as_str())))
            .collect(),
    );
    let constant_convergence = swarm.converge_constant();
    println!(
        "Swarm converges most efficiently to position {}, using {} units of fuel, using a constant fuel consumption model",
        constant_convergence.0, constant_convergence.1
    );
    let triangular_convergence = swarm.converge_triangular();
    println!(
        "Swarm converges most efficiently to position {}, using {} units of fuel, using a triangular fuel consumption model",
        triangular_convergence.0, triangular_convergence.1
    );
}

#[derive(Debug)]
struct CrabSub {
    pos: u32,
}

impl CrabSub {
    pub fn new(pos: u32) -> Self {
        CrabSub { pos }
    }

    pub fn dist_from(&self, other: u32) -> u32 {
        // Note to self: please just use signed integers. It'll be okay. You really don't need to implement abs() for unsigned ints.
        if self.pos < other {
            other.wrapping_sub(self.pos)
        } else {
            self.pos.wrapping_sub(other)
        }
    }

    pub fn dist_from_triangular(&self, other: u32) -> u32 {
        // The number that equals `1 + 2 + 3 + ... + (x - 1) + x` is known as a triangular number, and it can be calculated via `(x * (x + 1)) / 2`
        let dist = self.dist_from(other);
        (dist * (dist + 1)) / 2
    }

    pub fn get_pos(&self) -> u32 {
        self.pos
    }
}

impl From<u32> for CrabSub {
    fn from(input: u32) -> Self {
        CrabSub::new(input)
    }
}

#[derive(Debug)]
struct CrabSwarm {
    subs: Vec<CrabSub>,
}

impl CrabSwarm {
    pub fn new(subs: Vec<CrabSub>) -> Self {
        CrabSwarm { subs }
    }

    pub fn converge_constant(&self) -> (u32, u32) {
        self.converge(|sub, pos| sub.dist_from(pos))
    }

    pub fn converge_triangular(&self) -> (u32, u32) {
        self.converge(|sub, pos| sub.dist_from_triangular(pos))
    }

    fn converge<F: Fn(&CrabSub, u32) -> u32>(&self, dist_fn: F) -> (u32, u32) {
        let mut fuel: Option<(u32, u32)> = None;
        let range = self.get_min()..=self.get_max();

        for i in range {
            let mut fuel_sum = 0u32;
            for sub in &self.subs {
                fuel_sum += dist_fn(sub, i);
            }
            match fuel {
                None => fuel = Some((i, fuel_sum)),
                Some(f) => {
                    if fuel_sum < f.1 {
                        fuel = Some((i, fuel_sum))
                    }
                }
            }
        }
        fuel.expect("Calculations never ran!")
    }

    fn get_min(&self) -> u32 {
        let mut min = self.subs.first().expect("Swarm is empty!").get_pos();
        for sub in &self.subs {
            let sub_pos = sub.get_pos();
            if sub_pos < min {
                min = sub_pos;
            }
        }
        min
    }

    fn get_max(&self) -> u32 {
        let mut max = self.subs.first().expect("Swarm is empty!").get_pos();
        for sub in &self.subs {
            let sub_pos = sub.get_pos();
            if sub_pos > max {
                max = sub_pos;
            }
        }
        max
    }
}
