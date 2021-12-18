use aoc_core::bit::bitmap::Bitmap;
use aoc_core::bit::bitmap_list::BitmapList;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::new("aoc-03/input.txt");
    let sub_computer = SubmarineComputer::new(input.to_lines().into());
    let list_size = sub_computer.get_data_size();

    let gamma = sub_computer.get_count_bitmap(|c| c >= list_size as u32 / 2);
    let epsilon = !gamma; // ! is "not" on things like integers too!
    let product_gamma_epsilon: u32 = u32::from(gamma) * u32::from(epsilon);
    println!("Gamma-Epsilon product: {:?}", product_gamma_epsilon);

    let oxy = sub_computer.get_life_support(|count, size| count * 2 >= size);
    let co2 = sub_computer.get_life_support(|count, size| count * 2 < size);
    let product_oxy_co2 = u32::from(oxy) * u32::from(co2);
    println!("Oxygen-CO2 product: {:?}", product_oxy_co2);
}

struct SubmarineComputer {
    /// The internal collection of bitmaps.
    data: BitmapList,
    /// A collection of integer counts, where the index corresponds to the bit position (right-most first),
    /// and the element corresponds to the number of 1s in that bit position.
    bit_counts: Vec<u32>,
}

impl SubmarineComputer {
    pub fn new(data: BitmapList) -> Self {
        let bit_counts = SubmarineComputer::calc_bit_counts(&data);
        SubmarineComputer { data, bit_counts }
    }

    /// Returns the number of elements in the internal collection.
    pub fn get_data_size(&self) -> usize {
        self.data.list.len()
    }

    /// Counts the number of bits in each position of the bitmap.
    /// The resulting Vec contains integer bit counts, where the index corresponds to the bit position (right-most first).
    fn calc_bit_counts(bitmap_list: &BitmapList) -> Vec<u32> {
        let mut bit_counts = vec![0u32; bitmap_list.bitmap_size];
        for bitmap in &bitmap_list.list {
            for pos in 0..bitmap_list.bitmap_size {
                if bitmap.get(pos) {
                    bit_counts[pos] += 1;
                }
            }
        }
        bit_counts
    }

    /// Calculates a new bitmap according to the given test, where each bit represents whether or not the test returned true for that bit position.
    /// The argument passed to `test` is the number of 1s in the bit position being tested.
    fn calc_count_bitmap<F: Fn(u32) -> bool>(bit_counts: &Vec<u32>, test: F) -> Bitmap {
        let mut bitmap = Bitmap::new(0, bit_counts.len());
        for (index, count) in bit_counts.iter().enumerate() {
            if test(*count) == true {
                bitmap.set(index, true);
            }
        }
        bitmap
    }

    /// Calculates a new bitmap according to the given test, where each bit represents whether or not the test returned true for that bit position.
    /// The argument passed to `test` is the number of 1s in the bit position being tested.
    pub fn get_count_bitmap<F: Fn(u32) -> bool>(&self, test: F) -> Bitmap {
        SubmarineComputer::calc_count_bitmap(&self.bit_counts, test)
    }

    /// Recursively calculates a life support value according to the given test.
    ///
    /// The first argument passed to `test` is the number of 1s in the bit position being tested.
    /// The second argument passed to `test` is the total number of elements in the bitmap list being tested.
    ///
    /// For each step, this function generates a bitmap describing the results of `test` at each bit position for the provided `bitmap_list`.
    /// Then, a new bitmap list is generated containing only elements whose bit at `pos` matches the generated bitmap.
    /// If this list contains exactly one element, that element is returned, otherwise the function is called again with the filtered list and `pos - 1`.
    /// Panics if `pos` becomes negative or the filtered list becomes empty.
    fn calc_life_support<F: Fn(u32, u32) -> bool>(
        bitmap_list: &BitmapList,
        pos: usize,
        test: F,
    ) -> Bitmap {
        let bit_counts = SubmarineComputer::calc_bit_counts(bitmap_list);
        let bitmap_ref = SubmarineComputer::calc_count_bitmap(&bit_counts, |count| {
            test(count, bitmap_list.list.len() as u32)
        });
        let ref_value = bitmap_ref.get(pos);
        let filtered: Vec<Bitmap> = bitmap_list
            .list
            .iter()
            .filter(|b| b.get(pos) == ref_value)
            .map(|x| x.clone())
            .collect();
        if filtered.len() == 1 {
            return filtered.first().unwrap().clone();
        }
        if filtered.is_empty() {
            panic!("Filtered down to an empty list!")
        }
        let filtered_list = BitmapList::new(filtered, bitmap_list.bitmap_size);
        SubmarineComputer::calc_life_support(&filtered_list, pos - 1, test)
    }

    /// Returns a single element from the internal bitmap list according to the [life support computation rules](https://adventofcode.com/2021/day/3) and the provided `test`.
    ///
    /// The first argument passed to `test` is the number of 1s in the bit position being tested.
    /// The second argument passed to `test` is the total number of elements in the bitmap list being tested.
    pub fn get_life_support<F: Fn(u32, u32) -> bool>(&self, test: F) -> Bitmap {
        SubmarineComputer::calc_life_support(&self.data, self.data.bitmap_size, test)
    }
}
