use actix_web::{
    HttpResponse,
    Responder,
    get,
    post,
    web,
    Result
};
use actix_files::NamedFile;
use std::{
    fs,
    path,
};
use serde::{
    Serialize,
    Deserialize,
};
use std::process::Command;
use local_ip_address::local_ip;
use reqwest;

#[derive(Serialize)]
struct DirectoryObject {
    root:String,
    name:String,
    path:path::PathBuf,
    metadata:FileMeta
}
#[derive(Serialize)]
struct FileMeta{
    is_file:bool,
    file_extension:Option<String>,
}

#[derive(Serialize)]
struct DirectoryContent {
    contents: Vec<DirectoryObject>,
}

#[derive(Serialize)]
struct ErrorMessage{
    message: String,
}

#[derive(Serialize)]
struct Ip {
    internal: String,
    external: String,
}

#[derive(Deserialize, Clone)]
struct RootPath{
    root: path::PathBuf,
}

pub struct AppState {
    pub root_dir: path::PathBuf,
}

#[post("/directory_content")]
pub async fn directory_content(state: web::Data<AppState>, path: web::Json<RootPath>)-> HttpResponse{
    let root =&state.root_dir;
    let path_dir=&path.root;

    #[cfg(target_os="windows")]
    let path_dir_win=format!("{}",path_dir.clone().to_str().unwrap());
    #[cfg(target_os="windows")]
    let dir_path_win=path::PathBuf::from(path_dir_win.as_str());
    
    #[cfg(not(target_os="windows"))]
    let path_dir_unix=format!("/{}",path_dir.clone().to_str().unwrap());
    #[cfg(not(target_os="windows"))]
    let dir_path_unix=path::PathBuf::from(path_dir_unix.as_str());

    let directory_path = match path_dir.to_str().unwrap() {
        "root" => {
            println!("{}", root.to_str().unwrap());
            root
        },
        _ => {
            println!("{}", path_dir.to_str().unwrap());
            #[cfg(target_os="windows")]
            return &dir_path_win;
            
            #[cfg(not(target_os="windows"))]
            &dir_path_unix
        }
    };
    
    // Read the directory contents
    let contents = match fs::read_dir(directory_path) {
        Ok(entries) => {
            let mut contents = Vec::new();
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        let metadata= FileMeta{
                            is_file:directory_path.join(file_name.to_owned()).is_file(),
                            file_extension: Some(String::from("none"))
                            // file_extension:match directory_path.join(file_name.to_owned()).is_file() {
                            //     true => Some(format!("{}",directory_path.join(file_name.to_owned()).extension().unwrap().to_str().unwrap())),
                            //     false => Some(String::from("Folder")),
                            // }
                        };
                        let directory_object=DirectoryObject {
                            root:format!("{}",directory_path.to_str().unwrap()),
                            name:file_name.to_owned(),
                            path:directory_path.join(file_name.to_owned()),
                            metadata
                        };
                        contents.push(directory_object);
                    }
                }
            }
            contents
        }
        Err(_) => {
            let err_message=ErrorMessage{
                message:format!("Cannot find the folder named '{}'",directory_path.to_str().unwrap())
            };
            return HttpResponse::InternalServerError().json(err_message);
        }
    };

    let directory_content = DirectoryContent { contents };
    HttpResponse::Ok().json(&directory_content)
}

#[get("/{path}")]
pub async fn open_file_by_name(path: web::Path<String>) -> Result<NamedFile> {
    #[cfg(not(target_os="windows"))]
    let file_path= format!("/{}",path.into_inner());

    #[cfg(target_os="windows")]
    let file_path= format!("{}",path.into_inner());
    
    // let path: path::PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(file_path)?)
}

#[post("/{path}")]
pub async fn open_file_by_name_local(path: web::Path<String>) -> impl Responder {
    #[cfg(not(target_os="windows"))]
    let file_path= format!("/{}",path.into_inner());

    #[cfg(target_os="windows")]
    let file_path= format!("{}",path.into_inner());

    // On Windows, use the "start" command to open the file with the default program
    #[cfg(target_os="windows")]
    {
        let open_cmd=Command::new("cmd")
            .args(&["/C", "start", "", &file_path.as_str()])
            .spawn();

        if let Ok(file) = open_cmd {
            println!("{:?}", file);
            return HttpResponse::Ok().json("File opened");
        }else {
            return HttpResponse::InternalServerError().json("Failed to open file");
        };
    }
    // On Linux or macOS, use "xdg-open" to open the file with the default program
    #[cfg(not(target_os="windows"))]
    {
        let open_cmd=Command::new("xdg-open")
            .arg(&file_path.as_str())
            .spawn();
            
        if let Ok(file) = open_cmd {
            println!("{:?}", file);
            return HttpResponse::Ok().json("File opened");
        }else {
            return HttpResponse::InternalServerError().json("Failed to open file");
        };
    }
}

#[get("/get_ip_address")]
pub async fn get_ip_address()-> impl Responder {
    if let Ok(internal_ip) = local_ip() {
        // Make a request to httpbin to get the external IP address
        if let Ok(response) = reqwest::get("https://httpbin.org/ip").await{
            // Parse the JSON response to extract the IP address
            let ip_address: serde_json::Value = response.json().await.unwrap();
            let ip_external = ip_address["origin"].as_str().unwrap_or("Unknown");
            let ip=Ip{
                internal: internal_ip.to_string(),
                external: ip_external.to_string()
            };
            let json_response=serde_json::to_string(&ip).unwrap();
            return HttpResponse::Ok().json(json_response);
        }else {
            let ip=Ip{
                internal: internal_ip.to_string(),
                external: "No internet".to_string()
            };
            let json_response = serde_json::to_string(&ip).unwrap();
            return HttpResponse::Ok().json(json_response);
        };
    }else {
        let err_message=ErrorMessage{
            message: "Connect to a wifi or hotspot".to_string()
        };
        let json_response = serde_json::to_string(&err_message).unwrap();
        return HttpResponse::InternalServerError().json(json_response);
    }
}

pub async fn hello_world() -> impl Responder { 
    HttpResponse::Ok().body("Hello world!")
}