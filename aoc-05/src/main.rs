use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::new("aoc-05/input.txt");
    let lines: Vec<Line> = input.to_lines().map(|x| x.into()).collect();

    let straight_lines: Vec<&Line> = lines
        .iter()
        .filter(|line| line.a.x == line.b.x || line.a.y == line.b.y)
        .collect();

    let mut straight_line_grid = Grid::new();
    for line in straight_lines {
        straight_line_grid.add_line(line);
    }
    let count_straight_line_overlaps = straight_line_grid.get_count_overlaps();
    println!("Straight line overlaps: {}", count_straight_line_overlaps);

    let mut all_line_grid = Grid::new();
    for line in lines {
        all_line_grid.add_line(&line);
    }
    let count_overlaps = all_line_grid.get_count_overlaps();
    println!("All overlaps: {}", count_overlaps);
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<u8>>,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            data: vec![vec![0; 1000]; 1000],
        }
    }

    pub fn add_line(&mut self, line: &Line) {
        if line.a.x == line.b.x {
            // Is vertical
            let range = if line.a.y < line.b.y {
                line.a.y..=line.b.y
            } else {
                line.b.y..=line.a.y
            };
            for i in range {
                self.data[line.a.x as usize][i as usize] += 1
            }
        } else if line.a.y == line.b.y {
            // Is horizontal
            let range = if line.a.x < line.b.x {
                line.a.x..=line.b.x
            } else {
                line.b.x..=line.a.x
            };
            for i in range {
                self.data[i as usize][line.a.y as usize] += 1
            }
        } else {
            // Is diagonal
            let dist = if line.a.x < line.b.x {
                line.b.x - line.a.x
            } else {
                line.a.x - line.b.x
            };
            let is_h_increment = line.a.x < line.b.x;
            let is_v_increment = line.a.y < line.b.y;
            for i in 0..=dist {
                let x = if is_h_increment {
                    line.a.x + i
                } else {
                    line.a.x - i
                };
                let y = if is_v_increment {
                    line.a.y + i
                } else {
                    line.a.y - i
                };
                self.data[x as usize][y as usize] += 1;
            }
        }
    }

    pub fn get_count_overlaps(&self) -> u32 {
        let mut count = 0u32;
        for slice in &self.data {
            for overlaps in slice {
                if *overlaps >= 2 {
                    count += 1;
                }
            }
        }
        count
    }
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    pub fn new(a: Point, b: Point) -> Self {
        Line { a, b }
    }
}

impl From<&str> for Line {
    fn from(input: &str) -> Self {
        let mut points: Vec<Point> = input
            .split(" -> ")
            .map(|pair| {
                let mut p: Vec<u32> = pair.split(",").map(|x| x.parse().unwrap()).collect();
                Point::new(p.remove(0), p.remove(0))
            })
            .collect();
        Line::new(points.remove(0), points.remove(0))
    }
}
