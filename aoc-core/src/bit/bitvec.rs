use crate::bit::bitmap::Bitmap;

#[derive(Debug)]
pub struct BitVec<B = u16> {
    maps: Vec<Bitmap<B>>,
}

impl BitVec {}
