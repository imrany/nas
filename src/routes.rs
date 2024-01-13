use actix_web::{
    HttpResponse,
    Responder,
    get,
    HttpRequest,
    web,
    Result
};
use actix_files::NamedFile;
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

pub struct AppState {
    pub root_dir: path::PathBuf,
}

#[get("/directory_content")]
pub async fn directory_content(state: web::Data<AppState>)-> HttpResponse{
    let directory_path=path::Path::new("./resources");

    let root_dir=&state.root_dir.as_path();
    println!("{}", root_dir.display());
    // Read the directory contents
    let contents = match fs::read_dir(directory_path) {
        Ok(entries) => {
            let mut contents = Vec::new();
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        let metadata= FileMeta{
                            is_file:directory_path.join(file_name.to_owned()).is_file(),
                            file_extension: if directory_path.join(file_name.to_owned()).is_file() {
                                Some(format!("{}",directory_path.join(file_name.to_owned()).extension().unwrap().to_str().unwrap()))
                            }else{
                                Some(String::from("Folder"))
                            },
                        };
                        let directory_object=DirectoryObject {
                            id:2,
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

#[get("/{filename:.*}")]
pub async fn open_file_by_name(req: HttpRequest) -> Result<NamedFile> {
    let path: path::PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[get("/local/{filename:.*}")]
pub async fn open_file_by_name_local(req: HttpRequest) -> impl Responder {
    let path: path::PathBuf = req.match_info().query("filename").parse().unwrap();
    if let Ok(file) = fs::File::open(path) {
        println!("{:?}", file);
        return HttpResponse::Ok().json("Hello world!");
    }else {
        return HttpResponse::InternalServerError().json("Failed to open file");
    };
}

pub async fn hello_world() -> impl Responder { 
    HttpResponse::Ok().body("Hello world!")
}