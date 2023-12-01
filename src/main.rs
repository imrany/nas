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
        args,
        current_exe
    },
    process,
    path::{Path,PathBuf}
};
mod launch;
use launch::launch_browser;

#[actix_web::main]
async fn main() -> Result<(),std::io::Error> {
    thread::spawn(||{
        async_std::task::spawn(async move {
            // launching browser
            launch_browser("http://localhost:8080").await.unwrap_or_else(|err| println!("{}",err));
        });
    });

    let args: Vec<String>=args().collect();
    // let flag=&args[1];
    if args.len()<2 {
        println!("Problem parsing arguments: Enter path \nexample: live_server /home/username/Desktop/static");
        process::exit(1);
    }


    let file_path = args[1].clone();
    let server=
        HttpServer::new(move ||
            App::new()
                .service(Files::new("/",&file_path).show_files_listing().index_file("index.html")
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