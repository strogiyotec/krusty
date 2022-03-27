use log::info;
use sqlx::{Error, Pool, Postgres};

use crate::entity::stock_entity::{DbStock, TickerToSector};
use crate::router;
use crate::router::stock_payload::Stock;
use crate::router::stock_router::ErrorMessage;

pub async fn save_stocks(stocks: Vec<DbStock>, pool: &Pool<Postgres>) -> Result<(), Error> {
    let mut tx = pool.begin().await?;
    for mut stock in stocks {
        let mut ticket = stock.ticket.clone();
        sqlx::query(
            r#"
            INSERT INTO public.stock_info ( ticket, cnt, sector, market_value)
             VALUES ($1,$2,$3,$4)
            "#
        )
            .bind(&ticket)
            .bind(stock.cnt)
            .bind(stock.sector)
            .bind(stock.market_value)
            .execute(&mut *tx)
            .await
            .expect("Can't insert the row");
        info!("Stock with ticker {} was saved",ticket);
    }
    return tx.commit().await;
}

pub async fn save_ticker_to_sector(ticker: &String, sector : &String, pool: &Pool<Postgres>) -> Result<(), Error> {
    let result = sqlx::query(
        r#"
            INSERT INTO public.ticker_to_sector (ticker,sector)
             VALUES ($1,$2)
        "#
    )
        .bind(ticker)
        .bind(sector)
        .execute(pool)
        .await;
    return match result {
        Err(err) => Err(err),
        Ok(_) => Ok(())
    };
}

pub async fn sector_by_ticker(ticker: &String, pool: &Pool<Postgres>) -> Result<Option<TickerToSector>, ErrorMessage> {
    let result = sqlx::query_as::<_, TickerToSector>(
        "select * from public.ticker_to_sector where ticker = $1 "
    )
        .bind(ticker)
        .fetch_optional(pool)
        .await;
    return result.map_err(|e| ErrorMessage { message: format!("{:?}", e) })
}


pub async fn stock_by_ticker(ticker: &String, pool: &Pool<Postgres>) -> Result<DbStock, Error> {
    return sqlx::query_as::<_, DbStock>(
        "select * from public.stock_info where ticket = $1 "
    )
        .bind(ticker)
        .fetch_one(pool)
        .await;
}
