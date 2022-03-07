use rocket::serde::json::Json;
use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Stock<'s> {
    pub ticker: &'s str,
    pub amount: u32,
}
