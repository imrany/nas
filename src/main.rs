use clap::{
    Parser,
    Subcommand,
};
use rand::{thread_rng, Rng};
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
    hello_world,
    open_file_by_name,
    open_file_by_name_local,
    get_external_ip,
};

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
    let args = Args::parse();
    if let Some(path) = args.root.as_deref() {
        serve_me(path.to_string()).await;
    }
    
    match &args.command {
        Some(Commands::Serve { path }) => {
            if let Some(path) = path.as_deref() {
                serve_me(path.to_string()).await;
            }else {
                println!("  ERROR Specify a path to serve.");
                println!(" {}",format!(" HINT: To serve the current folder - 'zippy serve ./'."));
            }

        }
        None => {
            serve_zippy().await;
        }
    }
}

async fn serve_zippy(){
    // let path: PathBuf = Path::new(PathBuf::from(current_exe().unwrap()).parent().unwrap()).join("static_files");
    let path =Path::new("./static_files");
    let app_state = web::Data::new(AppState {
        root_dir: get_root_directory().unwrap(),
    });
    let port:u16=8000;
    let ipv4: (Ipv4Addr, u16)=("0.0.0.0".parse().unwrap(),port);
    let server=HttpServer::new(move ||{
        let app_state = app_state.clone();
        App::new()
            .app_data(app_state.clone()) 
            .service(
                web::scope("/api")
                    .route("", web::get().to(hello_world))
                    .service(directory_content)
                    .service(get_external_ip)
                    .service(open_file_by_name)
            )
            .service(
                web::scope("/local")
                    .service(open_file_by_name_local)
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
            let url=format!("http://localhost:{port}/");
            if let Err(e)= launch_browser(&url).await{
                println!(" ERROR: An error occurred when opening {url} {e}");
            }else {
                println!(" INFO: Launching zippy...");
                println!(" INFO: Open {url}");
            }
            server.run().await.unwrap_or_else(|err| println!(" ERROR: {err} "));
        },
        Err(e) =>  println!(" {} ",e)
    }
}

async fn serve_me(path: String) {
    let port:u16=thread_rng().gen_range(3000..=8080);
    let ipv4: (Ipv4Addr, u16)=("0.0.0.0".parse().unwrap(),port);
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
    .bind(ipv4);
    match server {
        Ok(serve) => {
            let url=format!("http://localhost:{port}/");
            if let Err(e) = launch_browser(&url).await {
                println!(" ERROR An error occurred when opening {url} {e}");
            }else{
                let my_local_ip = local_ip();
                println!(" Local: {url}");
                
                match my_local_ip {
                    Ok(ip) => {
                        println!(" Network: {}",format!("http://{ip}:{port}/"));
                    },
                    Err(e) => {
                        println!(" {} {}.", format!(" WARNING "),e );
                    }
                }
            };
            serve.run().await.unwrap_or_else(|err| println!(" ERROR {} ",err));
        },
        Err(e) =>  println!(" {} ",e)
    }
}