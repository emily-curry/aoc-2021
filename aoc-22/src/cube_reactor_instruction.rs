use crate::cuboid_cube::CuboidCube;
use crate::cuboid_state::CuboidState;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub struct CubeReactorInstruction {
    pub state: CuboidState,
    pub cube: CuboidCube,
}

impl From<&str> for CubeReactorInstruction {
    fn from(input: &str) -> Self {
        let mut split = input.trim().split(' ');
        let state = CuboidState::from(split.next().unwrap());
        let mut bounds = split.next().unwrap().split(',').map(|bound| {
            let without_prefix = &bound[2..];
            let mut bound_split = without_prefix.split("..");
            let left: i32 = bound_split.next().unwrap().parse().unwrap();
            let right: i32 = bound_split.next().unwrap().parse().unwrap();
            if left > right {
                panic!("Unexpected input! I don't think this happens but I sure do want to know if it does.")
            }
            left..(right+1)
        });
        let x = bounds.next().unwrap();
        let y = bounds.next().unwrap();
        let z = bounds.next().unwrap();

        CubeReactorInstruction {
            state,
            cube: CuboidCube::new(x, y, z),
        }
    }
}

impl Display for CubeReactorInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Turning {} all cubes for x: {}..{}, y: {}..{}, z: {}..{}",
            self.state,
            self.cube.x.start,
            self.cube.x.end,
            self.cube.y.start,
            self.cube.y.end,
            self.cube.z.start,
            self.cube.z.end
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::cube_reactor_instruction::CubeReactorInstruction;
    use crate::cuboid_cube::CuboidCube;
    use crate::cuboid_state::CuboidState;

    #[test]
    fn from() {
        let actual =
            CubeReactorInstruction::from("on x=61051..78708,y=39294..50128,z=-20827..1285");
        let expected = CubeReactorInstruction {
            state: CuboidState::On,
            cube: CuboidCube::new(61051..78709, 39294..50129, -20827..1286),
        };
        assert_eq!(actual, expected);

        let actual =
            CubeReactorInstruction::from("off x=-27246..8755,y=4459..30303,z=-93666..-72766");
        let expected = CubeReactorInstruction {
            state: CuboidState::Off,
            cube: CuboidCube::new(-27246..8756, 4459..30304, -93666..-72765),
        };
        assert_eq!(actual, expected);
    }
}
