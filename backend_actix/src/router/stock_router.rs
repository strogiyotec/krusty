use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::web::{Data, Json};
use serde::{Deserialize, Serialize};
use sqlx::Error;

use crate::AppState;
use crate::repository::stock_repository::{save_stocks, stock_by_ticker};
use crate::router::stock_payload::Stock;

#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorMessage {
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SuccessMessage {
    pub message: String,
}

pub async fn save_stocks_from_wealthica(stocks: Json<Vec<Stock>>, state: Data<AppState>) -> HttpResponse {
    return match save_stocks(stocks.into_inner(), &state.db_conn).await {
        Ok(()) => {
            HttpResponse::Ok().json(SuccessMessage { message: "OK".to_string() })
        }
        Err(err) => {
            HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(ErrorMessage { message: err.to_string() })
        }
    };
}
