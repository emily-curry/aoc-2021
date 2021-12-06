// Keeping this implementation just to compare it to the actual implementation.
// This was a fun puzzle, this impl works for the first half, but doesn't perform well enough to compute the second part in a reasonable amount of time,
// making a different approach to the problem necessary.

fn simulate_day(school: &mut Vec<Lanternfish>) {
    let mut spawn = 0u32;
    for fish in school.iter_mut() {
        match fish.days_until_spawn {
            0 => {
                fish.days_until_spawn = 6;
                spawn += 1;
            }
            _ => fish.days_until_spawn -= 1,
        }
    }
    for _ in 0..spawn {
        school.push(Lanternfish::new(8));
    }
}

struct Lanternfish {
    days_until_spawn: u8,
}

impl Lanternfish {
    pub fn new(days_until_spawn: u8) -> Self {
        Lanternfish { days_until_spawn }
    }
}

impl From<&str> for Lanternfish {
    fn from(input: &str) -> Self {
        let days_until_spawn: u8 = input
            .parse()
            .expect(format!("Couldn't parse {:?}!", input).as_str());
        Lanternfish::new(days_until_spawn)
    }
}
