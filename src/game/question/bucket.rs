use core::fmt;

use rand::seq::SliceRandom;

pub enum BucketTypes {
    Multiple(usize),
    Custom(Vec<u8>)
}

impl fmt::Display for BucketTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BucketTypes::Multiple(size) => write!(f, "{} pacotes padrao", size),
            BucketTypes::Custom(custom) => write!(f, "Pacotes custom: {:?}", custom)
        }
    }
}

pub struct Bucket {
    pub bucket_type: BucketTypes,
    _base_bucket: Vec<u8>,
    _current_bucket: Vec<u8>
}

impl Bucket {   
    pub fn new(bucket_type: BucketTypes) -> Self {
        let base  = Self::get_bucket_of_type(&bucket_type);
        let mut rng = rand::thread_rng();
        let mut current = base.clone();
        current.shuffle(&mut rng);
        Bucket {
            bucket_type,
            _base_bucket: base,
            _current_bucket: current
        }
    }

    fn get_bucket_of_type(bucket_type: &BucketTypes) -> Vec<u8> {
        match bucket_type {
            BucketTypes::Multiple(size) => {
                let mut ret = Vec::new();
                for _ in 0..*size {
                    ret.append(&mut vec![0,1,2,3,4,5]);
                }
                ret
            },
            BucketTypes::Custom(custom) => {
                custom.clone()
            }
        }
    }

    pub fn get_next(&mut self) -> u8 {
        match self._current_bucket.pop() {
            Some(n) => n,
            None => {
                self.reset();
                self.get_next()
            }
        }
    }

    fn reset(&mut self) {
        self._current_bucket = self._base_bucket.clone();
        let mut rng = rand::thread_rng();
        self._current_bucket.shuffle(&mut rng);
    }

    pub fn change_type(&mut self, bucket_type: BucketTypes) {
        self._base_bucket = Self::get_bucket_of_type(&bucket_type);
        self.bucket_type = bucket_type;
        self.reset();
    }
}