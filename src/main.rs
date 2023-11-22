use actix_web::{
    HttpServer,
    HttpRequest,
    Result,
    App,
    get
};
use actix_files::NamedFile;
use std::path::PathBuf;

#[get("/{filename:.*}")]
async fn server_static(req: HttpRequest)-> Result<NamedFile>{
    let path: PathBuf=req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .service(server_static)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}
