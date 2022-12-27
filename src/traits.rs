use std::ops::RangeInclusive;

pub trait RangeExtensions<Rhs = Self> {
    fn includes(&self, other: &Rhs) -> bool;
    fn overlaps(&self, other: &Rhs) -> bool;
}

impl<Idx: PartialOrd<Idx>> RangeExtensions for RangeInclusive<Idx> {
    fn includes(&self, other: &RangeInclusive<Idx>) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.start() <= other.end() && self.end() >= other.start()) ||
            (other.start() <= self.end() && other.end() >= self.start())
    }
}

pub trait StringExtensions {
    fn to_usize_vector(&self) -> Vec<usize>;
    fn to_int_vector(&self) -> Vec<i32>;
    fn tokens(&self) -> Vec<&str>;
}

impl StringExtensions for str {

    fn to_usize_vector(&self) -> Vec<usize> {
        self.split(|c: char| !(c.is_ascii_digit()))
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect()
    }

    fn to_int_vector(&self) -> Vec<i32> {
        self.split(|c: char| !(c.is_ascii_digit() || c == '-'))
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect()
    }

    fn tokens(&self) -> Vec<&str> {
        self.split(|c: char| c.is_whitespace())
            .filter(|s| !s.is_empty())
            .collect()
    }
}
