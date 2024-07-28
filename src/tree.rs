use crate::{error::HuffErr, Bag, BitVec, Result};
use serde::{Deserialize, Serialize};
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tree<T> {
    freq: usize,
    kind: Kind<T>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
enum Kind<T> {
    Leaf { token: T },
    Inner { children: Box<[Tree<T>; 2]> },
}

impl<T: Ord> PartialOrd for Tree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for Tree<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.freq.cmp(&other.freq)
    }
}

impl<T: Hash + Ord> Tree<T> {
    pub fn new<I>(data: I) -> Result<Tree<T>>
    where
        I: IntoIterator<Item = T>,
    {
        let mut freqs: BinaryHeap<_> = Bag::from(data)
            .into_iter()
            .map(|(ch, freq)| Reverse(Tree::leaf(ch, freq)))
            .collect();

        loop {
            let Reverse(left) = freqs.pop().ok_or(HuffErr::InputIsEmpty)?;

            match freqs.pop() {
                Some(Reverse(right)) => {
                    let parent = Tree::inner(left, right);
                    freqs.push(Reverse(parent));
                }

                None => break Ok(left),
            }
        }
    }
}

impl<T> Tree<T> {
    fn leaf(token: T, freq: usize) -> Self {
        Self {
            freq,
            kind: Kind::Leaf { token },
        }
    }

    fn inner(left: Self, right: Self) -> Self {
        Self {
            freq: left.freq + right.freq,
            kind: Kind::Inner {
                children: Box::new([left, right]),
            },
        }
    }
}

impl<T: Hash + Eq> Tree<T> {
    fn dfs(self, encoding: &mut BitVec, encoder: &mut HashMap<T, BitVec>) {
        match self.kind {
            Kind::Leaf { token } => {
                encoder.insert(token, encoding.clone());
            }

            Kind::Inner { children } => {
                let [left, right] = *children;

                encoding.push(false);
                left.dfs(encoding, encoder);
                encoding.pop();

                encoding.push(true);
                right.dfs(encoding, encoder);
                encoding.pop();
            }
        }
    }

    pub fn encoder(self) -> HashMap<T, BitVec> {
        let mut table = HashMap::new();
        self.dfs(&mut BitVec::new(), &mut table);
        table
    }
}
