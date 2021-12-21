use crate::image_algorithm::ImageAlgorithm;
use crate::image_point::ImagePoint;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use std::str::Lines;

pub struct Image {
    alg: ImageAlgorithm,
    /// The set of all _lit_ pixels. We're not really using any particular property
    /// of the BTreeSet anymore, but originally this was a BTreeMap because we did
    /// care about the order of key-value pairs (which is why Ord is implemented for ImagePoint).
    pix: BTreeSet<ImagePoint>,
    /// Top left corner and bottom right corner
    bounds: (ImagePoint, ImagePoint),
    enhanced_count: usize,
}

impl Image {
    pub fn enhance(&mut self) {
        let next = self.calculate_enhance();
        self.pix = next;
        self.bounds = (
            ImagePoint::new(self.bounds.0.x - 1, self.bounds.0.y - 1),
            ImagePoint::new(self.bounds.1.x + 1, self.bounds.1.y + 1),
        );
        self.enhanced_count += 1;
    }

    pub fn count_light(&self) -> usize {
        self.pix.len()
    }

    pub fn get_enhance_count(&self) -> usize {
        self.enhanced_count
    }

    fn get_point(&self, point: &ImagePoint) -> bool {
        if point.x < self.bounds.0.x
            || point.y < self.bounds.0.y
            || point.x > self.bounds.1.x
            || point.y > self.bounds.1.y
        {
            // Every point outside the bounds is deterministic. It is not always dark, but it does always alternate between dark and light.
            self.enhanced_count % 2 == 1
        } else {
            self.pix.contains(point)
        }
    }

    fn calculate_enhance(&self) -> BTreeSet<ImagePoint> {
        let mut next = BTreeSet::new();

        for y in (self.bounds.0.y - 1)..=(self.bounds.1.y + 1) {
            for x in (self.bounds.0.x - 1)..=(self.bounds.1.x + 1) {
                let point = ImagePoint::new(x, y);
                let enhanced = self.get_enhancement_at_point(&point);
                if enhanced {
                    next.insert(point);
                }
            }
        }

        next
    }

    fn get_enhancement_at_point(&self, point: &ImagePoint) -> bool {
        self.alg.get(self.get_enhancement_index(point))
    }

    fn get_enhancement_index(&self, point: &ImagePoint) -> usize {
        let points_around = point.get_points_around();
        let mut enhancement_index = 0usize;
        for point in points_around {
            enhancement_index <<= 1;
            let value = if self.get_point(&point) { 1 } else { 0 };
            enhancement_index |= value;
        }
        enhancement_index
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let padding = 100 - (self.enhanced_count as i64 * 2);

        for y in (self.bounds.0.y - (padding / 2))..=(self.bounds.1.y + (padding / 2)) {
            let mut current = String::new();
            for x in (self.bounds.0.x - (padding / 2))..=(self.bounds.1.x + (padding / 2)) {
                let char = match self.get_point(&ImagePoint::new(x, y)) {
                    true => "#",
                    false => ".",
                };
                current += char;
            }
            f.write_fmt(format_args!("{}\n", current))?;
        }

        Ok(())
    }
}

impl From<Lines<'_>> for Image {
    fn from(mut input: Lines<'_>) -> Self {
        let alg_str = input.next().unwrap();
        let alg = ImageAlgorithm::from(alg_str);
        input.next();

        let mut pix = BTreeSet::new();
        let mut y_upper = 0;
        let mut x_upper = 0;
        for (y, line) in input.enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    pix.insert(ImagePoint::new(x as i64, y as i64));
                }
                if x as i64 > x_upper {
                    x_upper = x as i64;
                }
            }
            y_upper = y as i64;
        }
        Image {
            pix,
            alg,
            bounds: (ImagePoint::new(0, 0), ImagePoint::new(x_upper, y_upper)),
            enhanced_count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Image;
    use aoc_core::puzzle_input::PuzzleInput;

    #[test]
    fn image_display() {
        let input = PuzzleInput::new("../aoc-20/input.txt");
        let image = Image::from(input.to_lines());
        print!("{}", image);
    }

    #[test]
    fn image_display_enhance() {
        let input = PuzzleInput::new("../aoc-20/input.txt");
        let mut image = Image::from(input.to_lines());
        image.enhance();
        print!("{}", image);
    }

    #[test]
    fn image_display_enhance_2() {
        let input = PuzzleInput::new("../aoc-20/input.txt");
        let mut image = Image::from(input.to_lines());
        image.enhance();
        image.enhance();
        print!("{}", image);
    }
}
