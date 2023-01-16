// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    return (dividend / divisor, dividend % divisor);
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    return iter.enumerate().filter_map(|(i, val)| {
        let (_, remainer) = divmod(i as i16, 2);
        (remainer == 0).then_some(val)
    });
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        // m = |x1 - x2| + |y1 - y2|
        return (0 - self.0).abs() + (0 - self.1).abs()
    }
}
