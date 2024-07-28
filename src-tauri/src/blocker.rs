enum DataTypes {
    Base,
    BitVec
}

pub trait Blocker {
    fn block(
        &self,
        sequence: Vec<<T>>,
        bits_per_segment: usize,
        bits_per_overlap: usize,
    ) -> Vec<Vec<T>>;
}

pub struct DNABlocker {}
impl Blocker for DNABlocker {
    fn block(
        &self,
        sequence: Vec<crate::Base>,
        bits_per_segment: usize,
        bits_per_overlap: usize,
    ) -> Vec<Vec<crate::Base>> {
        todo!()
    }
}
