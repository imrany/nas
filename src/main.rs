use actix_web::{
    HttpServer,
    App,
};
use actix_files::Files;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .service(Files::new("/","/static_files").show_files_listing().index_file("/static_files/index.html")))
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}
