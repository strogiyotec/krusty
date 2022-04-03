use sqlx::{Error, Pool, Postgres};

use crate::router::stock_payload::Dividend;

pub async fn save_dividends(dividends: Vec<Dividend>, pool: &Pool<Postgres>) -> Result<(), Error> {
    let mut tx = pool.begin().await?;
    for mut dividend in dividends {
        dividend.to_toronto_exchange();
        sqlx::query(
            r#"
            INSERT INTO MM-DD'))
             ON CONFLICT DO NOTHING;
            "#
        )
            .bind(dividend.id)
            .bind(dividend.symbol)
            .bind(dividend.market_value.amount)
            .bind(dividend.effective_date.to_string())
            .execute(&mut *tx)
            .await
            .expect("Can't insert the row");
    }
    return tx.commit().await;

}
