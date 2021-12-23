use crate::cube_reactor_instruction::CubeReactorInstruction;
use crate::cuboid_set::CuboidSet;
use crate::cuboid_state::CuboidState;
use std::collections::VecDeque;
use std::str::Lines;

pub struct CubeReactor {
    instructions: VecDeque<CubeReactorInstruction>,
    reactor: CuboidSet,
}

impl CubeReactor {
    pub fn count(&self) -> usize {
        self.reactor.volume()
    }

    pub fn reboot_init(&mut self) {
        for _ in 0..20 {
            self.step();
        }
    }

    pub fn reboot(&mut self) {
        while self.has_next_step() {
            self.step();
        }
    }

    fn has_next_step(&self) -> bool {
        self.instructions.len() > 0
    }

    fn step(&mut self) {
        let inst = self
            .instructions
            .pop_front()
            .expect("No instructions left in the queue!");
        // println!("{}", inst);
        match inst.state {
            CuboidState::On => self.reactor.union(&inst.cube),
            CuboidState::Off => self.reactor.intersect(&inst.cube),
        };
    }
}

impl From<Lines<'_>> for CubeReactor {
    fn from(input: Lines<'_>) -> Self {
        let mut instructions = VecDeque::new();
        for line in input {
            let inst = CubeReactorInstruction::from(line);
            instructions.push_back(inst);
        }
        let reactor = CuboidSet::new(vec![]);

        CubeReactor {
            instructions,
            reactor,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cube_reactor::CubeReactor;

    #[test]
    fn example_step_1() {
        let mut reactor = get_example_reactor();
        reactor.step();
        assert_eq!(reactor.count(), 27);
    }

    #[test]
    fn example_step_4() {
        let mut reactor = get_example_reactor();
        reactor.reboot();
        assert_eq!(reactor.count(), 39);
    }

    fn get_example_reactor() -> CubeReactor {
        let input = r#"on x=10..12,y=10..12,z=10..12
                            on x=11..13,y=11..13,z=11..13
                            off x=9..11,y=9..11,z=9..11
                            on x=10..10,y=10..10,z=10..10"#;
        CubeReactor::from(input.lines())
    }
}
