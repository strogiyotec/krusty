use rocket::serde::json::Json;
use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Stock<'s> {
    ticker: &'s str,
    amount: u32,
}
