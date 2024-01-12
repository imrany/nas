use actix_web::{
    HttpResponse,
    Responder,
    get,
    // HttpRequest,
    web,
};
use std::{
    fs,
    path,
};
use serde::Serialize;

#[derive(Serialize)]
struct DirectoryObject {
    id: u32,
    name:String,
    path:path::PathBuf,
}

#[derive(Serialize)]
struct DirectoryContent {
    contents: Vec<DirectoryObject>,
}

pub struct AppState {
    pub root_dir: path::PathBuf,
}

#[get("/directory_content")]
pub async fn directory_content(state: web::Data<AppState>)-> HttpResponse{
    let directory_path=path::Path::new("./resources");

    let root_dir=&state.root_dir;
    println!("{:?}", root_dir);
    // Read the directory contents
    let contents = match fs::read_dir(directory_path) {
        Ok(entries) => {
            let mut contents = Vec::new();
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        let directory_object=DirectoryObject {
                            id:2,
                            name:file_name.to_owned(),
                            path:directory_path.join(file_name.to_owned())
                        };
                        contents.push(directory_object);
                    }
                }
            }
            contents
        }
        Err(_) => {
            return HttpResponse::InternalServerError().body("Failed to read directory");
        }
    };

     // Create a response containing the directory content as JSON
    let directory_content = DirectoryContent { contents };
    match serde_json::to_string(&directory_content) {
        Ok(json_string) => HttpResponse::Ok().json(json_string),
        Err(_) => HttpResponse::InternalServerError().body("Failed to serialize to JSON"),
    }
}

pub async fn hello_world() -> impl Responder { 
    HttpResponse::Ok().body("Hello world!")
}