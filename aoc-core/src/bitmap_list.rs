use crate::bitmap::Bitmap;
use std::str::Lines;

#[derive(Debug)]
pub struct BitmapList {
    pub list: Vec<Bitmap>,
    pub bitmap_size: usize,
}

impl BitmapList {
    pub fn new(list: Vec<Bitmap>, bitmap_size: usize) -> Self {
        BitmapList { list, bitmap_size }
    }
}

impl<'a> From<Lines<'a>> for BitmapList {
    fn from(lines: Lines<'a>) -> Self {
        let vec: Vec<&str> = lines.collect();
        let first = vec.first();
        assert_eq!(first.is_some(), true);
        let size = first.unwrap().len();
        let list: Vec<Bitmap> = vec
            .iter()
            .map(|x| Bitmap::new(u32::from_str_radix(x, 2).unwrap(), size))
            .collect();
        BitmapList::new(list, size)
    }
}

#[cfg(test)]
mod tests {
    use crate::bitmap_list::BitmapList;
    use crate::puzzle_input::PuzzleInput;

    #[test]
    fn it_can_construct() {
        let input = PuzzleInput::new("../aoc-03/input.txt");
        let list: BitmapList = input.to_lines().into();
        assert_eq!(list.bitmap_size, 12);
    }
}
