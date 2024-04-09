use rand::{seq::SliceRandom, Rng};

pub enum BucketTypes {
    Single,
    Multiple(usize),
    Incremental(usize),
    Custom(Vec<u8>)
}

pub struct Bucket {
    pub bucket_type: BucketTypes,
    _base_bucket: Vec<u8>,
    _current_bucket: Vec<u8>
}