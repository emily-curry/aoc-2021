pub trait BitSize {
    /// Returns the number of bits a value is represented by.
    fn bit_size(&self) -> usize;
}
