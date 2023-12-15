use clap::{
    Parser,
    Subcommand,
};
use local_ip_address::local_ip;
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
use colored::Colorize;
use std::{
    thread,
    env::{
        current_exe
    },
    path::{Path,PathBuf}
};
mod launch;
use launch::launch_browser;

#[derive(Parser)]
#[command(author="Imran <imranmat254@gmail.com>", version, about="A simple http server for static files.", long_about = None)]
struct Args {
    /// Path to the folder you want to serve
    #[arg(short, long, value_name= "PATH")]
    root: Option<String>,

    /// Get local IpAddr v4
    #[arg(short, long)]
    ip:Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Serves a specific folder.
    Serve {
        path: Option<String>,
    },
}

#[actix_web::main]
async fn main() -> Result<(),std::io::Error> {
    let args = Args::parse();

    if let Some(ip) = args.ip.as_deref() {
        println!(" Value for ip: {ip}");
        let my_local_ip = local_ip().unwrap();
        println!(" This is my local IP address: {:?}", my_local_ip);
    }


    thread::spawn(||{
        async_std::task::spawn(async move {
            launch_browser("http://localhost:8080/").await.unwrap_or_else(|err| println!(" {} {}"," ERROR ".on_red().color("white"),err));
        });
    });

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
   
    match &args.command {
        Some(Commands::Serve { path }) => {
            if let Some(path) = path.as_deref() {
                serve_me(path.to_string()).await;
            }else {
                println!(" {} Specify a path to serve."," ERROR ".on_red().color("white"));
                println!(" {}",format!("HINT: To serve the current folder - 'zippy serve ./'.").cyan());
            }

        }
        None => {}
    }

    // if let Some(path) = args.root.as_deref() {
    //     println!("gt {path}");
    // }
    
    Ok(())
}

async fn serve_me(path: String) {
    let server=HttpServer::new(move ||
        App::new()
            .service(Files::new("/", path.clone()).show_files_listing().index_file("index.html")
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
}