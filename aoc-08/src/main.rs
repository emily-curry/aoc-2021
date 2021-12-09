use aoc_core::puzzle_input::PuzzleInput;
use std::collections::HashMap;

fn main() {
    let input = PuzzleInput::new("aoc-08/input.txt");
    let mut segment_displays: Vec<SegmentDisplay> =
        input.to_lines().map(SegmentDisplay::from).collect();
    let mut count_unique_segments = 0u32;
    let mut sum_outputs = 0u32;
    for display in segment_displays.iter_mut() {
        display.decode();
        let decoded = display.get_decoded();
        for digit in &decoded {
            match digit {
                1 | 4 | 7 | 8 => count_unique_segments += 1,
                _ => {}
            }
        }
        let output = decoded.iter().rev().enumerate().fold(0u32, |acc, val| {
            let digit = *val.1 as u32;
            acc + (10u32.pow(val.0 as u32) * digit)
        });
        sum_outputs += output;
    }
    println!(
        "Occurrences of digits with unique # of segments: {}",
        count_unique_segments
    );
    println!("Sum of all outputs: {}", sum_outputs);
}

#[derive(Debug)]
struct SegmentDisplay {
    // The list of 10 unique signal inputs that will show on this display
    notes: Vec<SegmentDigit>,
    // The list of digits to decode
    output: Vec<SegmentDigit>,
}

impl SegmentDisplay {
    pub fn new(notes: Vec<SegmentDigit>, output: Vec<SegmentDigit>) -> Self {
        SegmentDisplay { notes, output }
    }

    pub fn decode(&mut self) {
        self.decode_notes();
        self.decode_output();
    }

    pub fn get_decoded(&self) -> Vec<u8> {
        self.output
            .iter()
            .map(|out| out.get_decoded().unwrap_or(10)) // 10 is a temp value while we're solving the entire thing.
            .collect()
    }

    fn decode_notes(&mut self) {
        self.decode_digit_by_segment_len(2, 1); // Decode the `1` digit.
        self.decode_digit_by_segment_len(4, 4); // Decode the `4` digit.
        self.decode_digit_by_segment_len(3, 7); // Decode the `7` digit.
        self.decode_digit_by_segment_len(7, 8); // Decode the `8` digit.

        let segment_occurrences = self.get_segment_occurrences();

        let segment_top = self.get_segment_top();
        let segment_bot_right = self.get_segment_bot_right(&segment_occurrences);
        let segment_bot_left = self.get_segment_bot_left(&segment_occurrences);
        let segment_top_right = self.get_segment_top_right(&segment_occurrences, segment_top);
        let segment_top_left = self.get_segment_top_left(&segment_occurrences);
        let segment_middle =
            self.get_segment_middle(segment_top_left, segment_top_right, segment_bot_right);
        let segment_bottom = self.get_segment_bottom(&segment_occurrences, segment_middle);

        self.decode_digit_by_segments(
            vec![
                segment_top,
                segment_top_left,
                segment_top_right,
                segment_bottom,
                segment_bot_right,
                segment_bot_left,
            ],
            0,
        ); // Decode the `0` digit.
        self.decode_digit_by_segments(
            vec![
                segment_top,
                segment_top_right,
                segment_middle,
                segment_bottom,
                segment_bot_left,
            ],
            2,
        ); // Decode the `2` digit.
        self.decode_digit_by_segments(
            vec![
                segment_top,
                segment_top_right,
                segment_middle,
                segment_bottom,
                segment_bot_right,
            ],
            3,
        ); // Decode the `3` digit.
        self.decode_digit_by_segments(
            vec![
                segment_top,
                segment_top_left,
                segment_middle,
                segment_bottom,
                segment_bot_right,
            ],
            5,
        ); // Decode the `5` digit.
        self.decode_digit_by_segments(
            vec![
                segment_top,
                segment_top_left,
                segment_middle,
                segment_bottom,
                segment_bot_left,
                segment_bot_right,
            ],
            6,
        ); // Decode the `6` digit.
        self.decode_digit_by_segments(
            vec![
                segment_top,
                segment_top_left,
                segment_top_right,
                segment_middle,
                segment_bottom,
                segment_bot_right,
            ],
            9,
        ); // Decode the `9` digit.
    }

    fn decode_output(&mut self) {
        for item in self.output.iter_mut() {
            let note = self
                .notes
                .iter()
                .find(|n| *n == item)
                .expect("No segment in notes that matches the output!");
            if let Some(i) = note.get_decoded() {
                item.set_decoded(i);
            }
        }
    }

    fn decode_digit_by_segment_len(&mut self, segment_len: usize, digit: u8) {
        let seg_digit = self
            .notes
            .iter_mut()
            .find(|dig| dig.get_segments_len() == segment_len)
            .expect(format!("No segment digit with {} segments!", segment_len).as_str());
        seg_digit.set_decoded(digit);
    }

    fn decode_digit_by_segments(&mut self, segments: Vec<Segment>, digit: u8) {
        let match_digit = SegmentDigit::new(segments.clone());
        let seg_digit = self
            .notes
            .iter_mut()
            .find(|dig| **dig == match_digit)
            .expect(format!("No segment digit with segments: {:?}", &segments).as_str());
        seg_digit.set_decoded(digit);
    }

