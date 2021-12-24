use crate::register::Register;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Instruction {
    Inp(Register),
    Add((Register, Register)),
    Mul((Register, Register)),
    Div((Register, Register)),
    Mod((Register, Register)),
    Eql((Register, Register)),
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let mut split = input.split(' ');
        match split.next().unwrap() {
            "inp" => Instruction::Inp(split.next().unwrap().into()),
            "add" => Instruction::Add((split.next().unwrap().into(), split.next().unwrap().into())),
            "mul" => Instruction::Mul((split.next().unwrap().into(), split.next().unwrap().into())),
            "div" => Instruction::Div((split.next().unwrap().into(), split.next().unwrap().into())),
            "mod" => Instruction::Mod((split.next().unwrap().into(), split.next().unwrap().into())),
            "eql" => Instruction::Eql((split.next().unwrap().into(), split.next().unwrap().into())),
            _ => panic!("Invalid value!"),
        }
    }
}
