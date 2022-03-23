use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Stock {
    pub ticker: String,
    pub cnt: u32,
    pub market_value: f64,
}

impl Stock {
    pub fn to_toronto_exchange(&mut self) {
        self.ticker += ".TO";
    }
}
