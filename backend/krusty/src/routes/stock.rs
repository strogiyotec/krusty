use rocket::serde::json::Json;

//TODO: check that it works
//TODO: Add excel dependency
#[post("/", format = "json", data = "<stocks>")]
fn report(
    stocks: Json<
        Vec<crate::models::stocks::Stock>,
    >,
) -> &'static str {
    println!("{:?}", stocks);
    return "Hello world";
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite(
        "JSON",
        |rocket| async {
            rocket
                .mount("/stocks", routes![report])
        },
    )
}
