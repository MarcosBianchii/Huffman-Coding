use crate::{BitVec, HuffErr, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct EncodedData<T> {
    pub decoder: HashMap<BitVec, T>,
    bits: BitVec,
}

impl<T> EncodedData<T> {
    pub fn new(decoder: HashMap<BitVec, T>, bits: BitVec) -> Self {
        Self { decoder, bits }
    }

    pub fn destructure(self) -> (HashMap<BitVec, T>, BitVec) {
        (self.decoder, self.bits)
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
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        bincode::deserialize(bytes).map_err(|_| HuffErr::InvalidBytes)
    }
}
