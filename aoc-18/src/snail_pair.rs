use regex::Regex;
use std::fmt::{Debug, Formatter};
use std::ops::Add;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct SnailPair {
    lhs: SnailPairElement,
    rhs: SnailPairElement,
}

impl SnailPair {
    pub fn str_size(&self) -> usize {
        self.lhs.str_size() + self.rhs.str_size() + 3
    }

    pub fn get_magnitude(&self) -> u64 {
        (self.lhs.get_magnitude() * 3) + (self.rhs.get_magnitude() * 2)
    }

    fn reduce(&mut self) {
        loop {
            if self.explode(0).is_some() {
                continue;
            }
            if self.split().is_some() {
                continue;
            }

            break;
        }
    }

    fn explode(&mut self, depth: u8) -> Option<SnailPairExplodeResult> {
        let result = match &mut self.lhs {
            SnailPairElement::Literal(_) => None,
            SnailPairElement::Pair(p) => {
                if depth == 3 {
                    let explode_left = match p.lhs {
                        SnailPairElement::Literal(val) => val,
                        _ => panic!("Invalid data!"),
                    };
                    let explode_right = match p.rhs {
                        SnailPairElement::Literal(val) => val,
                        _ => panic!("Invalid data!"),
                    };
                    Some(SnailPairExplodeResult::new(
                        explode_left,
                        explode_right,
                        false,
                    ))
                } else {
                    p.explode(depth + 1).take().map(|mut res| {
                        res.from_right = false;
                        res
                    })
                }
            }
        }
        .or_else(|| match &mut self.rhs {
            SnailPairElement::Literal(_) => None,
            SnailPairElement::Pair(p) => {
                if depth == 3 {
                    let explode_left = match p.lhs {
                        SnailPairElement::Literal(val) => val,
                        _ => panic!("Invalid data!"),
                    };
                    let explode_right = match p.rhs {
                        SnailPairElement::Literal(val) => val,
                        _ => panic!("Invalid data!"),
                    };
                    Some(SnailPairExplodeResult::new(
                        explode_left,
                        explode_right,
                        true,
                    ))
                } else {
                    p.explode(depth + 1).take().map(|mut res| {
                        res.from_right = true;
                        res
                    })
                }
            }
        });

        if let Some(mut res) = result {
            if res.should_remove {
                if res.from_right {
                    self.rhs = SnailPairElement::Literal(0);
                    self.lhs.add_left(res.left.take().unwrap());
                } else {
                    self.lhs = SnailPairElement::Literal(0);
                    self.rhs.add_right(res.right.take().unwrap());
                }
                res.should_remove = false;
            } else {
                if res.from_right {
                    if let Some(left_val) = res.left.take() {
                        self.lhs.add_left(left_val);
                    }
                } else {
                    if let Some(right_val) = res.right.take() {
                        self.rhs.add_right(right_val);
                    }
                }
            }
            return Some(res);
        }
        None
    }

    fn split(&mut self) -> Option<SnailPairSplitResult> {
        let result = match &mut self.lhs {
            SnailPairElement::Literal(val) => {
                if *val >= 10 {
                    Some(SnailPairSplitResult::new(*val, false))
                } else {
                    None
                }
            }
            SnailPairElement::Pair(p) => p.split(),
        }
        .or_else(|| match &mut self.rhs {
            SnailPairElement::Literal(val) => {
                if *val >= 10 {
                    Some(SnailPairSplitResult::new(*val, true))
                } else {
                    None
                }
            }
            SnailPairElement::Pair(p) => p.split(),
        });
        if let Some(mut res) = result {
            if let Some(val) = res.value.take() {
                let next_left = val / 2;
                let next_right = next_left + (val % 2);
                let next = SnailPairElement::Pair(Box::new(SnailPair {
                    lhs: SnailPairElement::Literal(next_left),
                    rhs: SnailPairElement::Literal(next_right),
                }));
                if res.from_right {
                    self.rhs = next;
                } else {
                    self.lhs = next;
                };
            };
            return Some(res);
        }
        None
    }
}

impl Debug for SnailPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("[{:?},{:?}]", self.lhs, self.rhs).as_str())?;
        Ok(())
    }
}

impl From<&str> for SnailPair {
    fn from(input: &str) -> Self {
        let exp = Regex::new("\\[(.*)]").unwrap();
        let cap = exp.captures(input).unwrap();
        let inner = cap.get(1).unwrap().as_str();
        let lhs_first = inner.chars().next().unwrap().to_digit(10);
        let lhs = match lhs_first {
            None => SnailPairElement::Pair(Box::new(inner.into())),
            Some(digit) => SnailPairElement::Literal(digit as u8),
        };
        let rhs_start = 1 + lhs.str_size();
        let rhs_first = inner.chars().nth(rhs_start).unwrap().to_digit(10);
        let rhs = match rhs_first {
            None => SnailPairElement::Pair(Box::new(inner[rhs_start..].into())),
            Some(digit) => SnailPairElement::Literal(digit as u8),
        };
        SnailPair { lhs, rhs }
    }
}

impl Add for SnailPair {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut pair = SnailPair {
            lhs: SnailPairElement::Pair(Box::new(self)),
            rhs: SnailPairElement::Pair(Box::new(rhs)),
        };
        pair.reduce();
        pair
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
enum SnailPairElement {
    Literal(u8),
    Pair(Box<SnailPair>),
}

impl SnailPairElement {
    fn str_size(&self) -> usize {
        match self {
            SnailPairElement::Literal(_) => 1,
            SnailPairElement::Pair(p) => p.str_size(),
        }
    }

