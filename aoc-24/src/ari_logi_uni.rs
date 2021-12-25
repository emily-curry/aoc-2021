use crate::instruction_set::InstructionSet;
use crate::instruction_set_pair::InstructionSetPair;

use std::str::Lines;

pub struct AriLogiUni {
    instructions: Vec<InstructionSet>,
}

impl AriLogiUni {
    pub fn solve(&self) -> ([i8; 14], [i8; 14]) {
        let mut instructions = self.instructions.clone();
        let mut pairs: Vec<InstructionSetPair> = Vec::with_capacity(7);
        while instructions.len() > 0 {
            let copy = instructions.clone();
            for (idx, slc) in copy.windows(2).enumerate() {
                if slc[0].add_1 > 0 && slc[1].add_1 <= 0 {
                    let pair = InstructionSetPair::from(&slc[0], &slc[1]);
                    pairs.push(pair);
                    instructions.remove(idx);
                    instructions.remove(idx);
                    break;
                }
            }
        }
        let mut max = [0i8; 14];
        let mut min = [0i8; 14];
        for pair in pairs {
            max[pair.pos_index] = pair.max_pos_value;
            max[pair.neg_index] = pair.max_neg_value;
            min[pair.pos_index] = pair.min_pos_value;
            min[pair.neg_index] = pair.min_neg_value;
        }
        (max, min)
    }

    // Leaving this here because I want to remember about from_fn.
    // fn digits(start: &i64) -> impl Iterator<Item = i64> {
    //     let mut value = *start;
    //     let mut divisor = 1;
    //     while value >= divisor * 10 {
    //         divisor *= 10;
    //     }
    //
    //     // I cannot believe I'm just now learning about iter::from_fn, this is going to change my life
    //     std::iter::from_fn(move || {
    //         if divisor == 0 {
    //             None
    //         } else {
    //             let next = value / divisor;
    //             value %= divisor;
    //             divisor /= 10;
    //             Some(next)
    //         }
    //     })
    // }
}

impl From<Lines<'_>> for AriLogiUni {
    fn from(mut lines: Lines<'_>) -> Self {
        let mut instructions = Vec::with_capacity(14);
        for i in 0..14usize {
            let set = [
                lines.next().unwrap().into(),
                lines.next().unwrap().into(),
                lines.next().unwrap().into(),
                lines.next().unwrap().into(),
                lines.next().unwrap().into(),
                lines.next().unwrap().into(),
                lines.next().unwrap().into(),
                lines.next().unwrap().into(),
                lines.next().unwrap().into(),
                lines.next().unwrap().into(),
                lines.next().unwrap().into(),
                lines.next().unwrap().into(),
                lines.next().unwrap().into(),
                lines.next().unwrap().into(),
                lines.next().unwrap().into(),
                lines.next().unwrap().into(),
                lines.next().unwrap().into(),
                lines.next().unwrap().into(),
            ];
            let r = InstructionSet::new(set, i);
            instructions.push(r);
        }
        AriLogiUni { instructions }
    }
}

#[cfg(test)]
mod tests {
    use crate::AriLogiUni;
    use aoc_core::puzzle_input::PuzzleInput;

    #[test]
    fn input() {
        let input = PuzzleInput::new("../aoc-24/input.txt");
        let alu = AriLogiUni::from(input.to_lines());
        for inst in alu.instructions {
            println!("{:?}", inst);
        }
    }
}
