use std::collections::{hash_map::IntoIter, HashMap};

#[derive(Debug)]
pub struct Bag<T> {
    table: HashMap<T, usize>,
}

impl<I, T> From<I> for Bag<T>
where
    T: std::hash::Hash + Eq,
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
        self.table.into_iter()
    }
}

impl<T> Bag<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            table: HashMap::with_capacity(capacity),
        }
    }
}

impl<T: std::hash::Hash + Eq> Bag<T> {
    pub fn add(&mut self, item: T) {
        *self.table.entry(item).or_default() += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_adding_an_item_means_having_one_of_it() {
        let bag = Bag::from(['a']);
        assert_eq!(Some(&1), bag.table.get(&'a'));
    }

    #[test]
    fn test02_adding_the_same_item_twice_sets_count_to_2() {
        let bag = Bag::from(['a', 'a']);
        assert_eq!(Some(&2), bag.table.get(&'a'));
    }

    #[test]
    fn test03_has_different_counters_for_each_unique_item() {
        let bag = Bag::from(['a', 'b', 'c', 'a', 'A', 'b', 'b']);
        assert_eq!(Some(&2), bag.table.get(&'a'));
        assert_eq!(Some(&3), bag.table.get(&'b'));
        assert_eq!(Some(&1), bag.table.get(&'c'));
        assert_eq!(Some(&1), bag.table.get(&'d'));
    }
}
