use actix_web::{
    HttpResponse,
    Responder,
    get,
};

#[get("/")]
pub async fn get_index_page() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}