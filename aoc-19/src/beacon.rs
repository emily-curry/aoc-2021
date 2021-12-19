use crate::rotation::{Rotation, RotationAxis};
use std::fmt::{Debug, Formatter};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Beacon {
    a: i32,
    b: i32,
    c: i32,
}

impl Beacon {
    pub fn new(a: i32, b: i32, c: i32) -> Self {
        Beacon { a, b, c }
    }

    pub fn a(&self) -> &i32 {
        &self.a
    }

    pub fn b(&self) -> &i32 {
        &self.b
    }

    pub fn c(&self) -> &i32 {
        &self.c
    }

    pub fn rotate(&self, rotation: &Rotation) -> Self {
        let a = self.rotate_axis(&rotation.a);
        let b = self.rotate_axis(&rotation.b);
        let c = self.rotate_axis(&rotation.c);
        Beacon::new(a, b, c)
    }

    pub fn translate(&self, translation: &(i32, i32, i32)) -> Self {
        Beacon::new(
            self.a + translation.0,
            self.b + translation.1,
            self.c + translation.2,
        )
    }

    fn rotate_axis(&self, axis: &RotationAxis) -> i32 {
        match axis {
            RotationAxis::A(flip) => {
                if *flip {
                    self.a * -1
                } else {
                    self.a
                }
            }
            RotationAxis::B(flip) => {
                if *flip {
                    self.b * -1
                } else {
                    self.b
                }
            }
            RotationAxis::C(flip) => {
                if *flip {
                    self.c * -1
                } else {
                    self.c
                }
            }
        }
    }
}

impl From<&str> for Beacon {
    fn from(input: &str) -> Self {
        let mut split = input.split(",");
        let a = split.next().unwrap().parse().unwrap();
        let b = split.next().unwrap().parse().unwrap();
        let c = split.next().unwrap().parse().unwrap();
        Beacon::new(a, b, c)
    }
}

impl Debug for Beacon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:<5},{:^9},{:>5}", self.a, self.b, self.c))?;
        Ok(())
    }
}
