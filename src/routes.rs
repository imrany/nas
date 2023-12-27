use actix_web::{
    HttpResponse,
    Responder,
    Result,
    get,
    // HttpRequest,
};
use actix_files as fs;
use std::{
    path::{
        // PathBuf,
        Path,
    },
    // env::{
    //     current_exe
    // },
};

#[get("/")]
pub async fn index()-> Result<fs::NamedFile>{
    // let path: PathBuf = Path::new(PathBuf::from(current_exe().unwrap()).parent().unwrap()).join("static_files/index.html");
    let path =Path::new("./static_files/index.html");
    Ok(fs::NamedFile::open(path)?)
}

pub async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}