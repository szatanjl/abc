use bounded_vec_deque::BoundedVecDeque;
use ordered_float::NotNan;

pub struct BoundedPrefixSums(BoundedVecDeque<f64>);

impl BoundedPrefixSums {
    pub fn new(max_len: usize) -> Self {
        let mut sums = BoundedVecDeque::new(max_len + 1);
        sums.push_back(0.0);
        Self(sums)
    }

    pub fn push(&mut self, value: f64) {
        // Treat NaN as 0 when calculating sums
        let value =
            self.0.back().copied().unwrap_or_default() +
            NotNan::new(value).unwrap_or_default().into_inner();
        self.0.push_back(value);
    }

    pub fn get_sum(&self, n: usize) -> Option<(f64, usize)> {
        if self.0.len() <= 0 || n <= 0 {
            None
        } else if n < self.0.len() {
            let first_sum = self.0.get(self.0.len() - n - 1).copied().unwrap_or_default();
            let last_sum = self.0.back().copied().unwrap_or_default();
            Some((last_sum - first_sum, n))
        } else {
            let first_sum = self.0.front().copied().unwrap_or_default();
            let last_sum = self.0.back().copied().unwrap_or_default();
            Some((last_sum - first_sum, self.0.len() - 1))
        }
    }
}
