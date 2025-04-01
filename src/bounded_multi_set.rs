use std::collections::BTreeMap;
use bounded_vec_deque::BoundedVecDeque;
use ordered_float::NotNan;

pub struct BoundedMultiSet {
    queue: BoundedVecDeque<f64>,
    set: BTreeMap<NotNan<f64>, u32>,
}

impl BoundedMultiSet {
    pub fn new(max_len: usize) -> Self {
        Self {
            queue: BoundedVecDeque::new(max_len),
            set: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, value: f64) {
        let removed = self.queue.push_back(value).map(NotNan::new);
        // Ignore NaN values when searching for min and max values
        if let Ok(value) = NotNan::new(value) {
            *self.set.entry(value).or_default() += 1;
            if let Some(Ok(removed)) = removed {
                if let Some(v) = self.set.get_mut(&removed) {
                    if *v <= 1 {
                        self.set.remove(&removed);
                    } else {
                        *v -= 1;
                    }
                }
            }
        }
    }

    pub fn min(&self) -> Option<f64> {
        self.set.first_key_value().map(|(k, _)| k.into_inner())
    }

    pub fn max(&self) -> Option<f64> {
        self.set.last_key_value().map(|(k, _)| k.into_inner())
    }
}
