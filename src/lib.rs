use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
mod tests;

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T>(Vec<T>);

impl<T> CustomSet<T>
where
    T: Hash + Copy + Ord,
{
    pub fn new(_input: &[T]) -> Self {
        let mut new_set = CustomSet(vec![]);
        _input.iter().for_each(|&m| new_set.add(m));
        new_set
    }

    fn hash(value: T) -> u64 {
        let mut hasher = &mut DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.0
            .iter()
            .any(|m| CustomSet::hash(m) == CustomSet::hash(_element))
    }

    pub fn add(&mut self, _element: T) {
        if !self.contains(&_element) {
            self.0.push(_element);
            self.0.sort_unstable();
        }
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        self.0.iter().all(|m| _other.contains(m))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        self.intersection(_other).is_empty()
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        CustomSet::new(
            self.0
                .iter()
                .filter(|m| _other.contains(m))
                .chain(_other.0.iter().filter(|m| self.contains(m)))
                .map(|&m| m)
                .collect::<Vec<T>>()
                .as_slice(),
        )
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        CustomSet(
            self.0
                .iter()
                .filter(|m| !_other.contains(m))
                .map(|&m| m)
                .collect(),
        )
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        CustomSet::new(
            self.0
                .iter()
                .chain(_other.0.iter())
                .map(|&m| m)
                .collect::<Vec<T>>()
                .as_slice(),
        )
    }
}
