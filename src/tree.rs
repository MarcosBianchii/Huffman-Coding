use crate::bag::Bag;
use bitvec::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tree<T> {
    Leaf {
        freq: usize,
        item: T,
    },
    Inner {
        freq: usize,
        children: Box<[Self; 2]>,
    },
}

impl<T: Ord> PartialOrd for Tree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for Tree<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.freq().cmp(&other.freq())
    }
}

impl<T: std::hash::Hash + Eq + Ord> Tree<T> {
    pub fn new<I>(data: I) -> Tree<T>
    where
        I: IntoIterator<Item = T>,
    {
        let mut freqs: BinaryHeap<_> = Bag::from(data)
            .into_iter()
            .map(|(ch, freq)| Reverse(Tree::leaf(ch, freq)))
            .collect();

        loop {
            match (freqs.pop(), freqs.pop()) {
                (Some(Reverse(root)), None) => break root,
                (Some(Reverse(left)), Some(Reverse(right))) => {
                    let parent = Tree::inner(left, right);
                    freqs.push(Reverse(parent));
                }

                _ => unreachable!(),
            }
        }
    }
}

impl<T> Tree<T> {
    pub fn leaf(item: T, freq: usize) -> Self {
        Self::Leaf { item, freq }
    }

    pub fn inner(left: Self, right: Self) -> Self {
        Self::Inner {
            freq: left.freq() + right.freq(),
            children: Box::new([left, right]),
        }
    }

    fn freq(&self) -> usize {
        match *self {
            Self::Leaf { freq, .. } | Self::Inner { freq, .. } => freq,
        }
    }

    fn dfs<'a, F: FnMut(&'a T, &BitSlice)>(&'a self, encoding: &mut BitVec, f: &mut F) {
        match self {
            Self::Leaf { item, .. } => f(item, encoding),
            Self::Inner { children, .. } => {
                for (i, child) in children.iter().enumerate() {
                    encoding.push(i == 1);
                    child.dfs(encoding, f);
                    encoding.pop();
                }
            }
        }
    }
}

impl<T: std::hash::Hash + Clone + Eq> Tree<T> {
    pub fn encoder(&self) -> HashMap<T, BitVec> {
        let mut table = HashMap::new();
        let mut f = |item: &T, encoding: &BitSlice| {
            table.insert(item.clone(), encoding.to_bitvec());
        };

        self.dfs(&mut BitVec::new(), &mut f);
        table
    }

    pub fn decoder(&self) -> HashMap<BitVec, T> {
        let mut table = HashMap::new();
        let mut f = |item: &T, encoding: &BitSlice| {
            table.insert(encoding.to_bitvec(), item.clone());
        };

        self.dfs(&mut BitVec::new(), &mut f);
        table
    }
}
