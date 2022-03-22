use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::router::stock_payload::Stock;

#[derive(FromRow, Serialize, Deserialize)]
pub struct DbStock {
    id: i32,
    ticket: String,
    cnt: i32,
    sector: String,
    total_volume: i32,
}

impl DbStock {
    pub fn new(payload: Stock, sector: String) -> Self {
        Self { id: 0, ticket, cnt, sector, total_volume }
    }
}
