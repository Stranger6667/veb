use std::{collections::HashMap, mem};

pub struct VebTree {
    clusters: HashMap<usize, VebTree>,
    summary: Option<Box<VebTree>>,
    min: Option<usize>,
    max: Option<usize>,
    cluster_size: usize,
}

impl VebTree {
    #[must_use]
    pub fn new(max_element: usize) -> Self {
        Self {
            clusters: HashMap::new(),
            summary: None,
            min: None,
            max: None,
            cluster_size: (max_element as f64).sqrt() as usize,
        }
    }

    #[inline]
    const fn high(&self, value: usize) -> usize {
        value / self.cluster_size
    }

    #[inline]
    const fn low(&self, value: usize) -> usize {
        value % self.cluster_size
    }

    #[inline]
    const fn index(&self, cluster_idx: usize, in_cluster_idx: usize) -> usize {
        cluster_idx * self.cluster_size + in_cluster_idx
    }

    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.min.is_none()
    }

    /// Insert a value into the tree.
    pub fn insert(&mut self, value: usize) {
        match self.min {
            // First insertion - the tree is empty
            None => {
                self.min = Some(value);
                self.max = Some(value);
            }
            Some(min) => {
                // If `value` is lower than the existing minimum, then replace `min` with it,
                // and insert the old `min` value to the proper cluster.
                if value < min {
                    mem::swap(&mut self.min, &mut Some(value));
                }
                // Update `max` if this value is greater.
                if value > self.max.expect("Max value is set at this point") {
                    self.max = Some(value);
                }
                let high = self.high(value);
                let low = self.low(value);
                // The cluster where `value` should be placed
                let cluster = self
                    .clusters
                    .entry(high)
                    // Create an empty cluster if there is no corresponding one yet
                    .or_insert_with(|| VebTree::new(self.cluster_size));
                if cluster.is_empty() {
                    // If this cluster is empty, then we need to update the summary to reflect that
                    // this cluster is now non-empty.
                    // In this case, the `insert` call to this cluster is O(1) - see the
                    // "first insertion" block in the beginning of this function.
                    self.summary
                        .get_or_insert_with(|| Box::new(VebTree::new(self.cluster_size)))
                        .insert(high)
                }
                cluster.insert(low)
            }
        }
    }

    #[must_use]
    pub fn contains(&self, value: usize) -> bool {
        if self.min.map_or(false, |min| min == value) || self.max.map_or(false, |max| max == value)
        {
            return true;
        }
        self.clusters
            .get(&self.high(value))
            .map_or(false, |subtree| subtree.contains(self.low(value)))
    }

    #[must_use]
    pub fn find_next(&self, value: usize) -> Option<usize> {
        self.min.and_then(|min| {
            if value < min {
                Some(min)
            } else {
                let mut cluster_idx = self.high(value);
                let in_cluster_idx;
                let cluster = self.clusters.get(&cluster_idx)?;
                if self.low(value) < cluster.max? {
                    in_cluster_idx = cluster.find_next(self.low(value))?;
                } else {
                    cluster_idx = self.summary.as_ref()?.find_next(cluster_idx)?;
                    in_cluster_idx = self.clusters.get(&cluster_idx)?.min?;
                }
                Some(self.index(cluster_idx, in_cluster_idx))
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        assert!(VebTree::new(42).is_empty());
    }

    #[test]
    fn small_insert() {
        let mut tree = VebTree::new(4);
        tree.insert(2);
        assert_eq!(tree.min, Some(2));
        assert_eq!(tree.max, Some(2));
        tree.insert(3);
        assert_eq!(tree.min, Some(2));
        assert_eq!(tree.max, Some(3));
        tree.insert(1);
        assert_eq!(tree.min, Some(1));
        assert_eq!(tree.max, Some(3));
    }

    #[test]
    fn insert() {
        for max_element in 1..256 {
            let mut tree = VebTree::new(max_element);
            assert!(tree.is_empty());
            for value in 0..=max_element {
                assert!(!tree.contains(value));
                tree.insert(value);
                assert!(tree.contains(value));
                assert_eq!(tree.min, Some(0));
                assert_eq!(tree.max, Some(value));
            }
            assert!(!tree.is_empty());
        }
    }

    #[test]
    fn find_next() {
        let max_element = 50;
        let mut tree = VebTree::new(max_element);
        for value in 0..=max_element {
            assert!(tree.find_next(value).is_none());
        }
        let new_value = 25;
        tree.insert(new_value);
        for value in 0..new_value {
            assert_eq!(tree.find_next(value), Some(new_value));
        }
        for value in new_value..=max_element {
            assert!(tree.find_next(value).is_none());
        }
    }
}