    fn get_magnitude(&self) -> u64 {
        match self {
            SnailPairElement::Literal(val) => *val as u64,
            SnailPairElement::Pair(p) => p.get_magnitude(),
        }
    }

    fn add_left(&mut self, other: u8) {
        match self {
            SnailPairElement::Literal(val) => *val += other,
            SnailPairElement::Pair(p) => p.rhs.add_left(other),
        }
    }

    fn add_right(&mut self, other: u8) {
        match self {
            SnailPairElement::Literal(val) => *val += other,
            SnailPairElement::Pair(p) => p.lhs.add_right(other),
        }
    }
}

impl Debug for SnailPairElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailPairElement::Literal(val) => f.write_str(format!("{}", val).as_str())?,
            SnailPairElement::Pair(p) => p.fmt(f)?,
        }
        Ok(())
    }
}

struct SnailPairExplodeResult {
    left: Option<u8>,
    right: Option<u8>,
    should_remove: bool,
    from_right: bool,
}

impl SnailPairExplodeResult {
    fn new(left: u8, right: u8, from_right: bool) -> Self {
        SnailPairExplodeResult {
            left: Some(left),
            right: Some(right),
            from_right,
            should_remove: true,
        }
    }
}

struct SnailPairSplitResult {
    value: Option<u8>,
    from_right: bool,
}

impl SnailPairSplitResult {
    fn new(value: u8, from_right: bool) -> Self {
        SnailPairSplitResult {
            value: Some(value),
            from_right,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::snail_pair::{SnailPair, SnailPairElement};
    use aoc_core::puzzle_input::PuzzleInput;
    use regex::Regex;
    use std::collections::HashMap;

    #[test]
    fn can_parse_known_input() {
        // Input: [[5,[[8,5],8]],[[9,3],[0,3]]]
        let input = PuzzleInput::new("../aoc-18/input.txt");
        let mut lines = input.to_lines();
        let first = SnailPair::from(lines.next().unwrap());
        // Should've picked a shorter input geez
        let known = SnailPair {
            lhs: SnailPairElement::Pair(Box::new(SnailPair {
                lhs: SnailPairElement::Literal(5),
                rhs: SnailPairElement::Pair(Box::new(SnailPair {
                    lhs: SnailPairElement::Pair(Box::new(SnailPair {
                        lhs: SnailPairElement::Literal(8),
                        rhs: SnailPairElement::Literal(5),
                    })),
                    rhs: SnailPairElement::Literal(8),
                })),
            })),
            rhs: SnailPairElement::Pair(Box::new(SnailPair {
                lhs: SnailPairElement::Pair(Box::new({
                    SnailPair {
                        lhs: SnailPairElement::Literal(9),
                        rhs: SnailPairElement::Literal(3),
                    }
                })),
                rhs: SnailPairElement::Pair(Box::new({
                    SnailPair {
                        lhs: SnailPairElement::Literal(0),
                        rhs: SnailPairElement::Literal(3),
                    }
                })),
            })),
        };
        assert_eq!(first, known);
        while let Some(line) = lines.next() {
            SnailPair::from(line); // Just ensure no panics
        }
    }
    #[test]
    fn can_parse_all_input() {
        let input = PuzzleInput::new("../aoc-18/input.txt");
        let lines = input.to_lines();
        let lines_count = input.to_lines().count();
        let pairs = lines.map(SnailPair::from).collect::<Vec<_>>();
        assert_eq!(lines_count, pairs.len());
    }

    #[test]
    fn regex_capture() {
        let input = PuzzleInput::new("../aoc-18/input.txt");
        let mut lines = input.to_lines();
        let exp = Regex::new("\\[(.*)]").unwrap();
        let cap = exp.captures(lines.next().unwrap()).unwrap();
        assert_eq!(cap.get(1).unwrap().as_str(), "[5,[[8,5],8]],[[9,3],[0,3]]");
    }

    #[test]
    fn explode() {
        let mut example_1 = SnailPair::from("[[[[[9,8],1],2],3],4]");
        example_1.explode(0);
        assert_eq!(example_1, SnailPair::from("[[[[0,9],2],3],4]"));

        let mut example_2 = SnailPair::from("[7,[6,[5,[4,[3,2]]]]]");
        example_2.explode(0);
        assert_eq!(example_2, SnailPair::from("[7,[6,[5,[7,0]]]]"));

        let mut example_3 = SnailPair::from("[[6,[5,[4,[3,2]]]],1]");
        example_3.explode(0);
        assert_eq!(example_3, SnailPair::from("[[6,[5,[7,0]]],3]"));

        let mut example_4 = SnailPair::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        example_4.explode(0);
        assert_eq!(
            example_4,
            SnailPair::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
        );

        let mut example_5 = SnailPair::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        example_5.explode(0);
        assert_eq!(example_5, SnailPair::from("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
    }

    #[test]
    fn split() {
        let mut example_1 = SnailPair::from("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
        example_1.explode(0);
        example_1.split();
        example_1.split();
        assert_eq!(
            example_1,
            SnailPair::from("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]")
        );
    }

    #[test]
    fn reduce() {
        let mut example_1 = SnailPair::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        example_1.reduce();
        assert_eq!(
            example_1,
            SnailPair::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
    }

    #[test]
    fn hash() {
        let mut set = HashMap::<SnailPair, bool>::new();
        set.insert(
            SnailPair::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"),
            true,
        );
        let prev = set.insert(
            SnailPair::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"),
            false,
        );
        assert_eq!(set.len(), 1);
        assert_eq!(prev.unwrap(), true);
        assert_eq!(
            set.get(&SnailPair::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"))
                .unwrap(),
            &false
        );
    }
}
