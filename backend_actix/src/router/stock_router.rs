use actix_web::error::ParseError::Status;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::web::{Data, Json};
use log::info;
use serde::{Deserialize, Serialize};
use sqlx::{Encode, Error};

use crate::AppState;
use crate::entity::stock_entity::{DbStock, TickerToSector};
use crate::repository::dividend_repository::save_dividends;
use crate::repository::stock_repository::{delete_all, save_stocks, save_ticker_to_sector, sector_by_ticker, stock_by_ticker};
use crate::router::stock_payload::{Dividend, Stock};

#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorMessage {
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SuccessMessage {
    pub message: String,
}


pub async fn post_dividends(payload: Json<Vec<Dividend>>, state: Data<AppState>) -> HttpResponse {
    let dividends = payload.into_inner();
    return match save_dividends(dividends, &state.db_conn).await {
        Ok(()) => {
            HttpResponse::Ok().json(SuccessMessage { message: "OK".to_string() })
        }
        Err(err) => {
            HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(ErrorMessage { message: err.to_string() })
        }
    };
}

pub async fn save_stocks_from_wealthica(payload: Json<Vec<Stock>>, state: Data<AppState>) -> HttpResponse {
    if let Err(_) = delete_all(&state.db_conn).await {
        return HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(ErrorMessage { message: "Can't clean up rows".to_string() });
    }
    let stocks = payload.into_inner();
    let mut db_stocks: Vec<DbStock> = Vec::with_capacity(stocks.len());
    // Transform the payload into a list of stocks which will be persisted to database
    // get the sector for each individual stock
    for mut stock in stocks {
        stock.to_toronto_exchange();
        let ticker_sector = sector_by_ticker(&stock.ticker, &state.db_conn).await.expect("Sector by ticker");
        match ticker_sector {
            None => {
                let sector = state
                    .uni_bit_api
                    .get_sector_by_ticker(&stock.ticker)
                    .await
                    .expect("Ticker by Sector from API");
                let clone_sector = sector.clone();
                save_ticker_to_sector(&stock.ticker, &sector, &state.db_conn)
                    .await
                    .expect("Can't save sector to ticker");
                db_stocks.push(DbStock::new(&stock, clone_sector));
            }
            Some(ticker_sector) => {
                db_stocks.push(DbStock::new(&stock, ticker_sector.sector));
            }
        }
    }
    return match save_stocks(db_stocks, &state.db_conn).await {
        Ok(()) => {
            HttpResponse::Ok().json(SuccessMessage { message: "OK".to_string() })
        }
        Err(err) => {
            HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(ErrorMessage { message: err.to_string() })
        }
    };
}
