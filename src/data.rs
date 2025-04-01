use std::collections::HashMap;
use serde::Serialize;
use thiserror::Error;
use super::{
    bounded_multi_set::BoundedMultiSet,
    bounded_prefix_sums::BoundedPrefixSums,
};

#[derive(Default)]
pub struct DataPoints(HashMap<String, DataPointsInner>);

#[derive(Default, Serialize)]
pub struct Stats {
    min: Option<f64>,
    max: Option<f64>,
    last: Option<f64>,
    avg: Option<f64>,
    var: Option<f64>,
}

#[derive(Debug, Error)]
#[error("Invalid params provided")]
pub struct InvalidParams;

impl DataPoints {
    pub fn add(&mut self, symbol: String, values: &[f64]) {
        self.0.entry(symbol).or_default().add(values)
    }

    pub fn get(&self, symbol: &str, k: u8) -> Result<Stats, InvalidParams> {
        match self.0.get(symbol) {
            Some(dp) => dp.get(k),
            None => Ok(Stats::default()),
        }
    }
}

struct DataPointsInner {
    sets: [BoundedMultiSet; 8],
    last: Option<f64>,
    prefix_sums: BoundedPrefixSums,
    prefix_sums2: BoundedPrefixSums,
}

impl Default for DataPointsInner {
    fn default() -> Self {
        Self {
            sets: [
                BoundedMultiSet::new(10),
                BoundedMultiSet::new(100),
                BoundedMultiSet::new(1_000),
                BoundedMultiSet::new(10_000),
                BoundedMultiSet::new(100_000),
                BoundedMultiSet::new(1_000_000),
                BoundedMultiSet::new(10_000_000),
                BoundedMultiSet::new(100_000_000),
            ],
            last: None,
            prefix_sums: BoundedPrefixSums::new(100_000_000),
            prefix_sums2: BoundedPrefixSums::new(100_000_000),
        }
    }
}

impl DataPointsInner {
    fn add(&mut self, values: &[f64]) {
        for value in values {
            for set in &mut self.sets {
                set.insert(*value);
            }
            self.prefix_sums.push(*value);
            self.prefix_sums2.push(*value * *value);
        }
        self.last = values.last().copied();
    }

    fn get(&self, k: u8) -> Result<Stats, InvalidParams> {
        if k <= 0 || k > 8 {
            return Err(InvalidParams);
        }
        let n = 10usize.pow(u32::from(k));
        let k = usize::from(k - 1);
        let avg = self.prefix_sums.get_sum(n).map(|(v, n)| v / (n as f64));
        let var = {
            if let Some(avg) = avg {
                let (sum, n1) = self.prefix_sums.get_sum(n).unwrap_or((0.0, 1));
                let (sum2, n2) = self.prefix_sums2.get_sum(n).unwrap_or((0.0, 1));
                assert_eq!(n1, n2);
                Some(avg * avg + (sum2 - 2.0 * avg * sum) / (n1 as f64))
            } else {
                None
            }
        };
        Ok(Stats {
            min: self.sets[k].min(),
            max: self.sets[k].max(),
            last: self.last,
            avg,
            var,
        })
    }
}
