use serde::Serialize;
use thiserror::Error;

#[derive(Default)]
pub struct DataPoints {}

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
    pub fn add(&mut self, symbol: &str, values: &[f64]) {
        println!("ADD {} {:?}", symbol, values);
    }

    pub fn get(&self, symbol: &str, k: u8) -> Result<Stats, InvalidParams> {
        if k <= 0 || k > 8 {
            return Err(InvalidParams);
        }
        println!("GET {} {}", symbol, k);
        Ok(Stats::default())
    }
}
