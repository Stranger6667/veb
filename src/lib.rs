use std::collections::HashMap;
use std::mem;

pub struct VebTree {
    clusters: HashMap<usize, VebTree>,
    summary: Option<Box<VebTree>>,
    min: Option<usize>,
    max: Option<usize>,
    cluster_size: usize,
}

impl VebTree {
    pub fn new(max_element: usize) -> Self {
        let cluster_size = (max_element as f64).sqrt().floor() as usize;
        let summary = if max_element == 1 {
            None
        } else {
            Some(Box::new(Self::new(cluster_size)))
        };
        VebTree {
            clusters: HashMap::new(),
            summary,
            min: None,
            max: None,
            cluster_size,
        }
    }

    #[inline]
    fn high(&self, value: usize) -> usize {
        value / self.cluster_size
    }

    #[inline]
    fn low(&self, value: usize) -> usize {
        value % self.cluster_size
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.min.is_none()
    }

    pub fn insert(&mut self, mut value: usize) {
        match self.min {
            None => {
                self.min = Some(value);
                self.max = Some(value);
            }
            Some(mut min) => {
                if value < min {
                    mem::swap(&mut min, &mut value)
                }
                if value > self.max.expect("Max value is not set") {
                    self.max = Some(value)
                }
                let high = self.high(value);
                if let Some(cluster) = self.clusters.get(&high) {
                    if cluster.min.is_none() {
                        let summary = &mut **self.summary.as_mut().unwrap();
                        summary.insert(high);
                    }
                }
                let low = self.low(value);
                let entry = self
                    .clusters
                    .entry(high)
                    .or_insert_with(|| VebTree::new(self.cluster_size));
                entry.insert(low);
            }
        }
    }

    pub fn contains(&self, value: usize) -> bool {
        if self.min.map_or(false, |min| min == value) || self.max.map_or(false, |max| max == value)
        {
            return true;
        }
        self.clusters
            .get(&self.high(value))
            .map_or(false, |subtree| subtree.contains(self.low(value)))
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
    fn insert() {
        let max_element = 1024 * 16;
        let mut tree = VebTree::new(max_element);
        assert!(tree.is_empty());
        for value in 0..=max_element {
            assert!(!tree.contains(value));
            tree.insert(value);
            assert!(tree.contains(value));
        }
        assert!(!tree.is_empty());
    }
}
