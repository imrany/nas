use actix_web::{
    HttpServer,
    App,
    dev::{
        ServiceRequest, 
        ServiceResponse, 
        fn_service
    }
};
use actix_files::{
    Files,
    NamedFile
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .service(Files::new("/","/static_files").show_files_listing().index_file("index.html")
                .default_handler(fn_service(|req: ServiceRequest| async {
                    let (req, _) = req.into_parts();
                    let file = NamedFile::open_async("./static_files/404.html").await?;
                    let res = file.into_response(&req);
                    Ok(ServiceResponse::new(req, res))
                }))
            ))
            .bind(("127.0.0.1",8080))?
            .run()
            .await
}
