use std::{
    collections::{hash_map::IntoIter, HashMap},
    hash::Hash,
};

#[derive(Debug)]
pub struct Bag<T> {
    counts: HashMap<T, usize>,
}

impl<I, T> From<I> for Bag<T>
where
    T: Hash + Eq,
    I: IntoIterator<Item = T>,
{
    fn from(items: I) -> Self {
        let it = items.into_iter();
        let (lower, _) = it.size_hint();
        let mut bag = Self::with_capacity(lower);

        for item in it {
            bag.add(item);
        }

        bag
    }
}

impl<T> IntoIterator for Bag<T> {
    type IntoIter = IntoIter<T, usize>;
    type Item = (T, usize);

    fn into_iter(self) -> Self::IntoIter {
        self.counts.into_iter()
    }
}

impl<T> Bag<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            counts: HashMap::with_capacity(capacity),
        }
    }
}

impl<T: Hash + Eq> Bag<T> {
    pub fn add(&mut self, item: T) {
        *self.counts.entry(item).or_default() += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_adding_an_item_means_having_one_of_it() {
        let bag = Bag::from(['a']);
        assert_eq!(Some(&1), bag.counts.get(&'a'));
    }

    #[test]
    fn test02_adding_the_same_item_twice_sets_count_to_2() {
        let bag = Bag::from(['a', 'a']);
        assert_eq!(Some(&2), bag.counts.get(&'a'));
    }

    #[test]
    fn test03_has_different_counters_for_each_unique_item() {
        let bag = Bag::from(['a', 'b', 'c', 'a', 'A', 'b', 'b']);
        assert_eq!(Some(&2), bag.counts.get(&'a'));
        assert_eq!(Some(&3), bag.counts.get(&'b'));
        assert_eq!(Some(&1), bag.counts.get(&'c'));
        assert_eq!(Some(&1), bag.counts.get(&'d'));
    }
}
