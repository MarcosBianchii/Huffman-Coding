use crate::tree::Tree;
use bitvec::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EncodedData<T> {
    tree: Tree<T>,
    bits: BitVec,
}

impl<T> EncodedData<T> {
    pub fn new(tree: Tree<T>, bits: BitVec) -> Self {
        Self { tree, bits }
    }

    pub fn destructure(self) -> (Tree<T>, BitVec) {
        (self.tree, self.bits)
    }
}

impl<T: Serialize> EncodedData<T> {
    pub fn into_bytes(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

impl<T> EncodedData<T>
where
    for<'a> T: Deserialize<'a>,
{
    pub fn from_bytes(bytes: &[u8]) -> bincode::Result<Self> {
        bincode::deserialize(bytes)
    }
}
