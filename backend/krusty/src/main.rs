#[macro_use]
extern crate rocket;

mod excel;
mod models;
mod routes;

#[launch]
fn entry_point() -> _ {
    rocket::build().attach(routes::stock::stage())
}
