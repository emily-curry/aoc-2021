use crate::image::Image;
use aoc_core::puzzle_input::PuzzleInput;

mod image;
mod image_algorithm;
mod image_point;

fn main() {
    let input = PuzzleInput::new("aoc-20/input.txt");
    let mut image = Image::from(input.to_lines());
    while image.get_enhance_count() < 2 {
        image.enhance();
    }
    println!(
        "Count of light pixels after enhancing twice: {}",
        image.count_light()
    );

    while image.get_enhance_count() < 50 {
        image.enhance();
    }
    println!(
        "Count of light pixels after enhancing 50 times: {}",
        image.count_light()
    );
}
