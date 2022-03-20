use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Stock {
    pub ticker: String,
    pub cnt: u32,
    pub market_value: f64,
}
