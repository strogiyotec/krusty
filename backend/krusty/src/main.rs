#[macro_use]
extern crate rocket;

mod models;
mod routes;

#[launch]
fn entryPoint() -> _ {
    rocket::build().attach(routes::stock::stage())
}
