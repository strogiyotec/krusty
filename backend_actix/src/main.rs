use std::env;
use std::io::Error;

use actix_web::{App, HttpServer, middleware, Result, web};
use log::log;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

use crate::router::stock_router::save_stocks_from_wealthica;
extern crate env_logger;

mod router;
mod entity;
mod repository;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //{Logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    //}
    //{Postgres starts
    let db_url = env::var("DATABASE_URL").expect("DB_URL env variable must be present");
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(db_url.as_str())
        .await
        .expect("Database connection failed");
    let app_state = AppState{db_conn:pool};
    //}
    //{start http server
    HttpServer::new(
        move || App::new()
            .wrap(middleware::Logger::default())
            .route("/stocks", web::post().to(save_stocks_from_wealthica))
            .app_data(web::Data::new(app_state.clone()))
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    //}
}

