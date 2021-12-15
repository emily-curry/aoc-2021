use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum Element {
    B,
    C,
    F,
    H,
    K,
    N,
    O,
    P,
    S,
    V,
}

impl From<char> for Element {
    fn from(input: char) -> Self {
        match input {
            'B' => Element::B,
            'C' => Element::C,
            'F' => Element::F,
            'H' => Element::H,
            'K' => Element::K,
            'N' => Element::N,
            'O' => Element::O,
            'P' => Element::P,
            'S' => Element::S,
            'V' => Element::V,
            _ => panic!("{} is not a known element!", input),
        }
    }
}

impl Debug for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Element::B => 'B',
            Element::C => 'C',
            Element::F => 'F',
            Element::H => 'H',
            Element::K => 'K',
            Element::N => 'N',
            Element::O => 'O',
            Element::P => 'P',
            Element::S => 'S',
            Element::V => 'V',
        };
        f.write_str(format!("{}", char).as_str())?;
        Ok(())
    }
}
