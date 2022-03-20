use sqlx::{Error, Pool, Postgres};

use crate::entity::stock_entity::DbStock;
use crate::router;
use crate::router::stock_payload::Stock;

pub async fn save_stocks(stocks: Vec<Stock>, pool: &Pool<Postgres>) -> Result<(), Error> {
    let mut tx = pool.begin().await?;
    for stock in stocks {
        sqlx::query(
            r#"
            INSERT INTO stock_info ( ticket, cnt, sector, market_value)
             VALUES ($1,$2,$3,$4)
            "#
        )
            .bind(stock.ticker)
            .bind(stock.cnt)
            .bind("BANK")
            .bind(stock.market_value)
            .execute(&mut *tx)
            .await
            .expect("Can't insert the row");
    }
    return  tx.commit().await;
}

pub async fn stock_by_ticker(ticker: &String, pool: &Pool<Postgres>) -> Result<DbStock, Error> {
    return sqlx::query_as::<_, DbStock>(
        "select * from stock_info where ticket = $1 "
    )
        .bind(ticker)
        .fetch_one(pool)
        .await;
}
