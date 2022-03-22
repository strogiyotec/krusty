use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::web::{Data, Json};
use log::info;
use serde::{Deserialize, Serialize};
use sqlx::{Encode, Error};

use crate::AppState;
use crate::entity::stock_entity::DbStock;
use crate::repository::stock_repository::{save_stocks, sector_by_ticker, stock_by_ticker};
use crate::router::stock_payload::Stock;

#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorMessage {
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SuccessMessage {
    pub message: String,
}

pub async fn save_stocks_from_wealthica(payload: Json<Vec<Stock>>, state: Data<AppState>) -> HttpResponse {
    let stocks = payload.into_inner();
    let mut db_stocks: Vec<DbStock> = Vec::with_capacity(stocks.len());
    // Transform the payload into a list of stocks which will be persisted to database
    // get the sector for each individual stock
    for stock in &stocks {
        let ticker_sector = sector_by_ticker(&stock.ticker, &state.db_conn).await.expect("Sector by ticker");
        match ticker_sector {
            None => {
                let sector = state.uni_bit_api.get_sector_by_ticker(&stock.ticker).await.expect("Ticker by Sector from API");
                //TODO: save it to database
                db_stocks.push(DbStock::new(&stock, sector));
            }
            Some(ticker_sector) => {
                db_stocks.push(DbStock::new(&stock, ticker_sector.sector));
            }
        }
    }
    return match save_stocks(stocks, &state.db_conn).await {
        Ok(()) => {
            HttpResponse::Ok().json(SuccessMessage { message: "OK".to_string() })
        }
        Err(err) => {
            HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(ErrorMessage { message: err.to_string() })
        }
    };
}
