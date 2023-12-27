use actix_web::{
    HttpResponse,
    Responder,
    Result,
    get,
    HttpRequest,
};
use actix_files as fs;
use std::path::PathBuf;

#[get("/{filename:.*}")]
pub async fn index(req: HttpRequest)-> Result<fs::NamedFile>{
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(fs::NamedFile::open(path)?)
}

#[get("/")]
pub async fn get_index_page() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}