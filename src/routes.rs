use actix_web::{
    HttpResponse,
    Responder,
    get,
    post,
    web,
    Result
};
use reqwest::{
    Client,
    multipart::{
        Form, 
        Part
    }
};
use actix_files::NamedFile;
use actix_multipart::Multipart;
use futures_util::stream::StreamExt;
use tokio::{
    io::AsyncWriteExt,
    fs::{
        File,
        read,
    }
};
use std::{
    fs,
    // fs::File,
    path, 
};
use serde::{
    Serialize,
    Deserialize,
};
use std::process::Command;
use local_ip_address::local_ip;

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

#[derive(Deserialize, Clone)]
struct SendInfo{
    file_path: path::PathBuf,
    receiving_server: String
}

pub struct AppState {
    pub root_dir: path::PathBuf,
}

#[post("/directory_content")]
pub async fn directory_content(state: web::Data<AppState>, path: web::Json<RootPath>)-> HttpResponse{
    let root =&state.root_dir;
    let path_dir=&path.root;

    let directory_path = match path_dir.to_str().unwrap() {
        "root" => {
            println!("{}", root.to_str().unwrap());
            root
        },
        _ => {
            println!("{}", path_dir.to_str().unwrap());
            path_dir
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

#[get("/download/{filename}")]
pub async fn download(path: web::Path<RootPath>) -> Result<NamedFile> {
    let file_path= format!("shared/{}",&path.root.to_str().unwrap());
    Ok(NamedFile::open(file_path)?)
}

#[post("/receive")]
pub async fn receive(mut payload: Multipart) -> Result<HttpResponse> {
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disposition = field.content_disposition().clone();
        let filename = content_disposition.get_filename().unwrap_or_default();

        // Create a file with a unique name in the server's current directory
        let filepath = format!("./shared/{}", filename);
        let mut file = File::create(&filepath).await?;

        // Write file content
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            file.write_all(&data).await?;
        }

        println!("File saved: {}", filename);
    }

    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}

#[post("/send")]
pub async fn send(resource: web::Json<SendInfo>)-> HttpResponse{
    // Path to the media file you want to send
    // let file_path = "path/to/your/media/file.jpg";
    let file_path = &resource.file_path;

    // URL of the server that will receive the file
    // let server_url = "https://example.com/upload";
    let server_url = &resource.receiving_server;

    // Read the file asynchronously
    let file_content = read(file_path).await.unwrap();


    // Create a multipart form with the file
    let form = Form::new()
        .part("file", Part::bytes(file_content).file_name("file.jpg"));



    // Send the multipart form to the server
    let response = Client::new()
        .post(server_url)
        .multipart(form)
        .send()
        .await
        .unwrap(); 

    // Check the server's response
    if response.status().is_success() {
        println!("File uploaded successfully!");
        return HttpResponse::Ok().json("File uploaded successfully!");
    } else {
        println!("Failed to upload file. Status code: {}", response.status());
        let err_message=ErrorMessage{
            message:format!("Failed to upload file. Status code: '{}'",response.status())
        };
        return HttpResponse::InternalServerError().json(err_message);
    }
}

#[post("/open")]
pub async fn open_file(path: web::Json<RootPath>) -> impl Responder {
    let file_path= &path.root;

    // On Windows, use the "start" command to open the file with the default program
    #[cfg(target_os="windows")]
    {
        let open_cmd=Command::new("cmd")
            .args(&["/C", "start", "", &file_path])
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
            .arg(&file_path)
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