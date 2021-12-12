use aoc_core::puzzle_input::PuzzleInput;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use std::str::Lines;

fn main() {
    let input = PuzzleInput::new("aoc-12/input.txt");
    let map = CaveMap::from(input.to_lines());
    println!("--- Map of the caves ---\n{}", map);
    let paths = map.get_paths(false);
    println!(
        "Number of unique paths, without revisiting any tiny caves: {:?}",
        paths.len()
    );
    let paths = map.get_paths(true);
    println!(
        "Number of unique paths, allowing a revisit to a tiny cave one time: {:?}",
        paths.len()
    );
}

#[derive(Debug)]
struct CaveMap {
    map: HashMap<Cave, HashSet<Cave>>,
}

impl CaveMap {
    pub fn new(map: HashMap<Cave, HashSet<Cave>>) -> Self {
        CaveMap { map }
    }

    pub fn get_paths(&self, allow_tinly_revisit: bool) -> HashSet<Vec<Cave>> {
        self.continue_path(vec![Cave::Start], allow_tinly_revisit)
    }

    fn continue_path(&self, path: Vec<Cave>, allow_tiny_revisit: bool) -> HashSet<Vec<Cave>> {
        let mut result = HashSet::new();
        let last = path.iter().last().expect("No last element!");
        let next_steps = self.map.get(last).expect("No map element!");

        for step in next_steps {
            if Cave::Start.eq(step) || last.eq(step) {
                continue;
            } else if Cave::End.eq(step) {
                let mut next_path = path.clone();
                next_path.push(Cave::End);
                result.insert(next_path);
                continue;
            } else if let Cave::Tiny(_) = step {
                if path.contains(step) {
                    if allow_tiny_revisit == false {
                        continue;
                    }
                    let has_existing_duplicate = path
                        .iter()
                        .filter(|c| match c {
                            Cave::Tiny(_) => true,
                            _ => false,
                        })
                        .any(|c| path.iter().filter(|p| *p == c).count() > 1);
                    if has_existing_duplicate {
                        continue;
                    }
                }
            }
            let mut next_path = path.clone();
            next_path.push(step.clone());
            let next_paths = self.continue_path(next_path, allow_tiny_revisit);
            result.extend(next_paths);
        }

        result
    }
}

