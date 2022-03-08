use rocket::{http::Method, serde::json::Json};
use rocket_cors::{AllowedOrigins, CorsOptions};

//{Endpoints
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

//}

pub fn cors_options() -> CorsOptions {
    return CorsOptions::default()
        .allowed_origins(AllowedOrigins::All)
        .allowed_methods(
            vec![Method::Get, Method::Post]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite(
        "JSON",
        |rocket| async {
            rocket
                .mount("/",rocket_cors::catch_all_options_routes())
                .mount("/stocks", routes![report])
                .manage(cors_options().to_cors().unwrap())
        },
    )
}
