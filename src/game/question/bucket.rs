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

impl Bucket {
    pub fn new(bucket_type: BucketTypes) -> Self {
        Bucket {
            bucket_type,
            _base_bucket: Vec::new(),
            _current_bucket: Vec::new()
        }
    }

    pub fn get_bucket_type(&mut self, bucket_type: &BucketTypes) -> Vec<u8> {
        match bucket_type {
            BucketTypes::Single => {
                vec![0,1,2,3,4,5]
            },
            BucketTypes::Multiple(size) => {
                let ret = Vec::new();
                for _ in 0..*size {
                    self._base_bucket.append(&mut vec![0,1,2,3,4,5]);
                }
                ret
            },
            BucketTypes::Incremental(size) => {
                let ret = Vec::new();
                for i in 0..*size {
                    self._base_bucket.push((i as u8) % 6);
                }
                ret
            },
            BucketTypes::Custom(custom) => {
                custom.clone()
            }
        }
    }

    
}