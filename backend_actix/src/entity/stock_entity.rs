use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::router::stock_payload::Stock;

#[derive(FromRow, Serialize, Deserialize)]
pub struct DbStock {
    id: i32,
    ticket: String,
    cnt: u32,
    sector: String,
    market_value: f64,
}

impl DbStock {
    pub fn new(payload: Stock, sector: String) -> Self {
        Self { id: 0, ticket: payload.ticker, cnt: payload.cnt, sector, total_volume:payload.market_value }
    }
}