impl Display for CaveMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut names: Vec<Cave> = self.map.keys().map(|x| x.clone()).collect();
        names.sort();

        for name in &names {
            f.write_str(format!("{} -> ", String::from(name)).as_str())?;
            let values = self
                .map
                .get(&name)
                .expect(format!("No key in map for {}!", String::from(name)).as_str());
            for (i, c) in values.iter().enumerate() {
                if i != 0 {
                    f.write_str(" | ")?;
                }
                f.write_str(String::from(c).as_str())?;
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

impl<'a> From<Lines<'a>> for CaveMap {
    fn from(input: Lines) -> Self {
        let mut map = HashMap::new();
        for line in input {
            let mut split = line.split('-');
            let lhs: Cave = split.next().expect("No first element in split!").into();
            let rhs: Cave = split.next().expect("No second element in split!").into();

            map.entry(lhs.clone())
                .or_insert(HashSet::new())
                .insert(rhs.clone());
            map.entry(rhs).or_insert(HashSet::new()).insert(lhs);
        }

        CaveMap::new(map)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Cave {
    Start,
    End,
    Tiny(String),
    Huge(String),
}

impl PartialOrd for Cave {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cave {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(&Cave::Start) || other.eq(&Cave::End) {
            Ordering::Less
        } else if self.eq(&Cave::End) || other.eq(&Cave::Start) {
            Ordering::Greater
        } else {
            match self {
                Cave::Tiny(lhs) => match other {
                    Cave::Tiny(rhs) => lhs.cmp(rhs),
                    Cave::Huge(_) => Ordering::Less,
                    _ => panic!("Invalid state!"),
                },
                Cave::Huge(lhs) => match other {
                    Cave::Tiny(_) => Ordering::Greater,
                    Cave::Huge(rhs) => lhs.cmp(rhs),
                    _ => panic!("Invalid state!"),
                },
                _ => panic!("Invalid state!"),
            }
        }
    }
}

impl From<&str> for Cave {
    fn from(input: &str) -> Self {
        if input.eq("start") {
            Cave::Start
        } else if input.eq("end") {
            Cave::End
        } else if input.chars().next().expect("Empty string!").is_lowercase() {
            Cave::Tiny(input.to_string())
        } else {
            Cave::Huge(input.to_string())
        }
    }
}

impl From<&Cave> for String {
    fn from(input: &Cave) -> Self {
        match input {
            Cave::Start => "start".to_string(),
            Cave::End => "end".to_string(),
            Cave::Tiny(i) => i.clone(),
            Cave::Huge(i) => i.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Cave;
    use std::collections::HashSet;

    #[test]
    fn cave_eq() {
        let cave_1 = Cave::Start;
        let cave_2 = Cave::Start;
        assert_eq!(cave_1, cave_2);

        let cave_3 = Cave::End;
        assert_ne!(cave_1, cave_3);

        let cave_4 = Cave::Tiny("ab".to_string());
        let cave_5 = Cave::Tiny("ab".to_string());
        assert_ne!(cave_1, cave_4);
        assert_eq!(cave_4, cave_5);

        let cave_6 = Cave::Tiny("bc".to_string());
        assert_ne!(cave_5, cave_6);

        let cave_7 = Cave::Huge("AB".to_string());
        assert_ne!(cave_6, cave_7);
        assert_ne!(cave_5, cave_7);
    }

    #[test]
    fn cave_from_str() {
        let cave_start = Cave::from("start");
        assert_eq!(cave_start, Cave::Start);

        let cave_end = Cave::from("end");
        assert_eq!(cave_end, Cave::End);

        let cave_tiny = Cave::from("ab");
        assert_eq!(cave_tiny, Cave::Tiny("ab".to_string()));

        let cave_huge = Cave::from("AB");
        assert_eq!(cave_huge, Cave::Huge("AB".to_string()));
    }

    #[test]
    /// It does! Vectors are equal if all of their elements are equal and in the same order. They have the same hash too!
    fn vec_equality_works_like_i_think() {
        let vec_1 = vec![Cave::Start, Cave::End];
        let vec_2 = vec![Cave::Start, Cave::End];
        assert_eq!(vec_1, vec_2);

        let mut vec_3 = vec![Cave::Start, Cave::End, Cave::Tiny("ab".to_string())];
        assert_ne!(vec_1, vec_3);

        let vec_4 = vec![
            Cave::Start,
            Cave::End,
            Cave::Tiny("ab".to_string()),
            Cave::Huge("ZE".to_string()),
        ];
        assert_ne!(vec_3, vec_4);
        vec_3.push(Cave::Huge("ZE".to_string()));
        assert_eq!(vec_3, vec_4);

        let vec_5 = vec![Cave::Start, Cave::Tiny("ab".to_string()), Cave::End];
        assert_ne!(vec_3, vec_5);

        assert_ne!(
            vec_1,
            vec_1.iter().rev().map(Cave::clone).collect::<Vec<Cave>>()
        );

        assert_eq!(vec_1, vec_1.iter().map(Cave::clone).collect::<Vec<Cave>>());

        let mut vec_set: HashSet<Vec<Cave>> = HashSet::new();
        vec_set.insert(vec_1.clone());
        assert_eq!(vec_set.len(), 1);
        vec_set.insert(vec_1.clone());
        assert_eq!(vec_set.len(), 1);
        vec_set.insert(vec_2.clone());
        assert_eq!(vec_set.len(), 1);
        vec_set.insert(vec_5.clone());
        assert_eq!(vec_set.len(), 2);
    }
}
