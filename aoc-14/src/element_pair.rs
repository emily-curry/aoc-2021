use crate::element::Element;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct ElementPair {
    lhs: Element,
    rhs: Element,
}

impl ElementPair {
    pub fn new(lhs: Element, rhs: Element) -> Self {
        ElementPair { lhs, rhs }
    }

    pub fn get_lhs(&self) -> Element {
        self.lhs
    }

    pub fn get_rhs(&self) -> Element {
        self.rhs
    }
}
