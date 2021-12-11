use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::new("aoc-10/input.txt");
    let instructions: Vec<NavigationInstruction> = input
        .to_lines()
        .map(|l| NavigationInstruction::from(l))
        .collect();
    let corrupt: Vec<NavigationChunkType> = instructions
        .iter()
        .map(NavigationInstruction::complete)
        .filter(Result::is_err)
        .map(Result::unwrap_err)
        .collect();
    let corrupt_score = corrupt
        .iter()
        .fold(0u32, |acc, val| acc + val.get_corrupt_score());
    println!("Score from corrupt lines: {}", corrupt_score);

    let incomplete: Vec<Vec<NavigationChunkType>> = instructions
        .iter()
        .map(NavigationInstruction::complete)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect();
    let mut incomplete_scores: Vec<u64> = incomplete
        .iter()
        .map(|incomplete_item| {
            incomplete_item
                .iter()
                .fold(0u64, |acc, val| (acc * 5) + val.get_incomplete_score())
        })
        .collect();
    incomplete_scores.sort();
    let midpoint = (incomplete_scores.len() - 1) / 2;
    println!("Median incomplete score: {}", incomplete_scores[midpoint]);
}

struct NavigationInstruction {
    elements: Vec<NavigationChunkElement>,
}

impl NavigationInstruction {
    pub fn new(elements: Vec<NavigationChunkElement>) -> Self {
        NavigationInstruction { elements }
    }

    /// If the line is not corrupt, then returns Ok with a vec of elements that can complete the line.
    /// If the line is corrupt, then returns Err with the type that caused the error.
    pub fn complete(&self) -> Result<Vec<NavigationChunkType>, NavigationChunkType> {
        let mut stack: Vec<NavigationChunkType> = Vec::new();
        for element in &self.elements {
            match element {
                NavigationChunkElement::Open(subtype) => stack.push(*subtype),
                NavigationChunkElement::Close(subtype) => {
                    let stack_top = stack
                        .pop()
                        .expect("Got a close element, but nothing at all to pair it with!");
                    if stack_top != *subtype {
                        return Err(*subtype);
                    }
                }
            }
        }
        stack.reverse();
        Ok(stack)
    }
}

impl From<&str> for NavigationInstruction {
    fn from(input: &str) -> Self {
        let elements: Vec<NavigationChunkElement> = input
            .chars()
            .map(|c| NavigationChunkElement::from(c))
            .collect();
        NavigationInstruction::new(elements)
    }
}

enum NavigationChunkElement {
    Open(NavigationChunkType),
    Close(NavigationChunkType),
}

impl From<char> for NavigationChunkElement {
    fn from(value: char) -> Self {
        match value {
            '{' | '(' | '[' | '<' => NavigationChunkElement::Open(NavigationChunkType::from(value)),
            '}' | ')' | ']' | '>' => {
                NavigationChunkElement::Close(NavigationChunkType::from(value))
            }
            _ => panic!("Invalid value provided: {}", value),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum NavigationChunkType {
    Curly,
    Square,
    Round,
    Pointy, // yes they are called "pointy brackets"
}

impl NavigationChunkType {
    pub fn get_corrupt_score(&self) -> u32 {
        match self {
            NavigationChunkType::Curly => 1197,
            NavigationChunkType::Square => 57,
            NavigationChunkType::Round => 3,
            NavigationChunkType::Pointy => 25137,
        }
    }

    pub fn get_incomplete_score(&self) -> u64 {
        match self {
            NavigationChunkType::Curly => 3,
            NavigationChunkType::Square => 2,
            NavigationChunkType::Round => 1,
            NavigationChunkType::Pointy => 4,
        }
    }
}

impl From<char> for NavigationChunkType {
    fn from(input: char) -> Self {
        match input {
            '{' | '}' => NavigationChunkType::Curly,
            '[' | ']' => NavigationChunkType::Square,
            '(' | ')' => NavigationChunkType::Round,
            '<' | '>' => NavigationChunkType::Pointy,
            _ => panic!("Invalid value provided: {}", input),
        }
    }
}
