use crate::ari_logi_checkpoint::AriLogiCheckpoint;
use crate::ari_logi_uni_state::AriLogiUniState;
use crate::instruction_set::InstructionSet;
use std::ops::Range;
use std::str::Lines;
use std::sync::mpsc;

pub struct AriLogiUni {
    instructions: [InstructionSet; 14],
}

impl AriLogiUni {
    pub fn brute_force(&self) -> Option<AriLogiUniState> {
        let mut handles = vec![];
        let full = 10_000_000_000_000i64..100_000_000_000_000i64;
        for range in AriLogiUni::ranges(9, full).into_iter().rev() {
            let iter = AriLogiUni::iter_range(range, self.instructions.clone());
            let t = std::thread::spawn(move || {
                iter.filter(Option::is_some)
                    .map(Option::unwrap)
                    .find(|state| {
                        if state.z == 0 {
                            println!("Found solution!\n{}", state);
                            true
                        } else {
                            false
                        }
                    })
            });
            handles.push(t);
        }

        for handle in handles {
            let result = handle.join().unwrap();
            if result.is_some() {
                return result;
            }
        }
        None
    }

    fn ranges(count: i64, full: Range<i64>) -> Vec<Range<i64>> {
        let mut result = Vec::new();
        let step = (full.end - full.start) / &count;
        for i in 0..count {
            let step_range = (full.start + (step * i))..(full.start + (step * (i + 1)));
            result.push(step_range);
        }

        result
    }

    fn iter_range(
        range: Range<i64>,
        instructions: [InstructionSet; 14],
    ) -> impl Iterator<Item = Option<AriLogiUniState>> {
        let mut base = AriLogiCheckpoint::default();
        range
            .rev()
            .filter(|val| val % 10 != 0)
            .map(|val| AriLogiUni::digits(&val))
            .map(move |dig| base.get(dig, &instructions))
    }

    fn digits(start: &i64) -> impl Iterator<Item = i64> {
        let mut value = *start;
        let mut divisor = 1;
        while value >= divisor * 10 {
            divisor *= 10;
        }

        // I cannot believe I'm just now learning about iter::from_fn, this is going to change my life
        std::iter::from_fn(move || {
            if divisor == 0 {
                None
            } else {
                let next = value / divisor;
                value %= divisor;
                divisor /= 10;
                Some(next)
            }
        })
    }
}

impl From<Lines<'_>> for AriLogiUni {
    fn from(mut lines: Lines<'_>) -> Self {
        let get_set = |l: &mut Lines<'_>| {
            let set = [
                l.next().unwrap().into(),
                l.next().unwrap().into(),
                l.next().unwrap().into(),
                l.next().unwrap().into(),
                l.next().unwrap().into(),
                l.next().unwrap().into(),
                l.next().unwrap().into(),
                l.next().unwrap().into(),
                l.next().unwrap().into(),
                l.next().unwrap().into(),
                l.next().unwrap().into(),
                l.next().unwrap().into(),
                l.next().unwrap().into(),
                l.next().unwrap().into(),
                l.next().unwrap().into(),
                l.next().unwrap().into(),
                l.next().unwrap().into(),
                l.next().unwrap().into(),
            ];
            InstructionSet::new(set)
        };
        let instructions = [
            get_set(&mut lines),
            get_set(&mut lines),
            get_set(&mut lines),
            get_set(&mut lines),
            get_set(&mut lines),
            get_set(&mut lines),
            get_set(&mut lines),
            get_set(&mut lines),
            get_set(&mut lines),
            get_set(&mut lines),
            get_set(&mut lines),
            get_set(&mut lines),
            get_set(&mut lines),
            get_set(&mut lines),
        ];
        AriLogiUni { instructions }
    }
}

#[cfg(test)]
mod tests {
    use crate::AriLogiUni;

    #[test]
    fn stub() {}

    #[test]
    fn ranges() {
        let full = 10_000_000_000_000i64..100_000_000_000_000i64;
        let split_1 = AriLogiUni::ranges(9, full);
        assert_eq!(split_1.len(), 9);
        assert_eq!(split_1[0].start, 10_000_000_000_000);
        assert_eq!(split_1[0].end, 20_000_000_000_000);
        assert_eq!(split_1[1].start, 20_000_000_000_000);
        assert_eq!(split_1[1].end, 30_000_000_000_000);
        assert_eq!(split_1[8].start, 90_000_000_000_000);
        assert_eq!(split_1[8].end, 100_000_000_000_000);

        let split_2 = AriLogiUni::ranges(90, full);
        assert_eq!(split_2.len(), 90);
        assert_eq!(split_2[0].start, 10_000_000_000_000);
        assert_eq!(split_2[0].end, 11_000_000_000_000);
        assert_eq!(split_2[1].start, 11_000_000_000_000);
        assert_eq!(split_2[1].end, 12_000_000_000_000);
        assert_eq!(split_2[89].start, 99_000_000_000_000);
        assert_eq!(split_2[89].end, 100_000_000_000_000);
    }
}
