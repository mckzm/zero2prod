use actix_web::{web, HttpResponse};

// TODO temporarily silencing Clippy's warning about FormData's unused fields
// TODO rm once we have a proper implementation of `subscribe`.
#[derive(serde::Deserialize)]
pub struct FormData {
    #[allow(unused)]
    email: String,
    #[allow(unused)]
    name: String,
}

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
