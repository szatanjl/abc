use std::collections::HashMap;
use serde::Serialize;
use thiserror::Error;
use super::bounded_multi_set::BoundedMultiSet;

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
        }
    }
}

impl DataPointsInner {
    fn add(&mut self, values: &[f64]) {
        for value in values {
            for set in &mut self.sets {
                set.insert(*value);
            }
        }
        self.last = values.last().copied();
    }

    fn get(&self, k: u8) -> Result<Stats, InvalidParams> {
        if k <= 0 || k > 8 {
            return Err(InvalidParams);
        }
        let k = usize::from(k - 1);
        Ok(Stats {
            min: self.sets[k].min(),
            max: self.sets[k].max(),
            last: self.last,
            ..Stats::default()
        })
    }
}
