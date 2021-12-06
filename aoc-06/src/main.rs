use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::new("aoc-06/input.txt");
    let mut school: School = input.as_string().as_str().into();
    while school.day < 80 {
        school.simulate_day();
    }
    println!("Lanternfish school population on day 80: {}", school.size());
    while school.day < 256 {
        school.simulate_day();
    }
    println!(
        "Lanternfish school population on day 256: {}",
        school.size()
    );
}

struct School {
    /// The index is the days until spawn, the value is the amount of fish with that value.
    population: [u128; 9],
    day: u32,
}

impl School {
    pub fn new(population: [u128; 9]) -> Self {
        School { population, day: 0 }
    }

    pub fn simulate_day(&mut self) {
        let new_fish = self.population[0];
        for i in 0usize..=7usize {
            self.population[i] = self.population[i + 1];
        }
        self.population[8] = new_fish;
        self.population[6] += new_fish;
        self.day += 1;
    }

    pub fn size(&self) -> u128 {
        self.population.iter().fold(0u128, |acc, val| acc + *val)
    }
}

impl From<&str> for School {
    fn from(input: &str) -> Self {
        let values: Vec<usize> = input
            .split(",")
            .map(|x| {
                x.parse()
                    .expect(format!("Could not parse {:?}", x).as_str())
            })
            .collect();
        let mut population = [0u128; 9];
        for value in values {
            population[value] += 1;
        }
        School::new(population)
    }
}
