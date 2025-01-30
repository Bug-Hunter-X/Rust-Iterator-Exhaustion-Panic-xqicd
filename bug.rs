fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();

    // This will panic if we try to get a value from an exhausted iterator
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();
    let third = iter.next().unwrap();
    let fourth = iter.next().unwrap(); // This will panic!
}