    /// The top segment is the segment that 7 has that 1 doesn't
    fn get_segment_top(&self) -> Segment {
        let one = self
            .notes
            .iter()
            .find(|x| x.get_decoded().unwrap_or(10) == 1)
            .expect("No decoded value for 1!");
        let seven = self
            .notes
            .iter()
            .find(|x| x.get_decoded().unwrap_or(10) == 7)
            .expect("No decoded value for 7!");
        *seven
            .segments
            .iter()
            .find(|seg| one.segments.iter().all(|s2| s2 != *seg))
            .expect("")
    }

    /// The bottom right segment is the only segment that appears 9 times.
    fn get_segment_bot_right(&self, map: &HashMap<Segment, u8>) -> Segment {
        map.iter()
            .find(|x| *x.1 == 9)
            .map(|x| *x.0)
            .expect("No segment occurs 9 times!")
    }

    /// The bottom left segment is the only segment that appears 4 times.
    fn get_segment_bot_left(&self, map: &HashMap<Segment, u8>) -> Segment {
        map.iter()
            .find(|x| *x.1 == 4)
            .map(|x| *x.0)
            .expect("No segment occurs 4 times!")
    }

    /// The top right segment is the only segment that appears 8 times, and is not the top segment.
    fn get_segment_top_right(&self, map: &HashMap<Segment, u8>, top: Segment) -> Segment {
        map.iter()
            .find(|x| *x.1 == 8 && *x.0 != top)
            .map(|x| *x.0)
            .expect("No segment occurs 8 times!")
    }

    /// The top left segment is the only segment that appears 6 times.
    fn get_segment_top_left(&self, map: &HashMap<Segment, u8>) -> Segment {
        map.iter()
            .find(|x| *x.1 == 6)
            .map(|x| *x.0)
            .expect("No segment occurs 6 times!")
    }

    /// The middle segment is the segment in 4 that is not the top left, top right, or bot right.
    fn get_segment_middle(
        &self,
        top_left: Segment,
        top_right: Segment,
        bot_right: Segment,
    ) -> Segment {
        let four = self
            .notes
            .iter()
            .find(|x| x.get_decoded().unwrap_or(10) == 4)
            .expect("No decoded value for 4!");
        four.segments
            .iter()
            .find(|seg| **seg != top_left && **seg != top_right && **seg != bot_right)
            .map(Segment::clone)
            .expect("No middle segment found!")
    }

    /// The bottom segment is the only segment that appears 7 times, and is not the middle.
    fn get_segment_bottom(&self, map: &HashMap<Segment, u8>, middle: Segment) -> Segment {
        map.iter()
            .find(|x| *x.1 == 7 && *x.0 != middle)
            .map(|x| *x.0)
            .expect("No segment occurs 7 times!")
    }

    fn get_segment_occurrences(&self) -> HashMap<Segment, u8> {
        let mut occurrences: HashMap<Segment, u8> = HashMap::new();
        for item in &self.notes {
            for segment in &item.segments {
                let next_value = occurrences.get(segment).map(|i| *i).unwrap_or(0) + 1;
                occurrences.insert(*segment, next_value);
            }
        }
        occurrences
    }
}

impl From<&str> for SegmentDisplay {
    fn from(input: &str) -> Self {
        let mut split: Vec<Vec<SegmentDigit>> = input
            .split(" | ")
            .map(|inner| inner.split(' ').map(SegmentDigit::from).collect())
            .collect();
        let notes = split.remove(0);
        let output = split.remove(0);
        SegmentDisplay::new(notes, output)
    }
}

#[derive(Debug)]
struct SegmentDigit {
    segments: Vec<Segment>,
    decoded: Option<u8>,
}

impl SegmentDigit {
    pub fn new(segments: Vec<Segment>) -> Self {
        SegmentDigit {
            segments,
            decoded: None,
        }
    }

    pub fn get_segments_len(&self) -> usize {
        self.segments.len()
    }

    pub fn get_decoded(&self) -> Option<u8> {
        self.decoded
    }

    pub fn set_decoded(&mut self, value: u8) {
        self.decoded = Some(value);
    }
}

impl From<&str> for SegmentDigit {
    fn from(input: &str) -> Self {
        let segments: Vec<Segment> = input.chars().map(Segment::from).collect();
        SegmentDigit::new(segments)
    }
}

impl PartialEq for SegmentDigit {
    fn eq(&self, other: &Self) -> bool {
        self.get_segments_len() == other.get_segments_len()
            && self.segments.iter().all(|seg| other.segments.contains(seg))
    }
}

impl Eq for SegmentDigit {}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl From<char> for Segment {
    fn from(input: char) -> Self {
        match input {
            'a' => Segment::A,
            'b' => Segment::B,
            'c' => Segment::C,
            'd' => Segment::D,
            'e' => Segment::E,
            'f' => Segment::F,
            'g' => Segment::G,
            _ => panic!("Invalid value: {}", input),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Segment, SegmentDigit};

    #[test]
    fn can_eq_segments() {
        let seg_a = Segment::A;
        let seg_b = Segment::B;
        let seg_b_2 = Segment::B;
        assert_ne!(seg_a, seg_b);
        assert_eq!(seg_a, seg_a);
        assert_eq!(seg_b, seg_b_2);
    }

    #[test]
    fn can_eq_segment_digits() {
        let dig_1 = SegmentDigit::new(vec![Segment::B, Segment::C]);
        let dig_2 = SegmentDigit::new(vec![Segment::B, Segment::C, Segment::F]);
        let dig_3 = SegmentDigit::new(vec![Segment::F, Segment::C, Segment::B]);

        assert_ne!(dig_1, dig_2);
        assert_ne!(dig_1, dig_3);
        assert_eq!(dig_3, dig_2);
    }
}
