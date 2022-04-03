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


#[derive(Deserialize, Serialize, Debug)]
pub struct Dividend{
    pub id: String,
    pub symbol: String,
    pub market_value: MarketValue,

    pub effective_date: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MarketValue {
    pub amount : f64,
}

impl Dividend {
    pub fn to_toronto_exchange(&mut self) {
        self.symbol += ".TO";
    }
}
