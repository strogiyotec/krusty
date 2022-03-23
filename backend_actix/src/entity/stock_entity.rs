use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::router::stock_payload::Stock;

#[derive(FromRow, Serialize, Deserialize)]
pub struct TickerToSector {
    pub ticker: String,
    pub sector: String,
}

impl TickerToSector{
    pub fn new(ticker: String, sector: String) -> Self {
        Self { ticker, sector }
    }
}


#[derive(FromRow, Serialize, Deserialize)]
pub struct DbStock {
    pub id: i32,
    pub ticket: String,
    pub cnt: u32,
    pub sector: String,
    pub market_value: f64,
}

impl DbStock {
    pub fn new(payload: &Stock, sector: String) -> Self {
        Self { id: 0, ticket: payload.ticker.clone(), cnt: payload.cnt, sector, market_value:payload.market_value }
    }
}
