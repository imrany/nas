use actix_web::{
    HttpResponse,
    Responder,
    get,
    // HttpRequest,
};

#[get("/")]
pub async fn index()-> impl Responder{
    HttpResponse::Ok().body("Hello world!")
}

pub async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}