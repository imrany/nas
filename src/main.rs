use clap::{
    Parser,
    Subcommand,
};
use actix_web::{
    HttpServer,
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
use std::{
    env::{
        current_exe
    },
    path::{Path,PathBuf}
};
use local_ip_address::local_ip;
mod launch;
use launch::launch_browser;

#[path="./routes.rs"]
mod routes;
use routes::{
    index,
};

#[cfg(target_os="linux")]
use colored::Colorize;

#[derive(Parser)]
#[command(author="Imran <imranmat254@gmail.com>", version, about="A simple http server for static files.", long_about = None)]
struct Args {
    /// Path to the folder you want to serve
    #[arg(short, long, value_name= "PATH")]
    root: Option<String>,

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

    if let Some(path) = args.root.as_deref() {
        serve_me(path.to_string()).await;
    }
    
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &args.command {
        Some(Commands::Serve { path }) => {
            if let Some(path) = path.as_deref() {
                serve_me(path.to_string()).await;
            }else {
                #[cfg(target_os="linux")]
                {
                    println!(" {} Specify a path to serve."," ERROR ".on_red().color("white"));
                    println!(" {}",format!(" HINT: To serve the current folder - 'zippy serve ./'.").cyan());
                }

                #[cfg(target_os="windows")]
                {
                    println!("  ERROR Specify a path to serve.");
                    println!(" {}",format!(" HINT: To serve the current folder - 'zippy serve ./'."));
                }
            }

        }
        None => {
            serve_zippy().await;
        }
    }
    Ok(())
}

async fn serve_zippy(){
    let static_files=Path::new(PathBuf::from(current_exe().unwrap()).parent().unwrap()).join("static_files");
    // let static_files=Path::new("./static_files");
    let server=HttpServer::new(move ||
        App::new()
            .service(Files::new("/", &static_files).index_file("index.html")
                .default_handler(fn_service(|req: ServiceRequest| async {
                    let (req, _) = req.into_parts();
                    let current_exe_path=PathBuf::from(current_exe().unwrap());
                    let file = NamedFile::open_async(Path::new(current_exe_path.parent().unwrap()).join("static_files/404.html")).await?;
                    let res = file.into_response(&req);
                    Ok(ServiceResponse::new(req, res))
                }))
            )
            // .route("/hey", web::get().to(manual_hello))
            .service(
                // prefixes all resources and routes attached to it...
                web::scope("/api")
                // ...so this handles requests for `GET /app/index.html`
                .route("/", web::get().to(index)),
            )
    )
    .bind(("0.0.0.0",8000));
    match server {
        Ok(server) => {
            let port:i32=8000;
            let url=format!("http://localhost:{port}/");
            match launch_browser(port).await {
                Ok(_) => {
                   #[cfg(not(target_os="windows"))]
                   {
                    println!(" {} Launching zippy...",format!(" INFO ").on_cyan().color("white"));
                    println!(" Open {url}");
                   }
          
                   #[cfg(target_os="windows")]
                   {
                    println!(" INFO: Launching zippy...");
                    println!(" INFO: Open {url}");
                   }
                },
                Err(e) => {
                   #[cfg(not(target_os="windows"))]
                   println!(" {} An error occurred when opening {} {e}", " ERROR ".on_red().color("white"),format!("{url}").cyan());
          
                   #[cfg(target_os="windows")]
                   println!(" ERROR: An error occurred when opening {url} {e}");
                }
             };
            server.run().await.unwrap_or_else(|err| println!(" {err} "));
        },
        Err(e) =>  println!(" {} ",e)
    }
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
            let port:i32=8080;
            let url=format!("http://localhost:{port}/");
            match launch_browser(port).await {
                Ok(_) => {
                   let my_local_ip = local_ip();
                   #[cfg(target_os="linux")]
                   println!(" Local: {}",format!("{url}").cyan());
          
                   #[cfg(target_os="windows")]
                   println!(" Local: {url}");
                   
                   match my_local_ip {
                      Ok(ip) => {
                         #[cfg(target_os="linux")]
                         println!(" Network: {}",format!("http://{ip}:{port}/").cyan());
          
                         #[cfg(target_os="windows")]
                         println!(" Network: {}",format!("http://{ip}:{port}/"));
                      },
                      Err(e) => {
                         #[cfg(target_os="linux")]
                         println!(" {} {}.", format!(" WARNING ").on_yellow().color("white"),e );
          
                         #[cfg(target_os="windows")]
                         println!(" {} {}.", format!(" WARNING "),e );
                      }
                   }
                },
                Err(_) => {
                   #[cfg(target_os="linux")]
                   println!(" {} An error occurred when opening {}", " ERROR ".on_red().color("white"),format!("{url}").cyan());
          
                   #[cfg(target_os="windows")]
                   println!(" ERROR An error occurred when opening {url}");
                }
             };
            serve.run().await.unwrap_or_else(|err| println!(" {} ",err));
        },
        Err(e) =>  println!(" {} ",e)
    }
}