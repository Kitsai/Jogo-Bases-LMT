use super::*;

#[test]
fn test_default_bucket() {
    let bucket = Bucket::new(BucketTypes::Multiple(2));
    assert_eq!(bucket._base_bucket, vec![0,1,2,3,4,5,0,1,2,3,4,5]);
    assert_eq!(bucket._current_bucket.len(), 12);
}

#[test]
fn test_custom_bucket() {
    let bucket = Bucket::new(BucketTypes::Custom(vec![1,2,3,4,5]));
    assert_eq!(bucket._base_bucket, vec![1,2,3,4,5]);
    assert_eq!(bucket._current_bucket.len(), 5);
}

#[test]
fn test_get_next() {
    let mut bucket = Bucket::new(BucketTypes::Multiple(1));
    let expected = vec![0,1,2,3,4,5];
    for i in 0..6 {
        assert_eq!(bucket._current_bucket.len(), 6 - i);
        let n = bucket.get_next();
        assert_eq!(expected.contains(&n), true);
    }
}

#[test]
fn test_value() {
    let mut bucket = Bucket::new(BucketTypes::Multiple(1));
    let mut v = 0;
    for _ in 0..6 {
        v += bucket.get_next();
    }
    assert_eq!(v, 15);

    bucket.change_type(BucketTypes::Custom(vec![1,1,1,1,1]));
    v = 0;
    for _ in 0..5 {
        v += bucket.get_next();
    }
    assert_eq!(v, 5);

    bucket.change_type(BucketTypes::Multiple(2));
    v = 0;
    for _ in 0..12 {
        v += bucket.get_next();
    }
    assert_eq!(v, 30);
}