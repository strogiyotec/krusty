use sqlx::FromRow;
use serde::{Serialize,Deserialize};

#[derive(FromRow,Serialize,Deserialize)]
pub struct DbStock {
    id: i32,
    ticket: String,
    cnt: i32,
    sector: String,
    total_volume: i32,
}
