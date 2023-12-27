use actix_web::{
    HttpResponse,
    Responder,
    // Result,
    // get,
    // HttpRequest,
};

// #[get("/")]
// pub async fn index(req: HttpRequest)-> Result<fs::NamedFile>{
//     // let path: PathBuf = req.match_info().query("filename").parse().unwrap();
//     // Ok(fs::NamedFile::open(path)?)
// }

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}