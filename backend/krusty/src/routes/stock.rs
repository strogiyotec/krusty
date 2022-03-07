use rocket::serde::json::Json;

//TODO: check that it works
//TODO: Add excel dependency
#[post(
    "/report",
    format = "json",
    data = "<stocks>"
)]
fn report(
    stocks: Json<
        Vec<crate::models::stocks::Stock>,
    >,
) -> &'static str {
    match crate::excel::stock_export::generate_report(stocks){
        Ok(()) =>return "All good",
        Err(error)=>{
            let format = format!("{:?}",error).to_owned();
            return Box::leak(format.into_boxed_str());
        }


    }
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
