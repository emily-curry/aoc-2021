use crate::instruction_set::InstructionSet;

pub struct InstructionSetPair {
    pub pos_index: usize,
    pub neg_index: usize, // should always be greater than pos_index
    pub max_pos_value: i8,
    pub max_neg_value: i8,
    pub min_pos_value: i8,
    pub min_neg_value: i8,
}

impl InstructionSetPair {
    pub fn from(pos: &InstructionSet, neg: &InstructionSet) -> Self {
        let offset = pos.add_2 + neg.add_1; // input[neg_index] = input[pos_index] + offset, see InstructionSet for the painful journey here
        let mut max_pos_value = 9i8; // We start with this value, since it is more significant, we want it as high as possible.
        while max_pos_value + offset > 9 {
            max_pos_value -= 1; // Find the first valid pos valid that can also make neg value valid.
            debug_assert_ne!(max_pos_value, 0);
        }
        let mut min_pos_value = 1i8;
        while min_pos_value + offset < 1 {
            min_pos_value += 1;
            debug_assert_ne!(min_pos_value, 10);
        }
        InstructionSetPair {
            pos_index: pos.idx,
            neg_index: neg.idx,
            max_pos_value,
            max_neg_value: max_pos_value + offset,
            min_pos_value,
            min_neg_value: min_pos_value + offset,
        }
    }
}
