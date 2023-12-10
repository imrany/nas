use clap::Parser;
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
use std::{
    thread,
    env::{
        current_exe
    },
    path::{Path,PathBuf}
};
mod launch;
use launch::launch_browser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the folder you want to serve
    #[arg(short, long, value_name= "PATH")]
    root: String,
}

#[actix_web::main]
async fn main() -> Result<(),std::io::Error> {
    let args = Args::parse();

    thread::spawn(||{
        async_std::task::spawn(async move {
            // launching browser
            launch_browser("http://localhost:8080").await.unwrap_or_else(|err| println!("{}",err));
        });
    });

    let server=
        HttpServer::new(move ||
            App::new()
                .service(Files::new("/",&args.root).show_files_listing().index_file("index.html")
                    .default_handler(fn_service(|req: ServiceRequest| async {
                        let (req, _) = req.into_parts();
                        let current_exe_path=PathBuf::from(current_exe().unwrap());
                        let file = NamedFile::open_async(Path::new(current_exe_path.parent().unwrap()).join("static_files/404.html")).await?;
                        let res = file.into_response(&req);
                        Ok(ServiceResponse::new(req, res))
                    }))
                )
        )
        .bind(("0.0.0.0",8080));

    match server {
        Ok(serve) => {
            serve.run().await.unwrap_or_else(|err| println!("{}",err));
        },
        Err(e) =>  println!("{}",e)
    }
    Ok(())
}