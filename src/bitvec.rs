use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct BitVec {
    data: Vec<u8>,
    size: usize,
}

impl Debug for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bits = self
            .data
            .iter()
            .map(|byte| format!("{byte:08b}"))
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "[{bits}]")
    }
}

impl BitVec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, bit: bool) {
        if self.size % 8 == 0 {
            self.data.push(0);
        }

        if bit {
            if let Some(last) = self.data.last_mut() {
                *last |= 0x80 >> (self.size % 8);
            }
        }

        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<bool> {
        let last = self.data.last_mut()?;

        self.size -= 1;
        let mask = 0x80 >> (self.size % 8);

        let bit = *last & mask;
        *last &= !mask;

        if self.size % 8 == 0 {
            self.data.pop();
        }

        Some(bit != 0)
    }

    pub fn get(&self, idx: usize) -> Option<bool> {
        if idx >= self.size {
            return None;
        }

        let byte = idx >> 3;
        let mask = 0x80 >> (idx % 8);
        let bit = self.data[byte] & mask;

        Some(bit != 0)
    }

    pub fn extend(&mut self, other: &Self) {
        for bit in other {
            self.push(bit);
        }
    }

    pub fn to_bitvec(&self) -> Self {
        Self {
            data: self.data.clone(),
            size: self.size,
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.size = 0;
    }
}

impl IntoIterator for BitVec {
    type Item = bool;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { vec: self, pos: 0 }
    }
}

pub struct IntoIter {
    vec: BitVec,
    pos: usize,
}

impl Iterator for IntoIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let bit = self.vec.get(self.pos);
        self.pos += 1;
        bit
    }
}

impl<'a> IntoIterator for &'a BitVec {
    type Item = bool;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { vec: self, pos: 0 }
    }
}

pub struct Iter<'a> {
    vec: &'a BitVec,
    pos: usize,
}

impl Iterator for Iter<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let bit = self.vec.get(self.pos);
        self.pos += 1;
        bit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_adding_up_to_size_of_vec_t_accumulates_the_values_in_the_first_position_of_data() {
        let mut vec = BitVec::new();
        vec.push(true);
        assert_eq!(1, vec.size);
        assert_eq!(vec![0b_10000000], vec.data);

        vec.push(true);
        assert_eq!(2, vec.size);
        assert_eq!(vec![0b_11000000], vec.data);

        vec.push(false);
        assert_eq!(3, vec.size);
        assert_eq!(vec![0b_11000000], vec.data);

        vec.push(true);
        assert_eq!(4, vec.size);
        assert_eq!(vec![0b_11010000], vec.data);

        vec.push(false);
        assert_eq!(5, vec.size);
        assert_eq!(vec![0b_11010000], vec.data);

        vec.push(true);
        assert_eq!(6, vec.size);
        assert_eq!(vec![0b_11010100], vec.data);

        vec.push(true);
        assert_eq!(7, vec.size);
        assert_eq!(vec![0b_11010110], vec.data);

        vec.push(true);
        assert_eq!(8, vec.size);
        assert_eq!(vec![0b_11010111], vec.data);

        println!("{vec:?}");
    }

    #[test]
    fn test02_adding_one_more_extends_the_vec_by_1() {
        let mut vec = BitVec {
            data: vec![0b_11111111],
            size: 8,
        };

        vec.push(true);
        assert_eq!(9, vec.size);
        assert_eq!(2, vec.data.len());
        assert_eq!(vec![0b_11111111, 0b_10000000], vec.data);
    }

    #[test]
    fn test03_removing_in_the_same_byte_removes_the_last_bit_of_the_byte() {
        let mut vec = BitVec {
            data: vec![0b_11111011],
            size: 8,
        };

        assert_eq!(Some(true), vec.pop());
        assert_eq!(7, vec.size);
        assert_eq!(1, vec.data.len());

        assert_eq!(Some(true), vec.pop());
        assert_eq!(6, vec.size);
        assert_eq!(1, vec.data.len());

        assert_eq!(Some(false), vec.pop());
        assert_eq!(5, vec.size);
        assert_eq!(1, vec.data.len());

        assert_eq!(Some(true), vec.pop());
        assert_eq!(4, vec.size);
        assert_eq!(1, vec.data.len());

        println!("{vec:?}");
    }

    #[test]
    fn test04_removing_the_last_bit_of_a_byte_reduces_the_vec_by_1() {
        let mut vec = BitVec {
            data: vec![0b_11111111, 0b_10000000],
            size: 9,
        };

        assert_eq!(Some(true), vec.pop());
        assert_eq!(8, vec.size);
        assert_eq!(1, vec.data.len());
    }

    #[test]
    fn test05_iterator() {
        let vec = BitVec {
            data: vec![0b_01010101, 0b_001100_00],
            size: 14,
        };

        let mut iter = vec.into_iter();

        assert_eq!(Some(false), iter.next());
        assert_eq!(Some(true), iter.next());
        assert_eq!(Some(false), iter.next());
        assert_eq!(Some(true), iter.next());
        assert_eq!(Some(false), iter.next());
        assert_eq!(Some(true), iter.next());
        assert_eq!(Some(false), iter.next());
        assert_eq!(Some(true), iter.next());

        assert_eq!(Some(false), iter.next());
        assert_eq!(Some(false), iter.next());
        assert_eq!(Some(true), iter.next());
        assert_eq!(Some(true), iter.next());
        assert_eq!(Some(false), iter.next());
        assert_eq!(Some(false), iter.next());

        assert_eq!(None, iter.next());
    }
}
