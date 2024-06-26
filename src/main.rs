// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dirs;
use actix_web::{
    HttpServer,
    http,
    App,
    dev::{
        ServiceRequest, 
        ServiceResponse, 
        fn_service
    },
    web,
};
use actix_files::{
    Files,
    NamedFile
};
use actix_cors::Cors;
use std::{
    env::{
        current_exe
    },
    net::Ipv4Addr,
    path::{Path,PathBuf}
};
use local_ip_address::local_ip;
mod launch;
use launch::launch_browser;

#[path="./routes.rs"]
mod routes;
use routes::{
    AppState,
    directory_content,
    open_file,
    get_ip_address,
    send,
    receive,
    download,
    websocket,
    ping,
    pong
    // get_shared_folder_contents,
};


fn get_root_directory() -> Option<PathBuf> {
    // On Unix-like systems (Linux, macOS), the root directory is "/"
    #[cfg(unix)]
    {
        Some(PathBuf::from("/"))
    }

    // On Windows, the root directory is "C:\" or another drive letter
    #[cfg(windows)]
    {
        Some(PathBuf::from(r"C:\"))
    }

    // Add more platform-specific cases as needed

    // For unsupported platforms, return None
    #[cfg(not(any(unix, windows)))]
    {
        None
    }
}

#[actix_web::main]
async fn main(){
    serve_anvel().await;
}

async fn serve_anvel(){
    // Create the '/home/username/Downloads/Anvel shared' directory if it doesn't exist
    let mut shared_dir=PathBuf::new();
    shared_dir.push(dirs::download_dir().unwrap().display().to_string());   
    shared_dir.push("Anvel shared");
    tokio::fs::create_dir_all(shared_dir.to_str().unwrap()).await.unwrap();

    // let path: PathBuf = Path::new(PathBuf::from(current_exe().unwrap()).parent().unwrap()).join("static_files");
    let path =Path::new("./static_files");
    let app_state = web::Data::new(AppState {
        root_dir: get_root_directory().unwrap(),
        home_dir:dirs::home_dir().unwrap(),
        download_dir:dirs::download_dir().unwrap(),
        shared_dir:shared_dir
    });
    let port:u16=80;
    let ipv4: (Ipv4Addr, u16)=("0.0.0.0".parse().unwrap(),port);
    let server=HttpServer::new(move ||{
        let app_state = app_state.clone();
        let cors=Cors::default()
            .allow_any_origin() // Specify the allowed origin or for all us /"*"/
            .allowed_methods(vec!["GET", "POST","PATCH","PUT"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT, http::header::CONTENT_TYPE])
            .max_age(3600); // Set the maximum age of the preflight request in seconds

        App::new()
            .wrap(cors)
            .app_data(app_state.clone()) 
            .service(
                web::scope("/api")
                    .service(directory_content)
                    .service(get_ip_address)
                    .service(open_file)
                    .service(send)
                    .service(receive)
                    .service(download)
                    .service(ping)
                    .service(pong)
                    .service(websocket)
                    // .service(get_shared_folder_contents)
            )
            .service(
                web::scope("/*")
                    .service(Files::new("/", &path).index_file("index.html")
                        .default_handler(fn_service(|req: ServiceRequest| async {
                            let (req, _) = req.into_parts();
                            // let current_exe_path=PathBuf::from(current_exe().unwrap());
                            let file = NamedFile::open_async(Path::new("./static_files/index.html")).await?;
                            let res = file.into_response(&req);
                            Ok(ServiceResponse::new(req, res))
                        }))
                    )
            )
    })
    .bind(ipv4);
    match server {
        Ok(server) => {
            let url=format!("http://{}:{port}/",local_ip().unwrap());
            if let Err(e)= launch_browser(&url).await{
                println!(" ERROR: An error occurred when opening {url} {e}");
            }else {
                println!(" INFO: Launching anvel...");
                println!(" INFO: Open {url}");
            }
            server.run().await.unwrap_or_else(|err| println!(" ERROR: {err} "));
        },
        Err(e) =>  println!(" {} ",e)
    }
}
