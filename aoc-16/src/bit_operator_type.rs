use aoc_core::bit::bit_size::BitSize;

#[derive(Debug)]
pub enum BitOperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equal,
}

impl From<u8> for BitOperatorType {
    fn from(input: u8) -> Self {
        match input {
            0 => BitOperatorType::Sum,
            1 => BitOperatorType::Product,
            2 => BitOperatorType::Minimum,
            3 => BitOperatorType::Maximum,
            5 => BitOperatorType::GreaterThan,
            6 => BitOperatorType::LessThan,
            7 => BitOperatorType::Equal,
            _ => panic!("Invalid value {}", input),
        }
    }
}

impl BitSize for BitOperatorType {
    fn bit_size(&self) -> usize {
        3
    }
}
