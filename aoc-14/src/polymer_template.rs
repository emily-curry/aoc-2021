use crate::element::Element;
use crate::element_pair::ElementPair;
use std::collections::HashMap;
use std::str::Lines;

/// A model of a polymer chain.
///
/// Internally, we don't actually care about the order of elements, we're only interested in keeping track of the number of each element,
/// and the element is appears adjacent to, since pairs of elements determine how we insert new elements.
#[derive(Debug)]
pub struct PolymerTemplate {
    /// A map of element pairs, where the value is the number of times that pair occurs in the chain.
    pairs: HashMap<ElementPair, u128>,
    /// A map of element insertion rules, where the key is an element pair, and the value is the 2 resulting pairs after an element is inserted in the middle.
    /// We store the tuple of pairs instead of the inserted element because it's a slight bit faster (since it can be pre-computed).
    rules: HashMap<ElementPair, (ElementPair, ElementPair)>,
}

impl PolymerTemplate {
    pub fn new(
        pairs: HashMap<ElementPair, u128>,
        rules: HashMap<ElementPair, (ElementPair, ElementPair)>,
    ) -> Self {
        PolymerTemplate { pairs, rules }
    }

    pub fn get_least_common(&self) -> Option<(Element, u128)> {
        self.get_counts().iter().fold(None, |acc, val| match acc {
            None => Some((*val.0, *val.1)),
            Some(c) => {
                if *val.1 < c.1 {
                    Some((*val.0, *val.1))
                } else {
                    acc
                }
            }
        })
    }

    pub fn get_most_common(&self) -> Option<(Element, u128)> {
        self.get_counts().iter().fold(None, |acc, val| match acc {
            None => Some((*val.0, *val.1)),
            Some(c) => {
                if *val.1 > c.1 {
                    Some((*val.0, *val.1))
                } else {
                    acc
                }
            }
        })
    }

    /// Updates the polymer chain by iterating through every pair that exists in the `pairs` map,
    /// finding a matching rule for that pair and, if one exists, updating the counts to remove all
    /// instances of the input pair and insert instances of the two resulting pairs by the same amount.
    pub fn step(&mut self) {
        let pairs = self.pairs.clone(); // Make a copy, we want a snapshot of our counts at the beginning of the step.
        for (pair, count) in pairs {
            let insert_el = self.rules.get(&pair);
            if insert_el.is_some() {
                let (left, right) = insert_el.unwrap().to_owned();
                *self.pairs.entry(pair).or_insert(0) -= count;
                *self.pairs.entry(left).or_insert(0) += count;
                *self.pairs.entry(right).or_insert(0) += count;
            }
        }
    }

    /// Counts the number of each element in the polymer chain.
    ///
    /// An element's occurrences can be counted by taking the number of pairs the element appears in (lhs == rhs are counted twice),
    /// divided by two, rounded up.
    fn get_counts(&self) -> HashMap<Element, u128> {
        let mut result: HashMap<Element, u128> = HashMap::new();
        for (el, count) in &self.pairs {
            *result.entry(el.get_lhs()).or_insert(0) += count;
            *result.entry(el.get_rhs()).or_insert(0) += count;
        }
        let elements: Vec<Element> = result.keys().map(|key| *key).collect();
        for el in elements {
            // It's possible for there to be an odd number of occurrences if the initial chain starts with that, so round up after dividing.
            let el_mod = result[&el] % 2;
            *result.entry(el).or_insert(0) /= 2;
            *result.entry(el).or_insert(0) += el_mod;
        }
        result
    }
}

impl<'a> From<Lines<'a>> for PolymerTemplate {
    fn from(input: Lines<'a>) -> Self {
        let mut lines = input.into_iter();
        let mut pairs = HashMap::new();
        let pairs_vec: Vec<Element> = lines
            .next()
            .expect("No first line!")
            .chars()
            .map(|c| Element::from(c))
            .collect();
        for pair in pairs_vec.windows(2) {
            let pair = ElementPair::new(pair[0], pair[1]);
            *pairs.entry(pair).or_insert(0u128) += 1;
        }
        lines.next();

        let mut rules = HashMap::new();
        for line in lines {
            let mut split = line.split(" -> ");
            let mut pair_chars = split.next().expect("No first element!").chars();
            let pair = ElementPair::new(
                pair_chars.next().expect("No first char in pair!").into(),
                pair_chars.next().expect("No second char in pair!").into(),
            );
            let insert_el: Element = split
                .next()
                .expect("No second element!")
                .chars()
                .next()
                .expect("No char!")
                .into();
            let insert = (
                ElementPair::new(pair.get_lhs(), insert_el),
                ElementPair::new(insert_el, pair.get_rhs()),
            );
            rules.insert(pair, insert);
        }

        println!("{:?}\n{:?}", pairs, rules);

        PolymerTemplate::new(pairs, rules)
    }
}
