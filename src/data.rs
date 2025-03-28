use std::collections::HashMap;
use serde::Serialize;
use thiserror::Error;

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

#[derive(Default)]
struct DataPointsInner {}

impl DataPointsInner {
    fn add(&mut self, values: &[f64]) {
        println!("ADD {:?}", values);
    }

    fn get(&self, k: u8) -> Result<Stats, InvalidParams> {
        if k <= 0 || k > 8 {
            return Err(InvalidParams);
        }
        println!("GET {}", k);
        Ok(Stats::default())
    }
}
